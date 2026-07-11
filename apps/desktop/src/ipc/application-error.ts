export const applicationErrorCodes = [
  "invalid_request",
  "invalid_response",
  "operation_failed",
] as const;

export type ApplicationErrorCode = (typeof applicationErrorCodes)[number];

export const applicationErrorMessageKeys = applicationErrorCodes;

export type ApplicationErrorMessageKey =
  (typeof applicationErrorMessageKeys)[number];

export interface ApplicationError {
  readonly code: ApplicationErrorCode;
  readonly messageKey: ApplicationErrorMessageKey;
  readonly retryable: boolean;
}

const fallbackError: ApplicationError = {
  code: "operation_failed",
  messageKey: "operation_failed",
  retryable: false,
};

function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isApplicationErrorCode(value: unknown): value is ApplicationErrorCode {
  return (
    typeof value === "string" &&
    applicationErrorCodes.some((candidate) => candidate === value)
  );
}

export function sanitizeApplicationError(value: unknown): ApplicationError {
  if (
    isRecord(value) &&
    isApplicationErrorCode(value.code) &&
    value.messageKey === value.code &&
    typeof value.retryable === "boolean"
  ) {
    return {
      code: value.code,
      messageKey: value.code,
      retryable: value.retryable,
    };
  }

  return { ...fallbackError };
}

export function invalidRequestError(): ApplicationError {
  return {
    code: "invalid_request",
    messageKey: "invalid_request",
    retryable: false,
  };
}

export function invalidResponseError(): ApplicationError {
  return {
    code: "invalid_response",
    messageKey: "invalid_response",
    retryable: false,
  };
}
