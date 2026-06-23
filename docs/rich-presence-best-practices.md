# Rich Presence — Boas Práticas

> Baseado na [documentação oficial do Discord](https://docs.discord.com/developers/rich-presence/best-practices).  
> Adaptado para o projeto **Discord Rich Presence Manager** (Tauri v2 + Vue 3).

---

## 1. Filosofia dos dados exibidos

O Rich Presence é a **primeira impressão** que outros usuários terão da atividade. Os dados precisam responder objetivamente:

- O que o usuário está fazendo agora?
- Ele pode ser convidado para uma partida/sessão?
- Quanto tempo já passou ou resta?
- Em qual estado está a party?

---

## 2. Strings curtas e objetivas

| Campo     | Correto ✅            | Errado ❌                                      |
|-----------|-----------------------|------------------------------------------------|
| `details` | `Ranked — Control Point` | `Atualmente jogando no modo ranqueado de Controle de Ponto` |
| `state`   | `Em fila (2 de 3)`    | `Aguardando o terceiro jogador entrar na fila` |

**Regras:**
- `details` e `state` devem ser **snippets**, não frases completas.
- Garantir que caibam em **uma linha** — especialmente no pop-out de perfil pequeno.
- Evitar repetição de informação entre os campos.

---

## 3. Dados acionáveis

Mantenha os dados **sincronizados com o estado real** da sessão:

- Party size sempre atualizado (`partySize` / `partyMax`).
- Estado claro: `In Queue`, `In Game`, `In Menus`, `Spectating`, etc.
- Incluir modo de jogo e tipo (ranqueado vs. casual) quando relevante.
- Remover o `joinSecret` quando o usuário **não puder mais convidar** ninguém.

---

## 4. Use todos os campos disponíveis

| Campo           | Uso recomendado                                      |
|-----------------|------------------------------------------------------|
| `details`       | O que o usuário está fazendo (ação principal)        |
| `state`         | Estado da party / fase atual                         |
| `largeImageKey` | Arte consistente para todos na party (ex: jogo/mapa) |
| `largeImageText`| Tooltip com nome do mapa ou contexto adicional       |
| `smallImageKey` | Customização por jogador (personagem, rank, classe)  |
| `smallImageText`| Tooltip com nome do personagem ou ícone              |
| `startTimestamp`| Timestamp de início (exibe "há X min")               |
| `endTimestamp`  | Countdown (exibe "X min restantes")                  |
| `partySize`     | Quantidade atual na party                            |
| `partyMax`      | Tamanho máximo da party                              |

> **Dica:** informações secundárias (nome do mapa, personagem) vão nos **tooltips** das imagens — economizando espaço nos campos de texto.

---

## 5. Arte de qualidade

- Resolução mínima recomendada: **1024 × 1024 px**.
- `largeImage`: deve ser **consistente** para todos os membros da party.
- `smallImage`: espaço para **customização individual** (personagem, rank, papel).
- A arte deve ser legível e clara — evitar imagens muito escuras ou com excesso de detalhes.
- Ter artwork para **cada estado** diferente, incluindo menu principal / idle.

---

## 6. Checklist de lançamento

### Strings
- [ ] Todos os campos aplicáveis estão preenchidos?
- [ ] As strings cabem em uma linha sem quebrar?
- [ ] Checou no pop-out de perfil pequeno?
- [ ] Fica claro o que o usuário está fazendo?
- [ ] O estado da party está visível (solo / grupo / em fila)?

### Arte
- [ ] Imagens com pelo menos 1024×1024 px?
- [ ] Arte limpa, expressiva e não excessivamente detalhada?
- [ ] Existe arte para cada estado diferente?
- [ ] Tooltips de imagem (`largeImageText` / `smallImageText`) configurados?

### Joins / Convites
- [ ] `joinSecret` implementado corretamente (se aplicável)?
- [ ] O invite representa corretamente o tamanho e slots disponíveis da party?
- [ ] `joinSecret` é **removido** quando o usuário não pode mais convidar?
- [ ] Discord *e* não-Discord users são contabilizados no party size?

---

## 7. Notas específicas para o Rich Presence Manager

> Considerações para a implementação no app Tauri.

- **Prioridade de processos:** garantir que jogos com presença ativa não sejam sobrescritos por apps de trabalho, a menos que explicitamente configurado.
- **Work-app detection:** apps de trabalho devem ter arte e strings próprias, discretas e profissionais.
- **Browser tab tracking:** exibir apenas tabs relevantes; evitar vazar informação sensível (URLs privadas, dashboards internos).
- **Estado idle:** definir um comportamento padrão quando nenhum processo rastreado está ativo — limpar a presença ou exibir um estado neutro.
- **Update rate:** o Discord limita atualizações a **1 vez a cada 15 segundos** — batchar mudanças de estado para respeitar esse limite.
