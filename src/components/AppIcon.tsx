import { Monitor } from "lucide-react";
import { useState } from "react";
import APP_CATALOG_JSON from "../../apps.json";
import { AppDefinition } from "../apps/app-types";

const appCatalog = APP_CATALOG_JSON as AppDefinition[];
const PUBLIC_ICONS_MARKER = "/public/assets/icons/";

function localIconFallback(url: string): string | null {
  const markerIndex = url.indexOf(PUBLIC_ICONS_MARKER);
  if (markerIndex === -1) {
    return null;
  }
  const rawFile = url.slice(markerIndex + PUBLIC_ICONS_MARKER.length);
  const file = rawFile.split(/[?#]/)[0];
  if (!file || !file.endsWith(".png") || file.includes("/")) {
    return null;
  }
  return `/assets/icons/${file}`;
}

interface AppIconProps {
  name: string;
  className?: string;
  size?: number;
}

export function AppIcon({ name, className = "", size = 24 }: AppIconProps) {
  const [errorState, setErrorState] = useState<{
    forName: string;
    stage: "fallback" | "failed";
  } | null>(null);

  const activeError =
    errorState && errorState.forName === name ? errorState.stage : null;
  const trimmed = name ? name.trim() : "";

  if (!trimmed || activeError === "failed") {
    return (
      <Monitor
        className={className}
        style={{ width: size, height: size, flexShrink: 0 }}
      />
    );
  }

  let resolvedUrl = trimmed;
  const isUrl =
    trimmed.startsWith("http://") ||
    trimmed.startsWith("https://") ||
    trimmed.startsWith("/");

  if (!isUrl) {
    const app = appCatalog.find(
      (a) => a.id === trimmed || a.executables.includes(trimmed.toLowerCase()),
    );
    if (app && app.icon_url) {
      resolvedUrl = app.icon_url;
    }
  }

  const localFallbackUrl = localIconFallback(resolvedUrl);
  const imgSrc =
    activeError === "fallback" && localFallbackUrl
      ? localFallbackUrl
      : resolvedUrl;

  const handleError = () => {
    if (activeError !== "fallback" && localFallbackUrl) {
      setErrorState({ forName: name, stage: "fallback" });
    } else {
      setErrorState({ forName: name, stage: "failed" });
    }
  };

  return (
    <img
      src={imgSrc}
      alt=""
      className={className}
      style={{ width: size, height: size, objectFit: "contain", flexShrink: 0 }}
      onError={handleError}
    />
  );
}
