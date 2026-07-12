#![deny(unsafe_code)]

use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    sync::{
        Arc, Condvar, Mutex, MutexGuard, Weak,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

pub const DEFAULT_MAX_BACKGROUND_OPERATIONS: usize = 4;
pub const MAX_BACKGROUND_OPERATIONS: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct OperationId(u64);

impl OperationId {
    pub fn new(value: u64) -> Result<Self, LifecycleError> {
        if value == 0 {
            return Err(LifecycleError::InvalidOperationId);
        }

        Ok(Self(value))
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleError {
    InvalidConcurrencyLimit,
    InvalidOperationId,
    ShuttingDown,
    CapacityReached,
    DuplicateOperationId,
}

impl fmt::Display for LifecycleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidConcurrencyLimit => "background operation limit is invalid",
            Self::InvalidOperationId => "background operation identifier is invalid",
            Self::ShuttingDown => "application shutdown is in progress",
            Self::CapacityReached => "background operation capacity is reached",
            Self::DuplicateOperationId => "background operation identifier is already active",
        };

        formatter.write_str(message)
    }
}

impl Error for LifecycleError {}

#[derive(Clone, Debug)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShutdownStatus {
    Complete,
    TimedOut { remaining_operations: usize },
}

struct ActiveOperation {
    cancellation: CancellationToken,
}

struct LifecycleState {
    accepting_operations: bool,
    active_operations: BTreeMap<OperationId, ActiveOperation>,
}

struct LifecycleInner {
    state: Mutex<LifecycleState>,
    operation_finished: Condvar,
}

pub struct BackgroundOperationPermit {
    operation_id: OperationId,
    cancellation: CancellationToken,
    owner: Weak<LifecycleInner>,
}

impl BackgroundOperationPermit {
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }
}

impl Drop for BackgroundOperationPermit {
    fn drop(&mut self) {
        let Some(owner) = self.owner.upgrade() else {
            return;
        };

        let mut state = lock_state(&owner);
        state.active_operations.remove(&self.operation_id);
        owner.operation_finished.notify_all();
    }
}

pub struct BackgroundOperationManager {
    max_active_operations: usize,
    inner: Arc<LifecycleInner>,
}

impl BackgroundOperationManager {
    pub fn new(max_active_operations: usize) -> Result<Self, LifecycleError> {
        if !(1..=MAX_BACKGROUND_OPERATIONS).contains(&max_active_operations) {
            return Err(LifecycleError::InvalidConcurrencyLimit);
        }

        Ok(Self {
            max_active_operations,
            inner: Arc::new(LifecycleInner {
                state: Mutex::new(LifecycleState {
                    accepting_operations: true,
                    active_operations: BTreeMap::new(),
                }),
                operation_finished: Condvar::new(),
            }),
        })
    }

    pub fn start(
        &self,
        operation_id: OperationId,
    ) -> Result<BackgroundOperationPermit, LifecycleError> {
        let mut state = lock_state(&self.inner);

        if !state.accepting_operations {
            return Err(LifecycleError::ShuttingDown);
        }
        if state.active_operations.contains_key(&operation_id) {
            return Err(LifecycleError::DuplicateOperationId);
        }
        if state.active_operations.len() >= self.max_active_operations {
            return Err(LifecycleError::CapacityReached);
        }

        let cancellation = CancellationToken {
            cancelled: Arc::new(AtomicBool::new(false)),
        };
        state.active_operations.insert(
            operation_id,
            ActiveOperation {
                cancellation: cancellation.clone(),
            },
        );

        Ok(BackgroundOperationPermit {
            operation_id,
            cancellation,
            owner: Arc::downgrade(&self.inner),
        })
    }

    pub fn cancel(&self, operation_id: OperationId) -> bool {
        let state = lock_state(&self.inner);
        let Some(operation) = state.active_operations.get(&operation_id) else {
            return false;
        };

        operation.cancellation.cancel();
        true
    }

    pub fn active_operation_count(&self) -> usize {
        lock_state(&self.inner).active_operations.len()
    }

    pub fn shutdown(&self, timeout: Duration) -> ShutdownStatus {
        let mut state = lock_state(&self.inner);
        state.accepting_operations = false;
        cancel_all(&state);

        if state.active_operations.is_empty() {
            return ShutdownStatus::Complete;
        }

        let wait_result =
            self.inner
                .operation_finished
                .wait_timeout_while(state, timeout, |current| {
                    !current.active_operations.is_empty()
                });
        state = match wait_result {
            Ok((current, _)) => current,
            Err(poisoned) => poisoned.into_inner().0,
        };

        if state.active_operations.is_empty() {
            ShutdownStatus::Complete
        } else {
            ShutdownStatus::TimedOut {
                remaining_operations: state.active_operations.len(),
            }
        }
    }
}

