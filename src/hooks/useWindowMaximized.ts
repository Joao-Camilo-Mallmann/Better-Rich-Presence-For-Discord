import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

/** Tracks whether the current window is maximized, updating on every resize. */
export function useWindowMaximized() {
  const [maximized, setMaximized] = useState(false);

  useEffect(() => {
    const appWindow = getCurrentWindow();
    let mounted = true;
    let unlisten: (() => void) | undefined;

    const sync = async () => {
      try {
        const value = await appWindow.isMaximized();
        if (mounted) setMaximized(value);
      } catch (error) {
        console.error("Failed to read window maximized state:", error);
      }
    };

    sync();
    appWindow
      .onResized(() => {
        sync();
      })
      .then((fn) => {
        if (mounted) {
          unlisten = fn;
        } else {
          fn();
        }
      })
      .catch((error) => {
        console.error("Failed to listen for window resize:", error);
      });

    return () => {
      mounted = false;
      unlisten?.();
    };
  }, []);

  return maximized;
}
