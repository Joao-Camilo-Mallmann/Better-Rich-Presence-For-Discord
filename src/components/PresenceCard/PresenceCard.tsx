import { PresenceData } from "../../types";
import { useEffect, useState } from "react";

interface PresenceCardProps {
  presence: PresenceData | null;
}

export function PresenceCard({ presence }: PresenceCardProps) {
  const [elapsed, setElapsed] = useState<string>("00:00");

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
        setElapsed(`${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')} decorridos`);
      } else {
        setElapsed(`${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')} decorridos`);
      }
    };

    updateTimer();
    const interval = setInterval(updateTimer, 1000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  if (!presence) {
    return (
      <div className="bg-surface-indigo rounded-md p-4 flex flex-col gap-3 border border-hairline max-w-[400px] shadow-sm text-muted-ink text-center py-8 italic justify-center items-center">
        Aguardando conexão ou atividade...
      </div>
    );
  }

  const getSourceIcon = (source: string) => {
    switch (source) {
      case "Game": return "🎮";
      case "Work": return "💼";
      case "Browser": return "🌐";
      case "Idle": return "💤";
      case "Manual": return "✏️";
      default: return "🖥️";
    }
  };

  // Auto-generate a color based on the first letter for the placeholder
  const getAvatarColor = (name: string) => {
    const colors = ['#5865F2', '#ED4245', '#FEE75C', '#EB459E', '#57F287', '#F47B67'];
    const index = name.length > 0 ? name.charCodeAt(0) % colors.length : 0;
    return colors[index];
  };

  const initial = presence.large_text ? presence.large_text.charAt(0).toUpperCase() : '?';

  return (
    <div className="bg-surface-indigo rounded-md p-4 flex flex-col gap-3 border border-hairline max-w-[400px] shadow-sm">
      <div className="flex justify-between items-center">
        <span className="text-[11px] font-extrabold text-ink uppercase">JOGANDO UM JOGO</span>
        <span className="text-xs bg-surface-onyx px-2 py-[2px] rounded-sm text-muted-ink flex items-center gap-1" title={`Source: ${presence.source}`}>
          {getSourceIcon(presence.source)} {presence.source}
        </span>
      </div>
      
      <div className="flex gap-4">
        <div 
          className="w-[72px] h-[72px] rounded-md flex items-center justify-center text-3xl font-bold text-white shrink-0 shadow-sm" 
          style={{ backgroundColor: getAvatarColor(presence.large_text) }}
        >
          {initial}
        </div>
        
        <div className="flex flex-col justify-center overflow-hidden gap-[2px]">
          <div className="font-bold text-md text-ink whitespace-nowrap overflow-hidden text-ellipsis mb-[2px]" title={presence.large_text}>
            {presence.large_text || "Better Rich Presence"}
          </div>
          <div className="text-sm text-ink whitespace-nowrap overflow-hidden text-ellipsis" title={presence.details}>
            {presence.details}
          </div>
          <div className="text-sm text-ink whitespace-nowrap overflow-hidden text-ellipsis" title={presence.state}>
            {presence.state}
          </div>
          {elapsed && (
            <div className="text-sm text-ink whitespace-nowrap overflow-hidden text-ellipsis mt-[2px]">
              {elapsed}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
