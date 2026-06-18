# Adições e Regras Extras

---

# Engine Central de Presence

Não permitir que qualquer parte da aplicação chame `set_activity()` diretamente.

Criar uma PresenceEngine central responsável por:

- Receber eventos.
- Resolver prioridades.
- Comparar com o estado atual.
- Evitar updates desnecessários.
- Chamar `clear_activity()` antes de qualquer troca.
- Fazer debounce de mudanças.

Arquitetura:

```text
Watchers
    ↓
Event Bus
    ↓
Presence Engine
    ↓
Discord RPC
```

Nenhum watcher conversa diretamente com o Discord.

---

# Debounce e Rate Limit

Evitar spam de atualizações.

Nunca atualizar o Discord mais rápido que:

- 1 update a cada 15 segundos.

Mudanças rápidas:

Chrome → VSCode → Chrome → VSCode

devem ser agregadas.

Só enviar update se:

- details mudou
- state mudou
- imagens mudaram

Se nada mudou:

Não chamar `set_activity()`.

---

# Máquina de Estados

Criar enum:

```rust
enum PresenceSource {
    Game,
    Work,
    Manual,
    Browser,
    Idle,
}
```

e:

```rust
enum PresenceState {
    Disconnected,
    WaitingDiscord,
    Connected,
    PausedByGame,
    Updating,
}
```

Toda UI consome apenas esse estado.

---

# Sistema Idle

Detectar inatividade.

Exemplos:

5 minutos sem teclado ou mouse:

```text
Ausente
```

30 minutos:

```text
Longe do computador
```

Configuração:

- Desativado
- 5 min
- 10 min
- 30 min

Pode substituir perfil manual.

Mas nunca substitui jogos.

---

# Templates Prontos

Perfis pré-instalados:

### Programando

Details:

```text
Codando
```

State:

```text
Fazendo commits duvidosos
```

---

### Trabalhando

```text
Trabalhando
```

---

### Estudando

```text
Estudando
```

---

### Reunião

```text
Em reunião
```

---

### Editando vídeo

```text
Editando vídeos
```

---

# Biblioteca de Aplicativos

Presets editáveis:

VSCode

Cursor

IntelliJ

Android Studio

Visual Studio

Figma

Photoshop

Premiere

After Effects

Blender

Excel

Word

PowerPoint

Notion

Obsidian

Slack

Discord

Terminal

Docker Desktop

GitHub Desktop

Spotify

Cada preset possui:

```rust
struct AppRule {
    process_name: String,
    details: String,
    state: String,
    large_image: String,
    priority: u32,
    enabled: bool
}
```

---

# Atualização Automática

Usar:

```toml
tauri-plugin-updater
```

Permitir:

- Verificar atualizações.
- Atualizar em um clique.
- Atualização silenciosa opcional.

---

# Logs

Criar:

```toml
tauri-plugin-log
```

Logs:

- info
- warning
- error

Rotação automática.

Nunca mostrar stack traces ao usuário.

---

# Exportar e Importar

Permitir:

Exportar:

```json
profiles.json
```

Contendo:

- perfis
- prioridades
- regras
- configurações

Importar em outro PC.

---

# Backup Automático

Salvar:

```text
profiles.backup.json
```

Sempre antes de alterações.

---

# Multi Monitor

Ler somente a janela em foco.

Não capturar tela.

Não analisar pixels.

---

# Modo Privacidade

Quando ativo:

Ignorar:

YouTube

Netflix

Twitch

Chrome

Firefox

Edge

Mostrar apenas:

```text
Usando o computador
```

---

# Anti Flicker

Se trocar rapidamente entre programas:

VSCode
↓
Chrome
↓
Terminal
↓
Chrome

esperar alguns segundos antes da troca.

Evita o Discord mudando a cada segundo.

---

# Desconexão Segura

Ao:

- sair do app
- crash
- fechar pelo tray
- reiniciar Windows

sempre executar:

```rust
clear_activity()
disconnect_rpc()
```

Garantindo que não fique uma presence presa.

---

# Watchdog do Discord

Detectar:

- Discord fechado.
- Discord reiniciado.
- Pipe quebrado.
- IPC perdido.

Reconectar automaticamente.

Sem interação do usuário.

---

# Transparência

Dashboard:

## Fonte Atual

🎮 Jogo

💼 Trabalho

👤 Perfil Manual

🌐 Navegador

💤 Idle

Mostrar:

```text
Exibindo:
VSCode

Fonte:
Software de Trabalho

Prioridade:
2
```

---

# Segurança

Não coletar:

- URLs completas.
- Histórico do navegador.
- Senhas.
- Cookies.
- Conteúdo da tela.
- Conteúdo das abas.
- Teclas digitadas.

Apenas:

- Nome do processo.
- Nome da janela ativa.
- Domínio básico opcional.

Tudo local.

Sem telemetria.

Sem servidores externos.

Sem analytics.

---

# Objetivos de Performance

RAM:

20–40 MB

CPU Idle:

< 0,5%

Sem Node.js

Sem Electron

Sem sidecars

Sem polling agressivo

Baixo consumo de bateria

Inicialização:

< 1 segundo

Tempo de reconexão:

< 3 segundos

---

# Objetivo Final

O produto deve se comportar como um aplicativo nativo do Windows, semelhante ao Discord, Spotify ou Steam:

- Invisível quando não necessário.
- Seguro.
- Extremamente leve.
- Amigável para usuários leigos.
- Altamente configurável para usuários avançados.
- Sem interferir em jogos reais.
- Totalmente local.
- Zero configuração obrigatória.
- Pronto para distribuição pública.
