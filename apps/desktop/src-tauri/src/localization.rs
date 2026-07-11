#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Locale {
    English,
    Russian,
}

fn try_resolve_locale(language: &str) -> Option<Locale> {
    language.split(':').find_map(|candidate| {
        let normalized = candidate.trim().to_ascii_lowercase();
        let mut parts = normalized.split(['-', '_', '.', '@']);
        let language_code = parts.next()?;

        match language_code {
            "en" => Some(Locale::English),
            "ru" => Some(Locale::Russian),
            _ => None,
        }
    })
}

pub fn resolve_locale<I, S>(languages: I) -> Locale
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    languages
        .into_iter()
        .find_map(|language| try_resolve_locale(language.as_ref()))
        .unwrap_or(Locale::English)
}

pub fn system_locale() -> Locale {
    const LOCALE_VARIABLES: [&str; 4] = ["LC_ALL", "LC_MESSAGES", "LANGUAGE", "LANG"];

    resolve_locale(
        LOCALE_VARIABLES
            .into_iter()
            .filter_map(|variable| std::env::var(variable).ok()),
    )
}

pub fn startup_failure_message(locale: Locale) -> &'static str {
    match locale {
        Locale::English => include_str!("../locales/en/startup-error.txt").trim_end(),
        Locale::Russian => include_str!("../locales/ru/startup-error.txt").trim_end(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Locale, resolve_locale, startup_failure_message};

    #[test]
    fn resolves_supported_locale_variants() {
        assert_eq!(resolve_locale(["ru_RU.UTF-8"]), Locale::Russian);
        assert_eq!(resolve_locale(["en-US"]), Locale::English);
    }

    #[test]
    fn selects_first_supported_locale() {
        assert_eq!(resolve_locale(["de-DE", "ru-RU", "en-US"]), Locale::Russian);
        assert_eq!(resolve_locale(["de-DE:ru-RU:en-US"]), Locale::Russian);
    }

    #[test]
    fn falls_back_to_english() {
        assert_eq!(resolve_locale(["de-DE"]), Locale::English);
        assert_eq!(resolve_locale::<[&str; 0], &str>([]), Locale::English);
    }

    #[test]
    fn provides_localized_startup_failures() {
        assert_eq!(
            startup_failure_message(Locale::English),
            "Dev Recall could not start."
        );
        assert_eq!(
            startup_failure_message(Locale::Russian),
            "Не удалось запустить Dev Recall."
        );
    }
}
