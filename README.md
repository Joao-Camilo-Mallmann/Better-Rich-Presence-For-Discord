<div align="center">
  <img src="public/logo.png" alt="Better Rich Presence For Discord Logo" width="200" />
</div>

# flow

_[Leia isso em Português](README.pt.md)_

Better Rich Presence For Discord is a Windows native application built with **Tauri v2**, **React**, and **Rust**. It automatically detects the programs you are running and updates your Discord Rich Presence accordingly, without any complex configuration.

### Features

- **Zero Configuration:** Comes with over 20 pre-configured popular apps (VSCode, browsers, Adobe suite, etc.).
- **Smart Engine:** Prioritizes games over work apps and has anti-flicker logic for smooth transitions.
- **Idle Detection:** Automatically changes your status to "Idle" when you step away from the keyboard.
- **Native Performance:** Uses Windows Win32 APIs for minimal CPU usage and memory footprint.

### Prerequisites

To run or build this project, you will need the following installed on your machine:

- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (Required for compiling Rust on Windows)

### How to Run

1. Clone the repository and navigate to the project directory:

   ```bash
   cd Better-Rich-Presence-For-Discord
   ```

2. Install the JavaScript dependencies:

   ```bash
   npm install
   ```

3. Run the application in development mode (this will compile the Rust backend and open the React frontend):

   ```bash
   npm run tauri dev
   ```

### How to Build (Production)

To create a standalone executable/installer for Windows:

```bash
npm run tauri build
```

The compiled `.exe` installers will be available in the `src-tauri/target/release/bundle/nsis/` directory.
