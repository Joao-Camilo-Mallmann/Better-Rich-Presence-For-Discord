import { useState } from "react";
import { PresenceCard } from "../../components/PresenceCard/PresenceCard";
import { DiscordProfile, PresenceData } from "../../types";

interface LandingPageProps {
  onEnterApp: () => void;
}

export function LandingPage({ onEnterApp }: LandingPageProps) {
  // Live Demo customization state
  const [profile, setProfile] = useState<DiscordProfile>({
    displayName: "Você no Discord",
    username: "seu_usuario",
    avatarUrl: "",
    customStatus: "🚀 Testando o Better RPC!",
    status: "online",
    themePrimary: "#5865f2",
    themeSecondary: "#ec48bd",
    isGradient: true,
  });

  const [demoPresence, setDemoPresence] = useState<PresenceData>({
    source: "Manual",
    large_text: "Visual Studio Code",
    details: "Editando src/App.tsx",
    state: "Linha 42: Coluna 12",
    large_image: "vscode",
    timestamp: Math.floor(Date.now() / 1000) - 300,
  });

  const [faqOpen, setFaqOpen] = useState<number | null>(null);

  // Softwares showcased
  const softwares = [
    { name: "VS Code", icon: "💻", key: "vscode" },
    { name: "Spotify", icon: "🎵", key: "spotify" },
    { name: "Figma", icon: "🎨", key: "figma" },
    { name: "Minecraft", icon: "⛏️", key: "minecraft" },
    { name: "Valorant", icon: "🔫", key: "valorant" },
    { name: "Steam", icon: "🎮", key: "steam" },
    { name: "Chrome", icon: "🌐", key: "chrome" },
    { name: "Custom App", icon: "⚙️", key: "custom" },
  ];

  return (
    <div className="w-full min-h-screen bg-canvas text-ink font-body selection:bg-primary/30 selection:text-ink overflow-x-hidden">
      {/* Background Gradient Mesh */}
      <div className="absolute top-0 left-0 right-0 h-[600px] bg-gradient-to-b from-primary/10 via-magenta-accent/5 to-transparent pointer-events-none z-0" />

      {/* Header / Nav */}
      <header className="relative z-10 max-w-7xl mx-auto px-6 py-6 flex justify-between items-center">
        <div className="flex items-center gap-2.5">
          <img
            src="/logo.png"
            alt="Better RPC Logo"
            className="w-8 h-6 object-contain"
          />
          <span className="font-display font-extrabold text-lg tracking-tight">
            Better RPC
          </span>
        </div>
        <nav className="hidden md:flex items-center gap-8 text-sm font-semibold text-muted-ink">
          <a href="#features" className="hover:text-ink transition-colors">
            Funcionalidades
          </a>
          <a href="#demo" className="hover:text-ink transition-colors">
            Simulador
          </a>
          <a href="#developers" className="hover:text-ink transition-colors">
            Desenvolvedores
          </a>
          <a href="#faq" className="hover:text-ink transition-colors">
            FAQ
          </a>
        </nav>
        <div className="flex items-center gap-3">
          <a
            href="https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord"
            target="_blank"
            rel="noopener noreferrer"
            className="hidden sm:flex items-center gap-1.5 px-4 py-2 rounded-sm border border-hairline bg-surface-indigo/40 hover:bg-surface-indigo text-xs font-bold transition-all"
          >
            ⭐ GitHub
          </a>
          <button
            onClick={onEnterApp}
            className="px-5 py-2 rounded-sm bg-green-accent text-ink-dark font-display font-bold text-xs hover:scale-[1.03] transition-all cursor-pointer shadow-md active:scale-95"
          >
            Abrir App Web
          </button>
        </div>
      </header>

      {/* Hero Section */}
      <section className="relative z-10 max-w-6xl mx-auto px-6 pt-16 pb-20 text-center flex flex-col items-center">
        <div className="inline-flex items-center gap-2 bg-magenta-accent/15 border border-magenta-accent/30 text-magenta-accent px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-6 animate-pulse">
          ✨ 100% Open Source & Customizável
        </div>

        <h1 className="font-display font-extrabold text-5xl md:text-7xl tracking-tighter leading-[1.05] max-w-4xl text-white uppercase">
          SEU DISCORD,{" "}
          <span className="text-transparent bg-clip-text bg-gradient-to-r from-primary via-magenta-accent to-green-accent">
            SUAS REGRAS
          </span>
          .
        </h1>

        <p className="text-muted-ink text-lg md:text-xl max-w-2xl mt-6 leading-relaxed">
          Mostre exatamente o que você está fazendo no seu perfil do Discord.
          Suporte nativo para múltiplos programas, detecção automática e
          integração simples para qualquer software.
        </p>

        <div className="flex flex-col sm:flex-row gap-4 mt-10 justify-center w-full max-w-md">
          <a
            href="https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/releases"
            target="_blank"
            rel="noopener noreferrer"
            className="px-8 py-4 rounded-sm bg-primary text-white font-display font-extrabold text-md hover:bg-primary/95 transition-all text-center flex items-center justify-center gap-2 hover:scale-[1.02] shadow-lg"
          >
            💾 Baixar App para Desktop
          </a>
          <button
            onClick={onEnterApp}
            className="px-8 py-4 rounded-sm bg-surface-indigo border border-hairline text-white font-display font-extrabold text-md hover:bg-surface-indigo/80 hover:border-white/20 transition-all text-center"
          >
            ⚙️ Testar no Navegador
          </button>
        </div>

        <p className="text-xs text-muted-ink/60 mt-4">
          Compatível com Windows, macOS e Linux. Código limpo e sem anúncios.
        </p>
      </section>

      {/* Showcase Grid Section */}
      <section
        id="features"
        className="max-w-7xl mx-auto px-6 py-20 border-t border-hairline/20"
      >
        <div className="text-center mb-16">
          <h2 className="font-display font-extrabold text-3xl md:text-5xl text-white uppercase tracking-tight">
            Funcionalidades Incríveis
          </h2>
          <p className="text-muted-ink text-md max-w-xl mx-auto mt-3">
            Construído para desenvolvedores, gamers e aficionados por
            personalização.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {/* Card 1 */}
          <div className="bg-surface-indigo rounded-xl border border-hairline p-8 flex flex-col justify-between hover:border-primary/50 transition-all group">
            <div>
              <span className="text-3xl">🔌</span>
              <h3 className="font-display font-bold text-xl text-white mt-4 uppercase">
                Múltiplos Softwares
              </h3>
              <p className="text-muted-ink text-sm mt-2 leading-relaxed">
                Integre facilmente com VS Code, Spotify, Steam, navegadores e
                qualquer outro executável rodando em segundo plano.
              </p>
            </div>
            <div className="text-xs text-primary font-bold tracking-wider uppercase mt-6 group-hover:translate-x-1 transition-transform">
              Fácil de configurar →
            </div>
          </div>

          {/* Card 2 */}
          <div className="bg-gradient-to-br from-magenta-accent/25 to-surface-indigo/90 rounded-xl border border-magenta-accent/20 p-8 flex flex-col justify-between hover:border-magenta-accent/40 transition-all group">
            <div>
              <span className="text-3xl">🎨</span>
              <h3 className="font-display font-bold text-xl text-white mt-4 uppercase">
                Customização Nitro
              </h3>
              <p className="text-muted-ink text-sm mt-2 leading-relaxed">
                Crie cartões de presença de alto nível com gradientes
                personalizados, avatares dinâmicos e status ricos com timers e
                botões.
              </p>
            </div>
            <div className="text-xs text-magenta-accent font-bold tracking-wider uppercase mt-6 group-hover:translate-x-1 transition-transform">
              Sem precisar de Nitro →
            </div>
          </div>

          {/* Card 3 */}
          <div className="bg-surface-indigo rounded-xl border border-hairline p-8 flex flex-col justify-between hover:border-green-accent/50 transition-all group">
            <div>
              <span className="text-3xl">⚡</span>
              <h3 className="font-display font-bold text-xl text-white mt-4 uppercase">
                Detecção Inteligente
              </h3>
              <p className="text-muted-ink text-sm mt-2 leading-relaxed">
                O aplicativo detecta dinamicamente qual programa está em foco e
                atualiza sua atividade do Discord de forma inteligente.
              </p>
            </div>
            <div className="text-xs text-green-accent font-bold tracking-wider uppercase mt-6 group-hover:translate-x-1 transition-transform">
              100% automático →
            </div>
          </div>
        </div>
      </section>

      {/* Simulator (Live Demo) Section */}
      <section
        id="demo"
        className="max-w-7xl mx-auto px-6 py-20 border-t border-hairline/20 bg-surface-black/20 rounded-xl p-8 my-10"
      >
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div className="flex flex-col gap-6 text-left">
            <div className="inline-block bg-primary/10 border border-primary/20 text-primary font-bold px-3 py-1 rounded-sm text-xs uppercase tracking-wider w-fit">
              Simulador em Tempo Real
            </div>

            <h2 className="font-display font-extrabold text-3xl md:text-5xl text-white uppercase leading-[1.1]">
              Experimente antes de baixar
            </h2>

            <p className="text-muted-ink leading-relaxed">
              Tente personalizar o cartão de perfil ao lado. Mude o nome, o
              status e as cores e veja o seu status de presença se transformar
              instantaneamente como seria no Discord.
            </p>

            <div className="flex flex-col gap-4 bg-surface-indigo/40 p-5 rounded-md border border-hairline/60">
              {/* Display Name Input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Nome de Exibição
                </label>
                <input
                  type="text"
                  value={profile.displayName}
                  onChange={(e) =>
                    setProfile({ ...profile, displayName: e.target.value })
                  }
                  className="bg-surface-onyx border border-hairline/50 text-ink text-xs px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />
              </div>

              {/* Status input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Status Personalizado
                </label>
                <input
                  type="text"
                  value={profile.customStatus}
                  onChange={(e) =>
                    setProfile({ ...profile, customStatus: e.target.value })
                  }
                  className="bg-surface-onyx border border-hairline/50 text-ink text-xs px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />
              </div>

              {/* Software Select */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Software Ativo (Simulação)
                </label>
                <select
                  value={demoPresence.large_text}
                  onChange={(e) => {
                    const selected = softwares.find(
                      (s) => s.name === e.target.value,
                    );
                    setDemoPresence({
                      ...demoPresence,
                      large_text: e.target.value,
                      large_image: selected?.key || "auto",
                      details: `Usando o ${e.target.value}`,
                    });
                  }}
                  className="bg-surface-onyx border border-hairline/50 text-ink text-xs px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full cursor-pointer"
                >
                  {softwares.map((s) => (
                    <option key={s.name} value={s.name}>
                      {s.icon} {s.name}
                    </option>
                  ))}
                </select>
              </div>

              {/* Themes Selection */}
              <div className="flex flex-col gap-1.5 mt-2">
                <label className="text-[10px] text-muted-ink font-bold uppercase">
                  Cor do Perfil
                </label>
                <div className="flex gap-2.5">
                  <button
                    onClick={() =>
                      setProfile({
                        ...profile,
                        themePrimary: "#5865f2",
                        themeSecondary: "#ec48bd",
                      })
                    }
                    className={`w-6 h-6 rounded-full border bg-gradient-to-r from-[#5865f2] to-[#ec48bd] ${profile.themePrimary === "#5865f2" ? "border-white" : "border-transparent"}`}
                  />
                  <button
                    onClick={() =>
                      setProfile({
                        ...profile,
                        themePrimary: "#35ed7e",
                        themeSecondary: "#0a0d3a",
                      })
                    }
                    className={`w-6 h-6 rounded-full border bg-gradient-to-r from-[#35ed7e] to-[#0a0d3a] ${profile.themePrimary === "#35ed7e" ? "border-white" : "border-transparent"}`}
                  />
                  <button
                    onClick={() =>
                      setProfile({
                        ...profile,
                        themePrimary: "#ec48bd",
                        themeSecondary: "#2c327d",
                      })
                    }
                    className={`w-6 h-6 rounded-full border bg-gradient-to-r from-[#ec48bd] to-[#2c327d] ${profile.themePrimary === "#ec48bd" ? "border-white" : "border-transparent"}`}
                  />
                  <button
                    onClick={() =>
                      setProfile({
                        ...profile,
                        themePrimary: "#1e1f22",
                        themeSecondary: "#1e1f22",
                      })
                    }
                    className={`w-6 h-6 rounded-full border bg-[#1e1f22] ${profile.themePrimary === "#1e1f22" ? "border-white" : "border-transparent"}`}
                  />
                </div>
              </div>
            </div>
          </div>

          <div className="flex justify-center items-center">
            <PresenceCard presence={demoPresence} profile={profile} />
          </div>
        </div>
      </section>

      {/* Developer Contribution Section */}
      <section
        id="developers"
        className="max-w-7xl mx-auto px-6 py-20 border-t border-hairline/20"
      >
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div className="order-2 lg:order-1 flex flex-col gap-4">
            <div className="bg-surface-indigo rounded-lg border border-hairline p-6 font-mono text-xs text-left shadow-lg overflow-x-auto">
              <span className="text-magenta-accent">
                // Como criar uma regra para o seu software em segundos:
              </span>
              <pre className="text-muted-ink mt-3">
                {`{
  "name": "Meu Editor Incrível",
  "processName": "my-editor",
  "rules": [
    {
      "windowTitleRegex": "^Editing - (.*)",
      "details": "Desenvolvendo código",
      "state": "Arquivo: $1"
    }
  ]
}`}
              </pre>
            </div>
          </div>

          <div className="order-1 lg:order-2 flex flex-col gap-6 text-left">
            <div className="inline-block bg-green-accent/15 border border-green-accent/30 text-green-accent font-bold px-3 py-1 rounded-sm text-xs uppercase tracking-wider w-fit">
              Construído para Desenvolvedores
            </div>

            <h2 className="font-display font-extrabold text-3xl md:text-5xl text-white uppercase leading-[1.1]">
              Ajude a expandir o projeto!
            </h2>

            <p className="text-muted-ink leading-relaxed">
              O Better RPC é open-source e focado na comunidade. Se você é um
              desenvolvedor ou entusiasta, você pode nos ajudar adicionando
              regras de detecção para novos softwares, melhorando a interface
              Tauri em React, ou otimizando a engine de detecção em Rust.
            </p>

            <div className="flex flex-col gap-3 mt-2 text-sm">
              <div className="flex items-center gap-2">
                <span className="text-green-accent">✔</span>
                <span>Fácil contribuição em JSON para regras de softwares</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-green-accent">✔</span>
                <span>Arquitetura modular em TypeScript & React</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-green-accent">✔</span>
                <span>Engine leve baseada em Rust e Tauri</span>
              </div>
            </div>

            <div className="mt-4">
              <a
                href="https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-primary to-magenta-accent rounded-sm font-display font-extrabold text-sm uppercase hover:opacity-90 transition-all shadow-md"
              >
                ⭐ Contribuir no GitHub
              </a>
            </div>
          </div>
        </div>
      </section>

      {/* FAQ Section */}
      <section
        id="faq"
        className="max-w-4xl mx-auto px-6 py-20 border-t border-hairline/20"
      >
        <div className="text-center mb-12">
          <h2 className="font-display font-extrabold text-3xl md:text-4xl text-white uppercase">
            Perguntas Frequentes
          </h2>
        </div>

        <div className="flex flex-col gap-4">
          {[
            {
              q: "Preciso pagar pelo Discord Nitro para usar o Better RPC?",
              a: "Não! O Better RPC funciona na conta padrão gratuita do Discord. Você pode customizar gradientes, avatares e mensagens sem precisar de qualquer assinatura.",
            },
            {
              q: "O aplicativo é seguro para usar?",
              a: "Totalmente. O Better RPC é open-source, todo o código está aberto no GitHub para auditoria pública, e não coleta dados pessoais nem envia informações para servidores externos.",
            },
            {
              q: "Como posso adicionar um programa customizado?",
              a: "Você pode adicionar qualquer programa informando o nome do executável e configurando regras simples via expressões regulares no painel de configurações.",
            },
            {
              q: "Como funciona a detecção em segundo plano?",
              a: "O aplicativo monitora levemente os processos ativos do sistema operacional e compara os nomes com a biblioteca de softwares registrada para decidir qual presença mostrar.",
            },
          ].map((item, index) => (
            <div
              key={index}
              className="bg-surface-indigo rounded-lg border border-hairline overflow-hidden cursor-pointer transition-colors hover:border-white/20"
              onClick={() => setFaqOpen(faqOpen === index ? null : index)}
            >
              <button className="w-full flex justify-between items-center p-5 font-display font-bold text-left text-white text-md">
                <span>{item.q}</span>
                <span className="text-primary">
                  {faqOpen === index ? "▲" : "▼"}
                </span>
              </button>
              {faqOpen === index && (
                <div className="px-5 pb-5 text-sm text-muted-ink leading-relaxed border-t border-hairline/30 pt-3">
                  {item.a}
                </div>
              )}
            </div>
          ))}
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-surface-black/30 border-t border-hairline/40 py-12 relative overflow-hidden">
        <div className="max-w-7xl mx-auto px-6 flex flex-col md:flex-row justify-between items-center gap-6 relative z-10">
          <div className="flex items-center gap-2.5">
            <img
              src="/logo.png"
              alt="Better RPC Logo"
              className="w-6 h-5 object-contain"
            />
            <span className="font-display font-extrabold text-sm tracking-tight text-white">
              Better RPC
            </span>
          </div>

          <p className="text-xs text-muted-ink">
            © {new Date().getFullYear()} Better Rich Presence For Discord.
            Projeto Open Source sob Licença MIT.
          </p>

          <div className="flex gap-4 text-xs font-bold">
            <a
              href="https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord"
              target="_blank"
              rel="noopener noreferrer"
              className="text-muted-ink hover:text-white transition-colors"
            >
              GitHub
            </a>
            <span className="text-hairline">|</span>
            <a
              href="#features"
              className="text-muted-ink hover:text-white transition-colors"
            >
              Funcionalidades
            </a>
          </div>
        </div>

        {/* Large Decorative Wordmark */}
        <div className="absolute bottom-[-50px] left-1/2 -translate-x-1/2 font-display font-black text-[120px] md:text-[200px] text-white/[0.02] tracking-tighter uppercase select-none pointer-events-none w-full text-center">
          BETTER RPC
        </div>
      </footer>
    </div>
  );
}
