import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AppRule } from "../types";

export function useAppRules() {
  const [rules, setRules] = useState<AppRule[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchRules = async () => {
    try {
      setLoading(true);
      const data = await invoke<AppRule[]>("get_app_rules");
      setRules(data);
    } catch (error) {
      console.error("Failed to fetch app rules:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { fetchRules(); }, []);

  // Wraps a Tauri command + refresh cycle with error logging and rethrow
  const withRefresh = async (cmd: string, args?: Record<string, unknown>) => {
    try {
      await invoke(cmd, args);
      await fetchRules();
    } catch (error) {
      console.error(`Failed to ${cmd}:`, error);
      throw error;
    }
  };

  const updateRule = (rule: AppRule) => withRefresh("update_app_rule", { rule });
  const addRule = (rule: AppRule) => withRefresh("add_app_rule", { rule });
  const deleteRule = (processName: string) => withRefresh("delete_app_rule", { processName });
  const resetToDefaults = () => withRefresh("reset_app_rules_to_defaults");
  /**
   * Reorder rules by providing the new order of process names.
   * The first name in the array becomes the highest-priority rule.
   */
  const reorderRules = (processNamesOrder: string[]) => withRefresh("reorder_app_rules", { processNamesOrder });

  return { rules, loading, updateRule, addRule, deleteRule, resetToDefaults, reorderRules, refresh: fetchRules };
}
