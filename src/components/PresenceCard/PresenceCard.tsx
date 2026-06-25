import { useEffect, useState } from "react";
import { DiscordProfile, PresenceData } from "../../types";

// Default Profile when none is provided
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

interface PresenceCardProps {
  presence: PresenceData | null;
  profile?: DiscordProfile;
}

function getIconUrl(appName: string, largeImage: string): string {
  if (
    largeImage &&
    (largeImage.startsWith("http://") || largeImage.startsWith("https://"))
  ) {
    return largeImage;
  }

  const nameToSearch = (appName || "").toLowerCase();
  if (!nameToSearch) return "";

  const lookup: Record<string, string> = {
    vscode: "visualstudio.com",
    "visual studio code": "visualstudio.com",
    code: "visualstudio.com",
    cursor: "cursor.com",
    intellij: "jetbrains.com",
    "android studio": "developer.android.com",
    figma: "figma.com",
    photoshop: "adobe.com",
    premiere: "adobe.com",
    "after effects": "adobe.com",
    blender: "blender.org",
    excel: "microsoft.com",
    word: "microsoft.com",
    powerpoint: "microsoft.com",
    notion: "notion.so",
    obsidian: "obsidian.md",
    slack: "slack.com",
    spotify: "spotify.com",
    chrome: "google.com",
    firefox: "mozilla.org",
    edge: "microsoft.com",
    discord: "discord.com",
    github: "github.com",
    docker: "docker.com",
    steam: "steampowered.com",
    cs2: "counter-strike.net",
    "counter-strike": "counter-strike.net",
    valorant: "playvalorant.com",
    minecraft: "minecraft.net",
  };

  let domain = "";
  if (largeImage && largeImage !== "auto") {
    const key = largeImage.toLowerCase();
    if (lookup[key]) {
      domain = lookup[key];
    } else {
      const formattedKey = key.replace(/[^a-z0-9]/g, "");
      for (const k of Object.keys(lookup)) {
        if (
          k.replace(/[^a-z0-9]/g, "").includes(formattedKey) ||
          formattedKey.includes(k.replace(/[^a-z0-9]/g, ""))
        ) {
          domain = lookup[k];
          break;
        }
      }
    }
  }

  if (!domain) {
    for (const key of Object.keys(lookup)) {
      if (nameToSearch.includes(key)) {
        domain = lookup[key];
        break;
      }
    }
  }

  if (!domain) {
    const clean = nameToSearch.replace(/[^a-zA-Z0-9\s-_]/g, "");
    const firstWord = clean.trim().split(/\s+/)[0];
    domain = firstWord
      ? `${firstWord}.com`
      : `${clean.replace(/\s+/g, "")}.com`;
  }

  return `https://www.google.com/s2/favicons?sz=128&domain=${domain}`;
}

