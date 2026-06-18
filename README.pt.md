# Better Rich Presence For Discord

_[Read this in English](README.md)_

O **Better Rich Presence For Discord** é um aplicativo nativo para Windows construído com **Tauri v2**, **React** e **Rust**. Ele detecta automaticamente os programas que você está usando e atualiza seu "Status (Rich Presence)" no Discord, sem precisar de configurações complexas.

### Recursos

- **Zero Configuração:** Já vem com mais de 20 aplicativos populares pré-configurados (VSCode, navegadores, pacote Adobe, etc.).
- **Motor Inteligente:** Prioriza jogos sobre aplicativos de trabalho e possui lógica "anti-flicker" para evitar que o status fique piscando ao trocar de janelas.
- **Detecção de Ausência (Idle):** Muda automaticamente seu status para "Inativo" quando você se afasta do teclado.
- **Performance Nativa:** Utiliza a API Win32 do Windows para gastar o mínimo possível de CPU e memória.

### Pré-requisitos

Para rodar ou compilar este projeto, você precisará ter instalado em sua máquina:

- [Node.js](https://nodejs.org/) (v18 ou superior)
- [Rust](https://www.rust-lang.org/tools/install)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (Necessário para compilar Rust no Windows)

### Como Rodar

1. Clone o repositório e acesse a pasta do projeto:

   ```bash
   cd Better-Rich-Presence-For-Discord
   ```

2. Instale as dependências do JavaScript:

   ```bash
   npm install
   ```

3. Rode o aplicativo em modo de desenvolvimento (isso irá compilar o backend em Rust e abrir a interface em React):

   ```bash
   npm run tauri dev
   ```

### Como Compilar (Produção)

Para gerar um instalador/executável independente para Windows:

```bash
npm run tauri build
```

O instalador `.exe` finalizado estará disponível na pasta `src-tauri/target/release/bundle/nsis/`.
