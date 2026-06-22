# Web Deploy and SEO Guidelines

## 🌐 Deploy na Vercel
- Configure o comportamento de rotas SPA no arquivo `vercel.json` redirecionando todas as requisições que não encontrem arquivos físicos diretamente para o `index.html`.
- Garanta que o bundle final estático gerado por `npm run build` não contenha erros de compilação ou assets ausentes.

## 🔍 Otimizações de SEO ("SEO Brabo")
- **Meta Tags:** Adicione títulos, meta descriptions atraentes com palavras-chave relevantes (como `Discord Rich Presence`, `Custom RPC`, `custom status`).
- **Open Graph (OG):** Mantenha tags de OG (`og:title`, `og:description`, `og:image`) configuradas corretamente para gerar pré-visualizações ricas ao compartilhar o site no Discord e redes sociais.
- **Structured Data:** Use o formato JSON-LD para declarar a aplicação no esquema `SoftwareApplication`, facilitando Rich Snippets na busca do Google.
- **Indexadores:** Certifique-se de que os arquivos `robots.txt` e `sitemap.xml` estejam no diretório `public/` e apontando para a URL de produção correta.
