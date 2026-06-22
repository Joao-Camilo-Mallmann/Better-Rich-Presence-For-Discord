import { useState } from "react";
import { PresenceCard } from "../../components/PresenceCard/PresenceCard";
import { DiscordProfile, PresenceData } from "../../types";

export function LandingPage() {
  // Live Demo customization state
  const [profile, setProfile] = useState<DiscordProfile>({
    displayName: "You on Discord",
    username: "your_username",
    avatarUrl: "",
    customStatus: "🚀 Tweak my RPC status!",
    status: "online",
    themePrimary: "#5865f2",
    themeSecondary: "#ec48bd",
    isGradient: true,
  });

  const [demoPresence, setDemoPresence] = useState<PresenceData>({
    source: "Manual",
    large_text: "Visual Studio Code",
    details: "Editing src/App.tsx",
    state: "Line 42: Column 12",
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
          <img src="/logo.png" alt="Better RPC Logo" className="w-8 h-6 object-contain" />
          <span className="font-display font-extrabold text-lg tracking-tight">Better RPC</span>
        </div>
        <nav className="hidden md:flex items-center gap-8 text-sm font-semibold text-muted-ink">
          <a href="#features" className="hover:text-ink transition-colors">Features</a>
          <a href="#demo" className="hover:text-ink transition-colors">Simulator</a>
          <a href="#developers" className="hover:text-ink transition-colors">Developers</a>
          <a href="#faq" className="hover:text-ink transition-colors">FAQ</a>
        </nav>
        <div className="flex items-center gap-3">
          <a 
            href="https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord" 
            target="_blank" 
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 px-4 py-2 rounded-sm border border-hairline bg-surface-indigo/40 hover:bg-surface-indigo text-xs font-bold transition-all"
          >
            ⭐ View on GitHub
          </a>
        </div>
      </header>

      {/* Hero Section */}
      <section className="relative z-10 max-w-6xl mx-auto px-6 pt-16 pb-20 text-center flex flex-col items-center">
        <div className="inline-flex items-center gap-2 bg-magenta-accent/15 border border-magenta-accent/30 text-magenta-accent px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider mb-6 animate-pulse">
          ✨ 100% Open Source & Customizable
        </div>
        
        <h1 className="font-display font-extrabold text-5xl md:text-7xl tracking-tighter leading-[1.05] max-w-4xl text-white uppercase">
          YOUR DISCORD, <span className="text-transparent bg-clip-text bg-gradient-to-r from-primary via-magenta-accent to-green-accent">YOUR RULES</span>.
        </h1>
        
        <p className="text-muted-ink text-lg md:text-xl max-w-2xl mt-6 leading-relaxed">
          Show exactly what you are doing in your Discord profile. Native support for multiple programs, automatic detection, and straightforward integrations for any software.
        </p>

        <div className="flex flex-col sm:flex-row gap-4 mt-10 justify-center w-full max-w-md">
          <a 
            href="https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/releases" 
            target="_blank" 
            rel="noopener noreferrer"
            className="px-8 py-4 rounded-sm bg-primary text-white font-display font-extrabold text-md hover:bg-primary/95 transition-all text-center flex items-center justify-center gap-2 hover:scale-[1.02] shadow-lg w-full"
          >
            💾 Download Desktop App
          </a>
        </div>

        <p className="text-xs text-muted-ink/60 mt-4">
          Compatible with Windows, macOS, and Linux. Safe, clean, and ad-free.
        </p>
      </section>

      {/* Showcase Grid Section */}
      <section id="features" className="max-w-7xl mx-auto px-6 py-20 border-t border-hairline/20">
        <div className="text-center mb-16">
          <h2 className="font-display font-extrabold text-3xl md:text-5xl text-white uppercase tracking-tight">
            Awesome Features
          </h2>
          <p className="text-muted-ink text-md max-w-xl mx-auto mt-3">
            Built for developers, gamers, and customization enthusiasts.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {/* Card 1 */}
          <div className="bg-surface-indigo rounded-xl border border-hairline p-8 flex flex-col justify-between hover:border-primary/50 transition-all group">
            <div>
              <span className="text-3xl">🔌</span>
              <h3 className="font-display font-bold text-xl text-white mt-4 uppercase">Multiple Software</h3>
              <p className="text-muted-ink text-sm mt-2 leading-relaxed">
                Seamlessly integrate with VS Code, Spotify, Steam, web browsers, and any other executable running on your machine.
              </p>
            </div>
            <div className="text-xs text-primary font-bold tracking-wider uppercase mt-6 group-hover:translate-x-1 transition-transform">
              Easy configuration →
            </div>
          </div>

          {/* Card 2 */}
          <div className="bg-gradient-to-br from-magenta-accent/25 to-surface-indigo/90 rounded-xl border border-magenta-accent/20 p-8 flex flex-col justify-between hover:border-magenta-accent/40 transition-all group">
            <div>
              <span className="text-3xl">🎨</span>
              <h3 className="font-display font-bold text-xl text-white mt-4 uppercase">Nitro Customization</h3>
              <p className="text-muted-ink text-sm mt-2 leading-relaxed">
                Create high-fidelity profile presence status cards with custom gradients, dynamic avatars, active timers, and rich button actions.
              </p>
            </div>
            <div className="text-xs text-magenta-accent font-bold tracking-wider uppercase mt-6 group-hover:translate-x-1 transition-transform">
              No Nitro required →
            </div>
          </div>

          {/* Card 3 */}
          <div className="bg-surface-indigo rounded-xl border border-hairline p-8 flex flex-col justify-between hover:border-green-accent/50 transition-all group">
            <div>
              <span className="text-3xl">⚡</span>
              <h3 className="font-display font-bold text-xl text-white mt-4 uppercase">Smart Detection</h3>
              <p className="text-muted-ink text-sm mt-2 leading-relaxed">
                The desktop client automatically detects which application is in active focus and updates your Discord status instantly.
              </p>
            </div>
            <div className="text-xs text-green-accent font-bold tracking-wider uppercase mt-6 group-hover:translate-x-1 transition-transform">
              100% automated →
            </div>
          </div>
        </div>
      </section>

      {/* Simulator (Live Demo) Section */}
      <section id="demo" className="max-w-7xl mx-auto px-6 py-20 border-t border-hairline/20 bg-surface-black/20 rounded-xl p-8 my-10">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          
          <div className="flex flex-col gap-6 text-left">
            <div className="inline-block bg-primary/10 border border-primary/20 text-primary font-bold px-3 py-1 rounded-sm text-xs uppercase tracking-wider w-fit">
              Real-time Simulator
            </div>
            
            <h2 className="font-display font-extrabold text-3xl md:text-5xl text-white uppercase leading-[1.1]">
              Try it before downloading
            </h2>
            
            <p className="text-muted-ink leading-relaxed">
              Tweak the profile customization controls below. Change your name, status, active software, and colors to see your Discord profile change instantly.
            </p>

            <div className="flex flex-col gap-4 bg-surface-indigo/40 p-5 rounded-md border border-hairline/60">
              {/* Display Name Input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">Display Name</label>
                <input 
                  type="text"
                  value={profile.displayName}
                  onChange={(e) => setProfile({ ...profile, displayName: e.target.value })}
                  className="bg-surface-onyx border border-hairline/50 text-ink text-xs px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />
              </div>

              {/* Status input */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">Custom Status</label>
                <input 
                  type="text"
                  value={profile.customStatus}
                  onChange={(e) => setProfile({ ...profile, customStatus: e.target.value })}
                  className="bg-surface-onyx border border-hairline/50 text-ink text-xs px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full"
                />
              </div>

              {/* Software Select */}
              <div className="flex flex-col gap-1">
                <label className="text-[10px] text-muted-ink font-bold uppercase">Active Software (Simulation)</label>
                <select 
                  value={demoPresence.large_text}
                  onChange={(e) => {
                    const selected = softwares.find(s => s.name === e.target.value);
                    setDemoPresence({
                      ...demoPresence,
                      large_text: e.target.value,
                      large_image: selected?.key || "auto",
                      details: `Using ${e.target.value}`,
                    });
                  }}
                  className="bg-surface-onyx border border-hairline/50 text-ink text-xs px-3 py-2 rounded-xs focus:border-primary focus:outline-none w-full cursor-pointer"
                >
                  {softwares.map((s) => (
                    <option key={s.name} value={s.name}>{s.icon} {s.name}</option>
                  ))}
                </select>
              </div>

              {/* Themes Selection */}
              <div className="flex flex-col gap-1.5 mt-2">
                <label className="text-[10px] text-muted-ink font-bold uppercase">Profile Theme Color</label>
                <div className="flex gap-2.5">
                  <button 
                    onClick={() => setProfile({ ...profile, themePrimary: "#5865f2", themeSecondary: "#ec48bd" })} 
                    className={`w-6 h-6 rounded-full border bg-gradient-to-r from-[#5865f2] to-[#ec48bd] ${profile.themePrimary === "#5865f2" ? "border-white" : "border-transparent"}`}
                  />
                  <button 
                    onClick={() => setProfile({ ...profile, themePrimary: "#35ed7e", themeSecondary: "#0a0d3a" })} 
                    className={`w-6 h-6 rounded-full border bg-gradient-to-r from-[#35ed7e] to-[#0a0d3a] ${profile.themePrimary === "#35ed7e" ? "border-white" : "border-transparent"}`}
                  />
                  <button 
                    onClick={() => setProfile({ ...profile, themePrimary: "#ec48bd", themeSecondary: "#2c327d" })} 
                    className={`w-6 h-6 rounded-full border bg-gradient-to-r from-[#ec48bd] to-[#2c327d] ${profile.themePrimary === "#ec48bd" ? "border-white" : "border-transparent"}`}
                  />
                  <button 
                    onClick={() => setProfile({ ...profile, themePrimary: "#1e1f22", themeSecondary: "#1e1f22" })} 
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
      <section id="developers" className="max-w-7xl mx-auto px-6 py-20 border-t border-hairline/20">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          
          <div className="order-2 lg:order-1 flex flex-col gap-4">
            <div className="bg-surface-indigo rounded-lg border border-hairline p-6 font-mono text-xs text-left shadow-lg overflow-x-auto">
              <span className="text-magenta-accent">// Build a rule for your custom app in seconds:</span>
              <pre className="text-muted-ink mt-3">
{`{
  "name": "My Custom Editor",
  "processName": "my-editor",
  "rules": [
    {
      "windowTitleRegex": "^Editing - (.*)",
      "details": "Developing code",
      "state": "File: $1"
    }
  ]
}`}
              </pre>
            </div>
          </div>

          <div className="order-1 lg:order-2 flex flex-col gap-6 text-left">
            <div className="inline-block bg-green-accent/15 border border-green-accent/30 text-green-accent font-bold px-3 py-1 rounded-sm text-xs uppercase tracking-wider w-fit">
              Built for Developers
            </div>
            
            <h2 className="font-display font-extrabold text-3xl md:text-5xl text-white uppercase leading-[1.1]">
              Help us expand the library!
            </h2>
            
            <p className="text-muted-ink leading-relaxed">
              Better RPC is open-source and community-driven. If you are a developer or enthusiast, you can help us by adding detection rules for new applications, improving the React + Tauri desktop user interface, or optimizing our lightweight Rust-based presence engine.
            </p>

            <div className="flex flex-col gap-3 mt-2 text-sm">
              <div className="flex items-center gap-2">
                <span className="text-green-accent">✔</span>
                <span>Simple JSON configurations for process matching</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-green-accent">✔</span>
                <span>Modular codebase using TypeScript, React & Tailwind CSS</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-green-accent">✔</span>
                <span>Blazing fast engine powered by Tauri & Rust</span>
              </div>
            </div>

            <div className="mt-4">
              <a 
                href="https://github.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-primary to-magenta-accent rounded-sm font-display font-extrabold text-sm uppercase hover:opacity-90 transition-all shadow-md"
              >
                ⭐ Contribute on GitHub
              </a>
            </div>
          </div>

        </div>
      </section>

      {/* FAQ Section */}
      <section id="faq" className="max-w-4xl mx-auto px-6 py-20 border-t border-hairline/20">
        <div className="text-center mb-12">
          <h2 className="font-display font-extrabold text-3xl md:text-4xl text-white uppercase">
            Frequently Asked Questions
          </h2>
        </div>

        <div className="flex flex-col gap-4">
          {[
            {
              q: "Do I need Discord Nitro to use Better RPC?",
              a: "Nope! Better RPC works perfectly fine on standard, free Discord accounts. You can customize profile gradients, status strings, avatars, and timers without paying for any subscription."
            },
            {
              q: "Is this application safe to use?",
              a: "Absolutely. Better RPC is 100% open-source, and all the code is publicly reviewable on GitHub. It operates locally on your machine and never transmits personal data to external servers."
            },
            {
              q: "How can I add custom rules for unsupported apps?",
              a: "You can easily add rules for any application by providing the executable process name and mapping details via regular expressions inside the advanced settings menu."
            },
            {
              q: "How does background process detection work?",
              a: "The client queries active operating system processes in a lightweight loop, compares them with our registered software database, and dynamically publishes your active status to Discord."
            }
          ].map((item, index) => (
            <div 
              key={index}
              className="bg-surface-indigo rounded-lg border border-hairline overflow-hidden cursor-pointer transition-colors hover:border-white/20"
              onClick={() => setFaqOpen(faqOpen === index ? null : index)}
            >
              <button className="w-full flex justify-between items-center p-5 font-display font-bold text-left text-white text-md">
                <span>{item.q}</span>
                <span className="text-primary">{faqOpen === index ? "▲" : "▼"}</span>
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
            <img src="/logo.png" alt="Better RPC Logo" className="w-6 h-5 object-contain" />
            <span className="font-display font-extrabold text-sm tracking-tight text-white">Better RPC</span>
          </div>
          
          <p className="text-xs text-muted-ink">
            © {new Date().getFullYear()} Better Rich Presence For Discord. Open-source software licensed under MIT.
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
            <a href="#features" className="text-muted-ink hover:text-white transition-colors">Features</a>
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
