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

  useEffect(() => {
    fetchRules();
  }, []);

  const updateRule = async (rule: AppRule) => {
    try {
      await invoke("update_app_rule", { rule });
      await fetchRules();
    } catch (error) {
      console.error("Failed to update rule:", error);
      throw error;
    }
  };

  const addRule = async (rule: AppRule) => {
    try {
      await invoke("add_app_rule", { rule });
      await fetchRules();
    } catch (error) {
      console.error("Failed to add rule:", error);
      throw error;
    }
  };

  const deleteRule = async (processName: string) => {
    try {
      await invoke("delete_app_rule", { processName });
      await fetchRules();
    } catch (error) {
      console.error("Failed to delete rule:", error);
      throw error;
    }
  };

  const resetToDefaults = async () => {
    try {
      await invoke("reset_app_rules_to_defaults");
      await fetchRules();
    } catch (error) {
      console.error("Failed to reset rules:", error);
      throw error;
    }
  };

  /**
   * Reorder rules by providing the new order of process names.
   * The first name in the array becomes the highest-priority rule.
   */
  const reorderRules = async (processNamesOrder: string[]) => {
    try {
      await invoke("reorder_app_rules", { processNamesOrder });
      await fetchRules();
    } catch (error) {
      console.error("Failed to reorder rules:", error);
      throw error;
    }
  };

  return { rules, loading, updateRule, addRule, deleteRule, resetToDefaults, reorderRules, refresh: fetchRules };
}
