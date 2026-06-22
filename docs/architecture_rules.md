# Architecture & Presence Engine Rules

## ⚙️ Máquina de Estados e Prioridades
A engine de presenças deve resolver conflitos seguindo estritamente a prioridade das fontes (do maior para o menor):
1. **Game** (Prioridade 0 - Máxima): Detecção direta de jogos.
2. **Manual** (Prioridade 1): Configurações manuais feitas diretamente na UI.
3. **Work** (Prioridade 2): Editores de código e IDEs (como VS Code, Android Studio).
4. **Browser** (Prioridade 3): Navegadores de internet em uso.
5. **Idle** (Prioridade 4 - Mínima): Estado de inatividade do usuário.

## ⏱️ Watchers e Timings
- **Window Watcher (Win32):** Deve fazer polling de processo e título da janela em foco a cada 2 ou 3 segundos.
- **Debounce:** Aplicar settle delay de 3 segundos antes de registrar uma nova alteração na presença.
- **Discord Rate-Limit:** Respeitar o limite de 15 segundos entre atualizações reais de IPC no Discord.

## 💾 Persistência de Dados
- **Desktop (Tauri):** Salvar configurações usando o plugin `tauri-plugin-store`.
- **Web (Browser/Vercel):** Fallback dinâmico para `localStorage` para manter o simulador interativo funcional.
