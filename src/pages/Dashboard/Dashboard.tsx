import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { PresenceCard } from "../../components/PresenceCard/PresenceCard";
import { usePresence } from "../../hooks/usePresence";
import { DiscordProfile } from "../../types";


const avatarPresets = [
  { name: "Blue", url: "https://cdn.discordapp.com/embed/avatars/0.png" },
  { name: "Gray", url: "https://cdn.discordapp.com/embed/avatars/1.png" },
  { name: "Green", url: "https://cdn.discordapp.com/embed/avatars/2.png" },
  { name: "Orange", url: "https://cdn.discordapp.com/embed/avatars/3.png" },
  { name: "Red", url: "https://cdn.discordapp.com/embed/avatars/4.png" },
  { name: "Pink", url: "https://cdn.discordapp.com/embed/avatars/5.png" },
];

export function Dashboard() {
  const { presence, connectionInfo, source, priorityInfo } = usePresence();
  const [elapsed, setElapsed] = useState<string>("0m");

  // Discord profile customizer state with localStorage persistence
  const [profile, setProfile] = useState<DiscordProfile>(() => {
    const saved = localStorage.getItem("discord_profile_customization");
    if (saved) {
      try {
        return JSON.parse(saved);
      } catch (e) {
        console.error("Failed to parse saved profile customization:", e);
      }
    }
    return {
      displayName: "Better RPC User",
      username: "better_rpc",
      avatarUrl: "",
      customStatus: "🚀 Customizando meu status",
      status: "online",
      themePrimary: "#5865f2",
      themeSecondary: "#2c327d",
      isGradient: true,
    };
  });

  useEffect(() => {
    localStorage.setItem(
      "discord_profile_customization",
      JSON.stringify(profile),
    );
  }, [profile]);

  useEffect(() => {
    const unlisten = listen<any>("discord-user-connected", (event) => {
      const user = event.payload;
      if (user) {
        setProfile((prev) => ({
          ...prev,
          displayName: prev.displayName === "Better RPC User" ? user.username : prev.displayName,
          username: prev.username === "better_rpc" ? user.username : prev.username,
          avatarUrl: user.avatar && !prev.avatarUrl ? user.avatar : prev.avatarUrl,
        }));
      }
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  useEffect(() => {
    if (!presence?.timestamp) {
      setElapsed("0m");
      return;
    }

    const updateTimer = () => {
      const now = Math.floor(Date.now() / 1000);
      const diff = Math.max(0, now - presence.timestamp!);
      const mins = Math.floor(diff / 60);
      setElapsed(`${mins}m`);
    };

    updateTimer();
    const interval = setInterval(updateTimer, 60000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  return (
    <div className="flex flex-col gap-5">
      {/* Visual Workspace Split */}
      <div className="grid grid-cols-1 lg:grid-cols-[auto_1fr] gap-6 items-start">
        {/* Left Column: Real-time Discord Popout Preview */}
        <div className="flex flex-col gap-3 shrink-0 items-center lg:items-start">
          <h3 className="text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display">
            Pré-visualização do Perfil
          </h3>
          <PresenceCard presence={presence} profile={profile} />
        </div>

        {/* Right Column: Customizer Dashboard Panel */}
        <div className="bg-surface-indigo rounded-md border border-hairline p-4 flex flex-col gap-4">
          <div className="flex items-center gap-2 border-b border-hairline/20 pb-3">
            <span className="text-xl">✨</span>
            <div>
              <h4 className="text-sm font-bold text-ink uppercase tracking-wider font-display">
                Personalizar Perfil do Discord
              </h4>
              <p className="text-[11px] text-muted-ink">
                Ajuste os temas de Discord do seu cartão de presença.
              </p>
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {/* Identity Group */}
            <div className="flex flex-col gap-3.5">
              <h5 className="text-[10px] font-bold text-primary uppercase tracking-wider">
                Identidade do Usuário
              </h5>

              {/* Display Name Input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Nome de Exibição
                </label>
                <input
                  type="text"
                  value={profile.displayName}
                  onChange={(e) =>
                    setProfile({ ...profile, displayName: e.target.value })
                  }
                  placeholder="Nome de exibição..."
                  className="bg-surface-onyx border border-hairline/50 text-ink text-sm px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />
              </div>

              {/* Username Input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Nome de Usuário (@)
                </label>
                <input
                  type="text"
                  value={profile.username}
                  onChange={(e) =>
                    setProfile({
                      ...profile,
                      username: e.target.value
                        .toLowerCase()
                        .replace(/\s+/g, ""),
                    })
                  }
                  placeholder="Nome de usuário..."
                  className="bg-surface-onyx border border-hairline/50 text-ink text-sm px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />
              </div>

              {/* Custom Status Input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Status Personalizado
                </label>
                <input
                  type="text"
                  value={profile.customStatus}
                  onChange={(e) =>
                    setProfile({ ...profile, customStatus: e.target.value })
                  }
                  placeholder="Defina um status personalizado..."
                  className="bg-surface-onyx border border-hairline/50 text-ink text-sm px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />
              </div>
            </div>

            {/* Avatar & Themes Group */}
            <div className="flex flex-col gap-3.5">
              <h5 className="text-[10px] font-bold text-magenta-accent uppercase tracking-wider">
                Aparência & Status
              </h5>

              {/* Status Select */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Status de Presença
                </label>
                <select
                  value={profile.status}
                  onChange={(e) =>
                    setProfile({ ...profile, status: e.target.value as any })
                  }
                  className="bg-surface-onyx border border-hairline/50 text-ink text-sm px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full cursor-pointer"
                >
                  <option value="online">🟢 Online</option>
                  <option value="idle">🌙 Ausente (Idle)</option>
                  <option value="dnd">🔴 Não Perturbe (DND)</option>
                  <option value="offline">⚪ Invisível (Offline)</option>
                </select>
              </div>

              {/* Avatar URL Input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  URL do Avatar
                </label>
                <input
                  type="text"
                  value={profile.avatarUrl}
                  onChange={(e) =>
                    setProfile({ ...profile, avatarUrl: e.target.value })
                  }
                  placeholder="Cole o link da imagem..."
                  className="bg-surface-onyx border border-hairline/50 text-ink text-sm px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />

                {/* Avatar Presets Selection */}
                <div className="flex items-center gap-1.5 mt-1.5">
                  <span className="text-[9px] text-muted-ink font-bold uppercase mr-1">
                    Presets:
                  </span>
                  <div className="flex gap-1">
                    {avatarPresets.map((preset) => (
                      <button
                        key={preset.name}
                        onClick={() =>
                          setProfile({ ...profile, avatarUrl: preset.url })
                        }
                        className={`w-6 h-6 rounded-full overflow-hidden border transition ${
                          profile.avatarUrl === preset.url
                            ? "border-primary scale-110"
                            : "border-hairline hover:border-white/40"
                        }`}
                        title={`Usar avatar ${preset.name}`}
                      >
                        <img
                          src={preset.url}
                          alt={preset.name}
                          className="w-full h-full object-cover"
                        />
                      </button>
                    ))}
                    <button
                      onClick={() => setProfile({ ...profile, avatarUrl: "" })}
                      className="text-[9px] font-bold text-muted-ink px-1.5 py-0.5 rounded-xs border border-hairline/40 hover:bg-white/5"
                    >
                      Limpar
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Info Status Row */}
      <div className="flex flex-wrap gap-2 text-xs border-t border-hairline/25 pt-4">
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold flex items-center gap-1.5 shadow-sm">
          <span
            className={`w-2 h-2 rounded-full ${connectionInfo.state === "Connected" ? "bg-green-accent animate-pulse" : "bg-danger"}`}
          />
          Engine Status:{" "}
          {connectionInfo.state === "Connected"
            ? "Conectado ao Discord"
            : "Desconectado do Discord"}
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Fonte:{" "}
          <span className="text-ink font-bold">{source}</span>
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Sessão:{" "}
          <span className="text-ink font-bold">{elapsed}</span>
        </span>

        {/* Priority Mode Indicator — shown when a higher-priority app overrides foreground */}
        {priorityInfo.active && (
          <span
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-sm font-semibold text-ink shadow-sm border animate-pulse"
            style={{
              background: "rgba(236,72,189,0.12)",
              borderColor: "rgba(236,72,189,0.35)",
            }}
            title={`Em foco: ${priorityInfo.foreground_app || "desconhecido"}`}
          >
            <span className="text-[13px]">⬆️</span>
            <span>
              Priorizando:{" "}
              <span className="font-bold" style={{ color: "#ec48bd" }}>
                {priorityInfo.prioritized_app || "app"}
              </span>
            </span>
          </span>
        )}
      </div>
    </div>
  );
}
