# Rules and Guidelines for Better Rich Presence for Discord Agent

Este arquivo contém as diretrizes gerais para o desenvolvimento do projeto. Para evitar arquivos gigantes, as regras específicas foram modularizadas na pasta `docs/`.

## 📌 Diretrizes Detalhadas (Arquivos de Referência)
- **[Design & Styling](file:///home/joao/projects/Better-Rich-Presence-For-Discord/docs/design_guidelines.md)**: Padrões visuais de cores, tipografia (Space Grotesk e Inter) e interações dinâmicas.
- **[Architecture & Engine](file:///home/joao/projects/Better-Rich-Presence-For-Discord/docs/architecture_rules.md)**: Prioridade das fontes (`Game > Manual > Work > Browser > Idle`), watchers do sistema operacional, timings e lógica de persistência.
- **[Web & SEO](file:///home/joao/projects/Better-Rich-Presence-For-Discord/docs/web_seo_guidelines.md)**: Configurações de SEO ("SEO brabo"), tags Open Graph para compartilhamento e regras de deploy na Vercel.

## 🧱 Organização do Código
- Mantenha componentes reutilizáveis e focados em `src/components/`.
- Mantenha a estrutura de páginas (ex: Dashboard, Apps, LandingPage) em `src/pages/`.
- Utilize Tailwind CSS (v4) com variáveis `@theme` estruturadas em [globals.css](file:///home/joao/projects/Better-Rich-Presence-For-Discord/src/globals.css).
