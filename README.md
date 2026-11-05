# Life Planner

Aplicativo desktop local-first para Windows, construído com Tauri 2, Vue 3, TypeScript e Vite.

## Requisitos de desenvolvimento

- Node.js compatível com Vite 8 (Node.js 20.19+ ou 22.12+).
- pnpm 12.16.1, ativado por Corepack.
- Rust stable e Cargo.
- Dependências nativas de Tauri para a plataforma usada. Consulte o [guia oficial de pré-requisitos](https://v2.tauri.app/start/prerequisites/).

O projeto usa somente pnpm para o frontend. Mantenha `pnpm-lock.yaml` versionado. O backend Rust usa Cargo e `Cargo.lock` versionado.

## Comandos

```sh
corepack pnpm install
corepack pnpm tauri dev
corepack pnpm build
corepack pnpm tauri build
```

`tauri build` compila o aplicativo; a configuração atual não ativa geração de instalador. A configuração de release/NSIS pertence à US-029.

O bootstrap não carrega recursos de rede, não registra comandos Tauri nem concede permissões de plugin à janela.

Falhas de montagem da interface aparecem na própria janela e os diagnósticos frontend são enviados ao console técnico. Falhas fatais de inicialização do backend Rust são registradas em `Life Planner/logs/startup.log` no diretório de estado local do usuário; no Windows, também exibem uma mensagem nativa sem detalhes técnicos.
