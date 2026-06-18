import { useState, useEffect } from "react";
import styles from "./Settings.module.css";
import { useSettings } from "../../hooks/useSettings";

export function Settings() {
  const { settings, loading, updateSettings } = useSettings();
  const [localSettings, setLocalSettings] = useState(settings);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (settings && !localSettings) {
      setLocalSettings(settings);
    }
  }, [settings]);

  if (loading || !localSettings) {
    return <div className={styles.loading}>Carregando configurações...</div>;
  }

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await updateSettings(localSettings);
      
      // Optional: show a success toast or indicator
      setTimeout(() => setIsSaving(false), 1000);
    } catch (e) {
      console.error(e);
      setIsSaving(false);
    }
  };

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <h2 className={styles.title}>Configurações</h2>
        <p className={styles.subtitle}>Ajuste o comportamento do Better Rich Presence</p>
      </header>

      <div className={styles.settingsGrid}>
        <section className={styles.section}>
          <h3 className={styles.sectionTitle}>Geral</h3>
          
          <div className={styles.settingItem}>
            <div className={styles.settingInfo}>
              <span className={styles.settingName}>Iniciar com o Windows</span>
              <span className={styles.settingDesc}>Abre o aplicativo minimizado ao iniciar o computador.</span>
            </div>
            <label className={styles.toggle}>
              <input 
                type="checkbox" 
                checked={localSettings.autostart_enabled}
                onChange={(e) => setLocalSettings({...localSettings, autostart_enabled: e.target.checked})}
              />
              <span className={styles.slider}></span>
            </label>
          </div>
        </section>

        <section className={styles.section}>
          <h3 className={styles.sectionTitle}>Inatividade (Idle)</h3>
          
          <div className={styles.settingItem}>
            <div className={styles.settingInfo}>
              <span className={styles.settingName}>Habilitar Inatividade</span>
              <span className={styles.settingDesc}>Muda o status no Discord quando você não mexe no mouse/teclado.</span>
            </div>
            <label className={styles.toggle}>
              <input 
                type="checkbox" 
                checked={localSettings.idle_enabled}
                onChange={(e) => setLocalSettings({...localSettings, idle_enabled: e.target.checked})}
              />
              <span className={styles.slider}></span>
            </label>
          </div>

          <div className={styles.settingItem}>
            <div className={styles.settingInfo}>
              <span className={styles.settingName}>Tempo Limite (minutos)</span>
              <span className={styles.settingDesc}>Quanto tempo até ser considerado ausente.</span>
            </div>
            <input 
              className={styles.numberInput}
              type="number" 
              min="1" 
              max="60"
              value={localSettings.idle_threshold_minutes}
              onChange={(e) => setLocalSettings({...localSettings, idle_threshold_minutes: parseInt(e.target.value) || 5})}
              disabled={!localSettings.idle_enabled}
            />
          </div>

          <div className={styles.settingItem}>
            <div className={styles.settingInfo}>
              <span className={styles.settingName}>Mensagem de Ausência</span>
              <span className={styles.settingDesc}>O que mostrar no Discord quando ausente.</span>
            </div>
            <input 
              className={styles.textInput}
              type="text" 
              value={localSettings.idle_message}
              onChange={(e) => setLocalSettings({...localSettings, idle_message: e.target.value})}
              disabled={!localSettings.idle_enabled}
            />
          </div>
        </section>

        <section className={styles.section}>
          <h3 className={styles.sectionTitle}>Avançado (Motor)</h3>
          
          <div className={styles.settingItem}>
            <div className={styles.settingInfo}>
              <span className={styles.settingName}>Debounce (segundos)</span>
              <span className={styles.settingDesc}>Tempo mínimo entre atualizações enviadas ao Discord (evita rate limit). O Discord exige 15s.</span>
            </div>
            <input 
              className={styles.numberInput}
              type="number" 
              min="15" 
              max="60"
              value={localSettings.debounce_seconds}
              onChange={(e) => setLocalSettings({...localSettings, debounce_seconds: parseInt(e.target.value) || 15})}
            />
          </div>

          <div className={styles.settingItem}>
            <div className={styles.settingInfo}>
              <span className={styles.settingName}>Atraso Anti-flicker (segundos)</span>
              <span className={styles.settingDesc}>Tempo de estabilização ao trocar de janelas rápidas (Alt+Tab).</span>
            </div>
            <input 
              className={styles.numberInput}
              type="number" 
              min="1" 
              max="10"
              value={localSettings.settle_delay_seconds}
              onChange={(e) => setLocalSettings({...localSettings, settle_delay_seconds: parseInt(e.target.value) || 3})}
            />
          </div>
        </section>
      </div>

      <div className={styles.actions}>
        <button 
          className={styles.btnSave} 
          onClick={handleSave}
          disabled={isSaving}
        >
          {isSaving ? "Salvando..." : "Salvar Configurações"}
        </button>
      </div>
    </div>
  );
}
