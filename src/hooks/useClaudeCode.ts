import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { ClaudeCodeStatus } from "../types";

function formatError(err: unknown): string {
  if (typeof err === "string") return err;
  if (err && typeof err === "object" && "message" in err) {
    const msg = (err as { message?: unknown }).message;
    if (typeof msg === "string") return msg;
  }
  return "Failed to update Claude Code hooks.";
}

/** Fetches and subscribes to live Claude Code hook and session status. */
export function useClaudeCode() {
  const [status, setStatus] = useState<ClaudeCodeStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    let unlisten: UnlistenFn | null = null;

    async function init() {
      try {
        const initial = await invoke<ClaudeCodeStatus>("get_claude_code_status");
        if (active) setStatus(initial);

        const stop = await listen<ClaudeCodeStatus>(
          "claude-code-status-changed",
          (event) => {
            setStatus(event.payload);
          },
        );
        if (active) {
          unlisten = stop;
        } else {
          stop();
        }
      } catch (err) {
        console.error("Failed to load Claude Code status:", err);
      }
    }

    init();
    return () => {
      active = false;
      if (unlisten) unlisten();
    };
  }, []);

  const connect = async () => {
    setBusy(true);
    setError(null);
    try {
      const next = await invoke<ClaudeCodeStatus>("connect_claude_code");
      setStatus(next);
    } catch (err) {
      setError(formatError(err));
    }
    setBusy(false);
  };

  const disconnect = async () => {
    setBusy(true);
    setError(null);
    try {
      const next = await invoke<ClaudeCodeStatus>("disconnect_claude_code");
      setStatus(next);
    } catch (err) {
      setError(formatError(err));
    }
    setBusy(false);
  };

  const selectSession = async (sessionId: string | null) => {
    setError(null);
    try {
      await invoke("select_claude_code_session", { sessionId });
    } catch (err) {
      setError(formatError(err));
    }
  };

  return { status, busy, error, connect, disconnect, selectSession };
}
