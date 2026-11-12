import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { PlatformStatus } from "../types";

const REFRESH_MS = 10_000;

/** Polls the backend's platform / activity-detection status while mounted. */
export function usePlatformStatus() {
  const [status, setStatus] = useState<PlatformStatus | null>(null);

  useEffect(() => {
    let mounted = true;

    const fetchStatus = async () => {
      try {
        const data = await invoke<PlatformStatus>("get_platform_status");
        if (mounted) setStatus(data);
      } catch (error) {
        console.error("Failed to fetch platform status:", error);
      }
    };

    fetchStatus();
    const timer = window.setInterval(fetchStatus, REFRESH_MS);
    return () => {
      mounted = false;
      window.clearInterval(timer);
    };
  }, []);

  return status;
}
