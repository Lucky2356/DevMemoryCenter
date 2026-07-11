import { invoke } from "@tauri-apps/api/core";

import {
  invalidRequestError,
  invalidResponseError,
  sanitizeApplicationError,
  type ApplicationError,
} from "./application-error";

const healthCommand = "get_application_health";
const healthApiVersion = 1;
export const maxCorrelationIdBytes = 64;
const correlationIdPattern = /^[A-Za-z0-9._-]+$/u;

export interface HealthRequest {
  readonly correlationId: string;
}

export interface HealthResponse {
  readonly status: "ready";
  readonly apiVersion: 1;
  readonly correlationId: string;
}

export type IpcResult<T> =
  | { readonly ok: true; readonly value: T }
  | { readonly ok: false; readonly error: ApplicationError };

export type InvokeTransport = (
  command: string,
  arguments_: Readonly<Record<string, unknown>>,
) => Promise<unknown>;

const tauriTransport: InvokeTransport = (command, arguments_) =>
  invoke<unknown>(command, arguments_);

function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isValidCorrelationId(value: unknown): value is string {
  return (
    typeof value === "string" &&
    value.length > 0 &&
    value.length <= maxCorrelationIdBytes &&
    new TextEncoder().encode(value).byteLength <= maxCorrelationIdBytes &&
    correlationIdPattern.test(value)
  );
}

function isHealthResponse(
  value: unknown,
  correlationId: string,
): value is HealthResponse {
  return (
    isRecord(value) &&
    Object.keys(value).length === 3 &&
    value.status === "ready" &&
    value.apiVersion === healthApiVersion &&
    value.correlationId === correlationId
  );
}

export async function getApplicationHealth(
  request: HealthRequest,
  transport: InvokeTransport = tauriTransport,
): Promise<IpcResult<HealthResponse>> {
  if (!isValidCorrelationId(request.correlationId)) {
    return { ok: false, error: invalidRequestError() };
  }

  try {
    const boundedRequest: HealthRequest = {
      correlationId: request.correlationId,
    };
    const response = await transport(healthCommand, {
      request: boundedRequest,
    });

    return isHealthResponse(response, request.correlationId)
      ? { ok: true, value: response }
      : { ok: false, error: invalidResponseError() };
  } catch (error: unknown) {
    return { ok: false, error: sanitizeApplicationError(error) };
  }
}
