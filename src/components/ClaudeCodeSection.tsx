import { TriangleAlert } from "lucide-react";
import { useClaudeCode } from "../hooks/useClaudeCode";

/** Connection controls and live status for the Claude Code HTTP hook integration. */
export function ClaudeCodeSection() {
  const { status, busy, error, connect, disconnect } = useClaudeCode();

  if (!status) {
    return <p className="py-3 text-[12px] text-muted-ink">Checking Claude Code…</p>;
  }

  const isInstalled = status.install_state === "installed";
  const isOutdated = status.install_state === "outdated";
  const hasActiveSession = status.active_sessions > 0;

  let dotClass = "bg-muted-ink";
  let summary = "Not connected · Add HTTP hooks to Claude Code";
  if (hasActiveSession && status.active_activity) {
    dotClass = "bg-green-accent";
    summary = `Live · ${status.active_activity}`;
  } else if (isInstalled && status.server_listening) {
    dotClass = "bg-green-accent";
    summary = "Connected · Ready for Claude Code sessions";
  } else if (isInstalled && !status.server_listening) {
    dotClass = "bg-yellow-accent";
    summary = "Hooks installed · Local listener unavailable";
  } else if (isOutdated) {
    dotClass = "bg-yellow-accent";
    summary = "Hooks need update · Reconnect to sync";
  }

  let buttonLabel = "Connect";
  if (busy) {
    buttonLabel = isInstalled ? "Removing…" : "Connecting…";
  } else if (isInstalled) {
    buttonLabel = "Disconnect";
  } else if (isOutdated) {
    buttonLabel = "Reconnect";
  }

  const buttonClass = isInstalled
    ? "shrink-0 neo-btn bg-canvas text-ink text-[12px] px-3 py-1.5 disabled:opacity-50 disabled:cursor-not-allowed"
    : "shrink-0 neo-btn bg-primary text-white text-[12px] px-3 py-1.5 disabled:opacity-50 disabled:cursor-not-allowed";

  const alertMessage = error || status.server_error;

  return (
    <div className="flex flex-col">
      <div className="flex justify-between items-center gap-3 py-3">
        <div className="flex items-center gap-3 min-w-0">
          <div
            className="w-8 h-8 shrink-0 flex items-center justify-center bg-canvas neo-border-2"
            style={{ borderRadius: "6px" }}
          >
            <img
              src="/assets/icons/claude.png"
              alt=""
              className="w-5 h-5 object-contain"
            />
          </div>
          <div className="flex flex-col gap-0.5 min-w-0">
            <span className="text-[13px] font-semibold text-ink">
              Claude Code hooks
            </span>
            <span className="flex items-center gap-1.5 text-[12px] leading-snug text-muted-ink">
              <span
                aria-hidden
                className={`w-2 h-2 rounded-full shrink-0 border border-black ${dotClass}`}
              />
              <span className="truncate select-text">{summary}</span>
            </span>
          </div>
        </div>

        <button
          type="button"
          className={buttonClass}
          style={{ borderRadius: "6px" }}
          onClick={isInstalled ? disconnect : connect}
          disabled={busy}
        >
          {buttonLabel}
        </button>
      </div>

      {alertMessage && (
        <div className="pb-3">
          <div
            role="alert"
            className="bg-yellow-accent text-ink-dark p-2.5 flex gap-2 items-start text-[12px] leading-snug neo-border-2"
            style={{ borderRadius: "6px" }}
          >
            <TriangleAlert size={14} className="shrink-0 mt-0.5" aria-hidden />
            <span className="select-text break-words min-w-0">{alertMessage}</span>
          </div>
        </div>
      )}

      <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1.5 py-3 text-[12px] border-t-2 border-[var(--neo-border-color)]">
        <div className="contents">
          <dt className="text-muted-ink">Config</dt>
          <dd className="text-ink select-text truncate" title={status.settings_path}>
            {status.settings_path || "~/.claude/settings.json"}
          </dd>
        </div>
        <div className="contents">
          <dt className="text-muted-ink">Listener</dt>
          <dd className="text-ink select-text truncate" title={status.endpoint_url}>
            {status.server_listening ? status.endpoint_url : "Unavailable"}
          </dd>
        </div>
      </dl>
    </div>
  );
}
