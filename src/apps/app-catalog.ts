import { AppDefinition } from "./app-types";

export const APP_CATALOG: AppDefinition[] = [
  {
    id: "visual-studio-code",
    name: "Visual Studio Code",
    executables: ["Code.exe", "code"],
    icon: "simple-icons:visualstudiocode",
    discordAsset: "vscode"
  },
  {
    id: "google-chrome",
    name: "Google Chrome",
    executables: ["chrome.exe"],
    icon: "simple-icons:googlechrome",
    discordAsset: "chrome"
  },
  {
    id: "spotify",
    name: "Spotify",
    executables: ["Spotify.exe"],
    icon: "simple-icons:spotify",
    discordAsset: "spotify"
  },
  {
    id: "cursor",
    name: "Cursor",
    executables: ["cursor.exe", "Cursor.exe"],
    icon: "lucide:square-terminal",
    discordAsset: "cursor"
  },
  {
    id: "antigravity-ide",
    name: "Antigravity IDE",
    executables: ["antigravity ide.exe", "antigravity-ide.exe"],
    icon: "lucide:orbit",
    discordAsset: "antigravity"
  },
  {
    id: "zen-browser",
    name: "Zen Browser",
    executables: ["zen.exe"],
    icon: "lucide:compass",
    discordAsset: "zen"
  },
  {
    id: "intellij-idea",
    name: "IntelliJ IDEA",
    executables: ["idea64.exe"],
    icon: "simple-icons:intellijidea",
    discordAsset: "intellij"
  },
  {
    id: "webstorm",
    name: "WebStorm",
    executables: ["webstorm64.exe"],
    icon: "simple-icons:webstorm",
    discordAsset: "webstorm"
  },
  {
    id: "pycharm",
    name: "PyCharm",
    executables: ["pycharm64.exe"],
    icon: "simple-icons:pycharm",
    discordAsset: "pycharm"
  },
  {
    id: "phpstorm",
    name: "PhpStorm",
    executables: ["phpstorm64.exe"],
    icon: "simple-icons:phpstorm",
    discordAsset: "phpstorm"
  }
];
