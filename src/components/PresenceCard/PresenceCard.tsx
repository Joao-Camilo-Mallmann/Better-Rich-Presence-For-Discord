import { useEffect, useState } from "react";
import { DiscordProfile, PresenceData } from "../../types";
import { getIconUrl } from "../../utils/iconUrl";

const defaultProfile: DiscordProfile = {
  displayName: "Better RPC User",
  username: "better_rpc",
  avatarUrl: "",
  customStatus: "🚀 Personalizing my presence",
  status: "online",
  themePrimary: "#5865f2",
  themeSecondary: "#2c327d",
  isGradient: true,
};

const avatarColors = ["#5865F2", "#ED4245", "#FEE75C", "#EB459E", "#57F287", "#F47B67"];

interface PresenceCardProps {
  presence: PresenceData | null;
  profile?: DiscordProfile;
}

export function PresenceCard({ presence, profile = defaultProfile }: PresenceCardProps) {
  const [elapsed, setElapsed] = useState<string>("");

  useEffect(() => {
    if (!presence?.timestamp) { setElapsed(""); return; }

    const updateTimer = () => {
      const diff = Math.max(0, Math.floor(Date.now() / 1000) - presence.timestamp!);
      const h = Math.floor(diff / 3600);
      const m = Math.floor((diff % 3600) / 60);
      const s = diff % 60;
      const pad = (n: number) => n.toString().padStart(2, "0");
      setElapsed(`${h > 0 ? `${pad(h)}:` : ""}${pad(m)}:${pad(s)} elapsed`);
    };

    updateTimer();
    const interval = setInterval(updateTimer, 1000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  const getAvatarColor = (name: string) =>
    avatarColors[name.length > 0 ? name.charCodeAt(0) % avatarColors.length : 0];

  const renderStatusIndicator = (status: "online" | "idle" | "dnd" | "offline") => {
    switch (status) {
      case "online":
        return <div className="w-full h-full rounded-full bg-green-accent" title="Online" />;
      case "idle":
        return (
          <svg className="w-full h-full text-yellow-accent" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12.1,22A10,10,0,0,1,10.76,2.07a1,1,0,0,1,1.13,1.37,8,8,0,1,0,9.67,9.67,1,1,0,0,1,1.37,1.13A10,10,0,0,1,12.1,22Z" />
          </svg>
        );
      case "dnd":
        return (
          <div className="w-full h-full rounded-full bg-danger flex items-center justify-center" title="Do Not Disturb">
            <div className="w-[10px] h-[2.5px] bg-surface-black rounded-none" />
          </div>
        );
      case "offline":
        return (
          <div className="w-full h-full rounded-full bg-muted-ink flex items-center justify-center" title="Invisible">
            <div className="w-[8px] h-[8px] bg-surface-black rounded-full" />
          </div>
        );
    }
  };

  const appName = presence?.large_text || "Better Rich Presence";
  const iconUrl = getIconUrl(appName, presence?.large_image || "");
  const appInitials = appName ? appName.charAt(0).toUpperCase() : "?";

  return (
    <div
      className="w-full max-w-[340px] overflow-hidden text-white font-body select-none flex flex-col shrink-0 neo-card p-3"
      style={{
        background: profile.isGradient
          ? `linear-gradient(180deg, ${profile.themePrimary} 0%, ${profile.themeSecondary} 100%)`
          : profile.themePrimary,
        borderRadius: '16px',
      }}
    >
      {/* Avatar Container */}
      <div className="flex justify-center">
        <div className="relative w-[70px] h-[70px] rounded-full overflow-visible"
          style={{ border: '4px solid var(--neo-border-color)', backgroundColor: 'var(--surface-black)' }}>
          {profile.avatarUrl ? (
            <img
              src={profile.avatarUrl}
              alt={profile.username}
              className="w-full h-full rounded-full object-cover"
              onError={(e) => {
                (e.target as HTMLElement).style.display = "none";
                const parent = (e.target as HTMLElement).parentElement;
                if (parent) {
                  const fallback = document.createElement("div");
                  fallback.className = "w-full h-full rounded-full flex items-center justify-center text-xl font-bold text-white";
                  fallback.style.backgroundColor = getAvatarColor(profile.username);
                  fallback.innerText = profile.username.charAt(0).toUpperCase();
                  parent.appendChild(fallback);
                }
              }}
            />
          ) : (
            <div
              className="w-full h-full rounded-full flex items-center justify-center text-xl font-extrabold text-white"
              style={{ backgroundColor: getAvatarColor(profile.username) }}
            >
              {profile.username.charAt(0).toUpperCase()}
            </div>
          )}
          {/* Status indicator badge */}
          <div className="absolute bottom-0 right-0 w-[20px] h-[20px] rounded-full flex items-center justify-center overflow-hidden"
            style={{ border: '3px solid var(--neo-border-color)', backgroundColor: 'var(--surface-black)' }}>
            {renderStatusIndicator(profile.status)}
          </div>
        </div>
      </div>
      {/* Profile Body (Card) */}
      <div className="mt-3 p-3 flex flex-col gap-3 z-0 relative neo-border-2"
        style={{ backgroundColor: 'rgba(0,0,0,0.85)', borderRadius: '8px' }}>
        {/* User Identity */}
        <div className="flex flex-col items-center text-center">
          <span className="font-extrabold text-[15px] leading-tight text-white font-display">
            @{profile.username}
          </span>
          {profile.customStatus && (
            <div className="text-[12px] text-zinc-200 mt-2 px-2 py-1.5 italic text-left neo-border-2"
              style={{ backgroundColor: 'rgba(255,255,255,0.05)', borderRadius: '4px' }}>
              {profile.customStatus}
            </div>
          )}
        </div>
        <div className="h-[3px] bg-[var(--neo-border-color)] my-0.5" />
        {/* Presence Section */}
        <div className="flex flex-col">
          <div className="flex justify-between items-center text-[10px] font-extrabold text-zinc-400 tracking-wider uppercase mb-2 font-display">
            <span className="neo-stroke" style={{ WebkitTextStroke: '0.5px rgba(0,0,0,0.3)' }}>PLAYING A GAME</span>
          </div>
          {/* Activity Layout */}
          <div className="flex items-start gap-3 mt-1 text-left">
            <div className="relative w-[60px] h-[60px] flex-shrink-0 flex items-center justify-center overflow-visible neo-border-2 neo-shadow-sm"
              style={{ backgroundColor: 'rgba(255,255,255,0.05)', borderRadius: '8px' }}>
              {presence ? (
                <img
                  src={iconUrl}
                  alt={appName}
                  className="w-full h-full object-contain p-1"
                  style={{ borderRadius: '6px' }}
                  onError={(e) => {
                    (e.target as HTMLElement).style.display = "none";
                    const parent = (e.target as HTMLElement).parentElement;
                    if (parent) {
                      const fallback = document.createElement("div");
                      fallback.className = "text-xl font-bold text-zinc-400";
                      fallback.innerText = appInitials;
                      parent.appendChild(fallback);
                    }
                  }}
                />
              ) : (
                <div className="text-xl font-bold text-zinc-400">?</div>
              )}
            </div>
            <div className="flex flex-col min-w-0 flex-1 justify-center py-0.5 select-text">
              <div className="font-bold text-[13px] text-white leading-tight truncate font-display">
                {presence?.large_text || "Better Rich Presence"}
              </div>
              <div className="text-[12px] text-zinc-300 leading-tight truncate mt-1">
                {presence?.details || "No application running"}
              </div>
              <div className="text-[12px] leading-tight truncate mt-1 text-zinc-300/80">
                {presence?.state || "Waiting for detection..."}
              </div>
              {elapsed && (
                <div className="text-[11.5px] text-zinc-400 leading-tight truncate mt-1 font-bold font-display">
                  {elapsed}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
