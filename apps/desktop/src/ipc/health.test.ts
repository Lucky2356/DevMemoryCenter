import { describe, expect, it } from "vitest";

import {
  getApplicationHealth,
  maxCorrelationIdBytes,
  type InvokeTransport,
} from "./health";

describe("application health IPC", () => {
  it("sends the bounded request to the narrow command", async () => {
    const transport: InvokeTransport = async (command, arguments_) => {
      expect(command).toBe("get_application_health");
      expect(arguments_).toEqual({
        request: { correlationId: "startup-check_01" },
      });

      return {
        status: "ready",
        apiVersion: 1,
        correlationId: "startup-check_01",
      };
    };

    await expect(
      getApplicationHealth({ correlationId: "startup-check_01" }, transport),
    ).resolves.toEqual({
      ok: true,
      value: {
        status: "ready",
        apiVersion: 1,
        correlationId: "startup-check_01",
      },
    });
  });

  it.each(["", "contains space", "control\n", "кириллица"])(
    "rejects the unsafe correlation id %j before IPC",
    async (correlationId) => {
      let transportCalled = false;
      const transport: InvokeTransport = async () => {
        transportCalled = true;
        return undefined;
      };

      const result = await getApplicationHealth({ correlationId }, transport);

      expect(transportCalled).toBe(false);
      expect(result).toEqual({
        ok: false,
        error: {
          code: "invalid_request",
          messageKey: "invalid_request",
          retryable: false,
        },
      });
    },
  );

  it("rejects an oversized correlation id before IPC", async () => {
    const correlationId = "a".repeat(maxCorrelationIdBytes + 1);
    const transport: InvokeTransport = async () => {
      throw new Error("transport must not be called");
    };

    const result = await getApplicationHealth({ correlationId }, transport);

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.code).toBe("invalid_request");
    }
  });

  it("rejects mismatched or malformed backend responses", async () => {
    const transport: InvokeTransport = async () => ({
      status: "ready",
      apiVersion: 1,
      correlationId: "different-request",
    });

    await expect(
      getApplicationHealth({ correlationId: "expected-request" }, transport),
    ).resolves.toEqual({
      ok: false,
      error: {
        code: "invalid_response",
        messageKey: "invalid_response",
        retryable: false,
      },
    });
  });

  it("rejects successful responses with unexpected fields", async () => {
    const transport: InvokeTransport = async () => ({
      status: "ready",
      apiVersion: 1,
      correlationId: "expected-request",
      internalDetail: "must not cross the boundary",
    });

    const result = await getApplicationHealth(
      { correlationId: "expected-request" },
      transport,
    );

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.code).toBe("invalid_response");
    }
    expect(JSON.stringify(result)).not.toContain("internalDetail");
  });

  it("preserves only recognized serialized errors", async () => {
    const transport: InvokeTransport = async () => {
      throw {
        code: "invalid_request",
        messageKey: "invalid_request",
        retryable: false,
        internalDetail: "must not cross the boundary",
      };
    };

    const result = await getApplicationHealth(
      { correlationId: "safe-request" },
      transport,
    );

    expect(result).toEqual({
      ok: false,
      error: {
        code: "invalid_request",
        messageKey: "invalid_request",
        retryable: false,
      },
    });
    expect(JSON.stringify(result)).not.toContain("internalDetail");
  });

  it("does not retain arbitrary rejected values", async () => {
    const sensitiveMarker = "fake-sensitive-marker";
    const transport: InvokeTransport = async () => {
      throw new Error(sensitiveMarker);
    };

    const result = await getApplicationHealth(
      { correlationId: "safe-request" },
      transport,
    );

    expect(result).toEqual({
      ok: false,
      error: {
        code: "operation_failed",
        messageKey: "operation_failed",
        retryable: false,
      },
    });
    expect(JSON.stringify(result)).not.toContain(sensitiveMarker);
  });
});