export function PresenceCard({
  presence,
  profile = defaultProfile,
}: PresenceCardProps) {
  const [elapsed, setElapsed] = useState<string>("");

  useEffect(() => {
    if (!presence?.timestamp) {
      setElapsed("");
      return;
    }

    const updateTimer = () => {
      const now = Math.floor(Date.now() / 1000);
      const diff = Math.max(0, now - presence.timestamp!);

      const hours = Math.floor(diff / 3600);
      const minutes = Math.floor((diff % 3600) / 60);
      const seconds = diff % 60;

      if (hours > 0) {
        setElapsed(
          `${hours.toString().padStart(2, "0")}:${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")} elapsed`,
        );
      } else {
        setElapsed(
          `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")} elapsed`,
        );
      }
    };

    updateTimer();
    const interval = setInterval(updateTimer, 1000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  const getSourceIcon = (source: string) => {
    switch (source) {
      case "Game":
        return "🎮";
      case "Work":
        return "💼";
      case "Browser":
        return "🌐";
      case "Idle":
        return "💤";
      case "Manual":
        return "✏️";
      default:
        return "🖥️";
    }
  };

  // Auto-generate a color based on the first letter for the placeholder
  const getAvatarColor = (name: string) => {
    const colors = [
      "#5865F2",
      "#ED4245",
      "#FEE75C",
      "#EB459E",
      "#57F287",
      "#F47B67",
    ];
    const index = name.length > 0 ? name.charCodeAt(0) % colors.length : 0;
    return colors[index];
  };

  const renderStatusIndicator = (
    status: "online" | "idle" | "dnd" | "offline",
  ) => {
    switch (status) {
      case "online":
        return (
          <div
            className="w-full h-full rounded-full bg-[#23a55a]"
            title="Online"
          />
        );
      case "idle":
        return (
          <svg
            className="w-full h-full text-[#f0b232]"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path d="M12.1,22A10,10,0,0,1,10.76,2.07a1,1,0,0,1,1.13,1.37,8,8,0,1,0,9.67,9.67,1,1,0,0,1,1.37,1.13A10,10,0,0,1,12.1,22Z" />
          </svg>
        );
      case "dnd":
        return (
          <div
            className="w-full h-full rounded-full bg-[#f23f43] flex items-center justify-center"
            title="Do Not Disturb"
          >
            <div className="w-[10px] h-[2.5px] bg-[#111214] rounded-full" />
          </div>
        );
      case "offline":
        return (
          <div
            className="w-full h-full rounded-full bg-[#80848e] flex items-center justify-center"
            title="Invisible"
          >
            <div className="w-[8px] h-[8px] bg-[#111214] rounded-full" />
          </div>
        );
    }
  };

  const appName = presence?.large_text || "Better Rich Presence";
  const iconUrl = getIconUrl(appName, presence?.large_image || "");
  const appInitials = appName ? appName.charAt(0).toUpperCase() : "?";

  return (
    <div
      className="w-full max-w-[340px] rounded-[16px] overflow-hidden text-white font-body shadow-xl relative select-none border border-white/5 flex flex-col shrink-0"
      style={{
        background: profile.isGradient
          ? `linear-gradient(180deg, ${profile.themePrimary} 0%, ${profile.themeSecondary} 100%)`
          : profile.themePrimary,
      }}
    >
      {/* Profile Banner */}
      <div
        className="w-full h-[75px] transition-all duration-300 flex-shrink-0"
        style={{ backgroundColor: profile.themePrimary }}
      />

      {/* Avatar Container overlay */}
      <div className="absolute top-[38px] left-[12px] z-10">
        <div className="relative w-[70px] h-[70px] rounded-full ring-[5px] ring-[#111214] bg-[#111214] overflow-visible">
          {profile.avatarUrl ? (
            <img
              src={profile.avatarUrl}
              alt={profile.displayName}
              className="w-full h-full rounded-full object-cover"
              onError={(e) => {
                // If avatar URL fails, fall back to initial
                (e.target as HTMLElement).style.display = "none";
                const parent = (e.target as HTMLElement).parentElement;
                if (parent) {
                  const fallback = document.createElement("div");
                  fallback.className =
                    "w-full h-full rounded-full flex items-center justify-center text-xl font-bold text-white";
                  fallback.style.backgroundColor = getAvatarColor(
                    profile.displayName,
                  );
                  fallback.innerText = profile.displayName
                    .charAt(0)
                    .toUpperCase();
                  parent.appendChild(fallback);
                }
              }}
            />
          ) : (
            <div
              className="w-full h-full rounded-full flex items-center justify-center text-xl font-bold text-white"
              style={{ backgroundColor: getAvatarColor(profile.displayName) }}
            >
              {profile.displayName.charAt(0).toUpperCase()}
            </div>
          )}

          {/* Status indicator badge */}
          <div className="absolute bottom-0 right-0 w-[20px] h-[20px] rounded-full ring-[3px] ring-[#111214] bg-[#111214] flex items-center justify-center overflow-hidden">
            {renderStatusIndicator(profile.status)}
          </div>
        </div>
      </div>

      {/* Profile Body (Card) */}
      <div className="mt-[40px] mx-[12px] mb-[12px] bg-[#111214]/90 backdrop-blur-xs rounded-[8px] p-3 flex flex-col gap-3 z-0 relative border border-white/5">
        {/* User Identity */}
        <div className="flex flex-col">
          <span className="font-extrabold text-[15px] leading-tight text-white">
            {profile.displayName}
          </span>
          <span className="text-[11.5px] text-zinc-400 font-medium leading-none mt-[2px]">
            @{profile.username}
          </span>
          {profile.customStatus && (
            <div className="text-[12px] text-zinc-200 mt-2 bg-white/5 px-2 py-1.5 rounded-sm border border-white/5 italic text-left">
              {profile.customStatus}
            </div>
          )}
        </div>

        <div className="h-[1px] bg-zinc-800/80 my-0.5" />

        {/* Presence Section */}
        <div className="flex flex-col">
          {/* Header */}
          <div className="flex justify-between items-center text-[10px] font-extrabold text-zinc-400 tracking-wider uppercase mb-2">
            <span>PLAYING A GAME</span>
            <span className="text-[9px] bg-white/10 px-1.5 py-[1px] rounded-xs text-zinc-300 font-bold uppercase tracking-wide flex items-center gap-1">
              {getSourceIcon(presence?.source || "Idle")}{" "}
              {presence?.source || "Idle"}
            </span>
          </div>

          {/* Activity Layout */}
          <div className="flex items-start gap-3 mt-1 text-left">
            {/* Logo/Icon */}
            <div className="relative w-[60px] h-[60px] rounded-[8px] bg-white/5 border border-white/5 flex-shrink-0 flex items-center justify-center overflow-visible">
              {presence ? (
                <img
                  src={iconUrl}
                  alt={appName}
                  className="w-full h-full rounded-[8px] object-contain p-1"
                  onError={(e) => {
                    // Fallback to initials square
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

              {/* Source micro-badge */}
              <div
                className="absolute bottom-[-3px] right-[-3px] w-[18px] h-[18px] rounded-full border border-white/10 bg-[#111214] flex items-center justify-center text-[9px]"
                title={`Data source: ${presence?.source || "Idle"}`}
              >
                {getSourceIcon(presence?.source || "Idle")}
              </div>
            </div>

            {/* Texts */}
            <div className="flex flex-col min-w-0 flex-1 justify-center py-0.5 select-text">
              <div className="font-semibold text-[13px] text-white leading-tight truncate">
                {presence?.large_text || "Better Rich Presence"}
              </div>
              <div className="text-[12px] text-zinc-300 leading-tight truncate mt-1">
                {presence?.details || "No application running"}
              </div>
              <div className="text-[12px] text-zinc-350 leading-tight truncate mt-1 text-zinc-300/80">
                {presence?.state || "Waiting for detection..."}
              </div>
              {elapsed && (
                <div className="text-[11.5px] text-zinc-400 leading-tight truncate mt-1 font-medium">
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
