import { GitBranch, Pin } from "lucide-react";
import { useClaudeCode } from "../hooks/useClaudeCode";

/** Live list of active Claude Code chats in the Now view, with click-to-pin support. */
export function ClaudeCodeChats() {
  const { status, selectSession } = useClaudeCode();

  if (!status || status.sessions.length === 0) {
    return null;
  }

  const isPinnedAny = Boolean(status.pinned_session_id);

  return (
    <div
      className="w-full max-w-[340px] bg-surface-indigo neo-border-2 neo-shadow-sm flex flex-col overflow-hidden"
      style={{ borderRadius: "8px" }}
    >
      <div className="flex items-center justify-between gap-2 px-3 py-2 border-b-2 border-[var(--neo-border-color)]">
        <div className="flex items-center gap-2 min-w-0">
          <img
            src="/assets/icons/claude.png"
            alt=""
            className="w-4 h-4 object-contain shrink-0"
          />
          <span className="text-[11px] font-extrabold font-display uppercase tracking-wider text-ink truncate">
            Claude Code chats
          </span>
          <span
            className="text-[10px] font-extrabold px-1.5 py-0.5 bg-canvas text-ink neo-border-2"
            style={{ borderRadius: "4px" }}
          >
            {status.sessions.length}
          </span>
        </div>

        {isPinnedAny && (
          <button
            type="button"
            onClick={() => selectSession(null)}
            className="shrink-0 text-[11px] font-bold px-2 py-0.5 bg-canvas text-ink neo-border-2 neo-press"
            style={{ borderRadius: "4px" }}
          >
            Auto-follow
          </button>
        )}
      </div>

      <div className="flex flex-col divide-y-2 divide-[var(--neo-border-color)]">
        {status.sessions.map((session) => {
          const isPinned = status.pinned_session_id === session.session_id;
          const isSelected = status.selected_session_id === session.session_id;
          const title =
            session.chat_title || session.project_name || "Active chat";
          const showProject =
            Boolean(session.project_name) &&
            session.project_name !== title;
          const dotClass = session.is_waiting
            ? "bg-yellow-accent"
            : "bg-green-accent";

          return (
            <button
              key={session.session_id}
              type="button"
              onClick={() =>
                selectSession(isPinned ? null : session.session_id)
              }
              aria-pressed={isPinned}
              className={`w-full text-left px-3 py-2.5 flex flex-col gap-1 transition-colors hover:bg-white/5 ${
                isSelected ? "bg-white/5" : ""
              }`}
            >
              <div className="flex items-center justify-between gap-2">
                <div className="flex items-center gap-2 min-w-0">
                  <span
                    aria-hidden
                    className={`w-2 h-2 rounded-full shrink-0 border border-black ${dotClass}`}
                  />
                  <span className="text-[12.5px] font-bold text-ink truncate">
                    {title}
                  </span>
                </div>
                {isPinned ? (
                  <span
                    className="shrink-0 inline-flex items-center gap-1 text-[10px] font-extrabold uppercase px-1.5 py-0.5 bg-primary text-white neo-border-2"
                    style={{ borderRadius: "4px" }}
                  >
                    <Pin size={10} strokeWidth={3} aria-hidden />
                    Pinned
                  </span>
                ) : (
                  isSelected && (
                    <span
                      className="shrink-0 text-[10px] font-extrabold uppercase px-1.5 py-0.5 bg-canvas text-muted-ink neo-border-2"
                      style={{ borderRadius: "4px" }}
                    >
                      Showing
                    </span>
                  )
                )}
              </div>

              <div className="flex flex-wrap items-center gap-x-2.5 gap-y-0.5 text-[11px] text-muted-ink pl-4">
                {showProject && (
                  <span className="truncate max-w-[110px]">
                    {session.project_name}
                  </span>
                )}
                {session.git_branch && (
                  <span className="inline-flex items-center gap-1 truncate max-w-[110px]">
                    <GitBranch size={11} className="shrink-0" aria-hidden />
                    <span className="truncate">{session.git_branch}</span>
                  </span>
                )}
                {session.model && (
                  <span className="truncate text-ink/80">{session.model}</span>
                )}
              </div>

              <div className="text-[11.5px] text-ink/90 truncate pl-4">
                {session.activity}
              </div>
            </button>
          );
        })}
      </div>
    </div>
  );
}