impl Drop for BackgroundOperationManager {
    fn drop(&mut self) {
        let mut state = lock_state(&self.inner);
        state.accepting_operations = false;
        cancel_all(&state);
    }
}

fn lock_state(inner: &LifecycleInner) -> MutexGuard<'_, LifecycleState> {
    match inner.state.lock() {
        Ok(state) => state,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn cancel_all(state: &LifecycleState) {
    for operation in state.active_operations.values() {
        operation.cancellation.cancel();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc,
        thread,
        time::{Duration, Instant},
    };

    use super::{
        BackgroundOperationManager, LifecycleError, MAX_BACKGROUND_OPERATIONS, OperationId,
        ShutdownStatus,
    };

    fn operation_id(value: u64) -> OperationId {
        match OperationId::new(value) {
            Ok(operation_id) => operation_id,
            Err(error) => panic!("test operation identifier must be valid: {error}"),
        }
    }

    fn manager(limit: usize) -> BackgroundOperationManager {
        match BackgroundOperationManager::new(limit) {
            Ok(manager) => manager,
            Err(error) => panic!("test manager limit must be valid: {error}"),
        }
    }

    #[test]
    fn rejects_invalid_limits_and_bounds_active_operations() {
        assert!(matches!(
            BackgroundOperationManager::new(0),
            Err(LifecycleError::InvalidConcurrencyLimit)
        ));
        assert!(matches!(
            BackgroundOperationManager::new(MAX_BACKGROUND_OPERATIONS + 1),
            Err(LifecycleError::InvalidConcurrencyLimit)
        ));
        assert!(matches!(
            OperationId::new(0),
            Err(LifecycleError::InvalidOperationId)
        ));

        let manager = manager(1);
        let first = manager.start(operation_id(1));
        assert!(first.is_ok());
        assert!(matches!(
            manager.start(operation_id(1)),
            Err(LifecycleError::DuplicateOperationId)
        ));
        assert!(matches!(
            manager.start(operation_id(2)),
            Err(LifecycleError::CapacityReached)
        ));
        drop(first);
        assert_eq!(manager.active_operation_count(), 0);
    }

    #[test]
    fn explicit_cancellation_is_scoped_to_the_requested_operation() {
        let manager = manager(2);
        let first = manager.start(operation_id(1));
        let second = manager.start(operation_id(2));
        let (first, second) = match (first, second) {
            (Ok(first), Ok(second)) => (first, second),
            _ => panic!("test operations must start"),
        };

        assert!(manager.cancel(operation_id(1)));
        assert!(first.cancellation_token().is_cancelled());
        assert!(!second.cancellation_token().is_cancelled());
        assert!(!manager.cancel(operation_id(3)));
    }

    #[test]
    fn shutdown_cancels_owned_work_and_waits_for_cleanup() {
        let manager = manager(1);
        let permit = match manager.start(operation_id(7)) {
            Ok(permit) => permit,
            Err(error) => panic!("test operation must start: {error}"),
        };
        let cancellation = permit.cancellation_token();
        let (cancelled_sender, cancelled_receiver) = mpsc::sync_channel(1);
        let worker = thread::spawn(move || {
            while !cancellation.is_cancelled() {
                thread::park_timeout(Duration::from_millis(1));
            }
            let _ = cancelled_sender.send(());
            drop(permit);
        });

        assert_eq!(
            manager.shutdown(Duration::from_secs(1)),
            ShutdownStatus::Complete
        );
        assert!(
            cancelled_receiver
                .recv_timeout(Duration::from_secs(1))
                .is_ok()
        );
        assert!(worker.join().is_ok());
        assert_eq!(manager.active_operation_count(), 0);
        assert!(matches!(
            manager.start(operation_id(8)),
            Err(LifecycleError::ShuttingDown)
        ));
    }

    #[test]
    fn shutdown_timeout_is_bounded_and_can_be_retried_after_cleanup() {
        let manager = manager(1);
        let permit = match manager.start(operation_id(9)) {
            Ok(permit) => permit,
            Err(error) => panic!("test operation must start: {error}"),
        };
        let cancellation = permit.cancellation_token();
        let started = Instant::now();

        assert_eq!(
            manager.shutdown(Duration::from_millis(10)),
            ShutdownStatus::TimedOut {
                remaining_operations: 1
            }
        );
        assert!(started.elapsed() < Duration::from_secs(1));
        assert!(cancellation.is_cancelled());

        drop(permit);
        assert_eq!(
            manager.shutdown(Duration::from_millis(10)),
            ShutdownStatus::Complete
        );
    }

    #[test]
    fn dropping_manager_signals_active_operation_cancellation() {
        let manager = manager(1);
        let permit = match manager.start(operation_id(11)) {
            Ok(permit) => permit,
            Err(error) => panic!("test operation must start: {error}"),
        };
        let cancellation = permit.cancellation_token();

        drop(manager);

        assert!(cancellation.is_cancelled());
        drop(permit);
    }
}
