import { Monitor } from "lucide-react";
import { useEffect, useState } from "react";
import APP_CATALOG_JSON from "../../apps.json";
import { AppDefinition } from "../apps/app-types";

const appCatalog = APP_CATALOG_JSON as AppDefinition[];

interface AppIconProps {
  name: string;
  className?: string;
  size?: number;
}

export function AppIcon({ name, className = "", size = 24 }: AppIconProps) {
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    setFailed(false);
  }, [name]);

  if (!name || failed) {
    return (
      <Monitor
        className={className}
        style={{ width: size, height: size, flexShrink: 0 }}
      />
    );
  }

  const trimmed = name.trim();
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
    } else if (trimmed.includes(":")) {
      resolvedUrl = `https://api.iconify.design/${trimmed}.svg`;
    }
  }

  return (
    <img
      src={resolvedUrl}
      alt=""
      className={className}
      style={{ width: size, height: size, objectFit: "contain", flexShrink: 0 }}
      onError={() => setFailed(true)}
    />
  );
}
