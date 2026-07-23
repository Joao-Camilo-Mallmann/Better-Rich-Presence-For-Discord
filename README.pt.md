<div align="center">
  <img src="public/logo.png" alt="Better Rich Presence For Discord Logo" width="200" />
  <br />
  <br />
  
  # 🎮 Better Rich Presence for Discord
  
  **Automatically showcase your active window on Discord with zero hassle.**
  
  [![Release](https://img.shields.io/github/v/release/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord?style=for-the-badge&color=5865F2)](https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/releases/latest)
  [![Platform](https://img.shields.io/badge/Platform-Windows-blue?style=for-the-badge&logo=windows)](https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/releases)
  [![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://reactjs.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app/)
  [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

  <img src="public/img/site.png" alt="Better Rich Presence Website Screenshot" width="700" style="border-radius: 10px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);" />
</div>

<br />

---

## 🚀 Welcome to Version 1!
We are incredibly excited to announce the stable `v1` release of Better Rich Presence! 

📥 **[Download the Latest Installer](https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/releases/latest/download/Better.Rich.Presence_0.1.0_x64-setup.exe)**

📜 **Full Changelog**: https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/commits/v1

---

## ✨ What it Does

**Better Rich Presence** is an advanced, ultra-lightweight desktop application that automatically detects your currently active window and seamlessly updates your Discord Rich Presence to match. 

Whether you are **coding in VSCode**, **designing in Figma**, or **browsing the web**, Better RPC ensures your Discord status accurately reflects your activity in real-time, completely hands-free!

## ⚡ Advantages & Key Features

- 🛠️ **Zero Configuration:** Comes with over 20 pre-configured popular applications (VSCode, browsers, Adobe suite, etc.) ready to use out of the box.
- 🧠 **Smart Priority Engine:** Prioritizes games and important applications over background tasks, ensuring your most relevant activity is what your friends see.
- 🛡️ **Anti-Flicker Logic:** Smooth transitions between apps, preventing status flickering when quickly tabbing between windows.
- 💤 **Idle Detection:** Automatically changes your status to "Idle" when you step away from your keyboard for a period of time.
- 🚀 **Native Performance:** Built with Rust and Tauri using native Windows Win32 APIs, ensuring minimal CPU usage and a tiny memory footprint.
- 🎨 **Modern UI:** Features a beautiful, retro-arcade Neo-Brutalist design interface built with React and TailwindCSS.

---

## 🛠️ How to Compile and Run

### 📋 Prerequisites

To run or build this project locally, you will need the following dependencies installed on your machine:
- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://www.rust-lang.org/tools/install)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (Required for compiling Rust on Windows. Ensure you select **Desktop development with C++**)

### 💻 Running in Development Mode

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord.git
   cd Better-Rich-Presence-For-Discord
   ```
2. **Install JavaScript dependencies:**
   ```bash
   npm install
   ```
3. **Run the application** *(Compiles Rust backend & starts React frontend)*:
   ```bash
   npm run tauri dev
   ```

### 📦 Building for Production

To create a standalone `.exe` installer for Windows:
```bash
npm run tauri build
```
The compiled installer will be outputted to the `src-tauri/target/release/bundle/nsis/` directory.

---
<div align="center">
  <i>Built with ❤️ for gamers, developers, and creators.</i>
</div>
