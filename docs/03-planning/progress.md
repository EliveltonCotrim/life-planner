# Progresso do projeto

**Atualizado:** 2026-11-05  
**Etapa atual:** Fase 1 — bootstrap desktop (US-001).
**Implementação de produto:** US-001 em andamento; demais histórias não iniciadas.

## Documentação produzida

- Requisitos normalizados e rastreados ao PRD.
- Arquitetura de frontend/backend e fronteiras de responsabilidade.
- Modelo lógico SQLite atualizado para séries, ocorrências, timer, notificações e restauração.
- Registro de ADR-001 a ADR-011 como aceitos.
- Roadmap e backlog de implementação.
- Bootstrap Tauri 2/Vue 3/Vite da US-001 em implementação; sem comandos de domínio ou persistência.

## Contagem

- Épicos: **6** (`EP-01` a `EP-06`).
- Histórias: **34** (`US-001` a `US-034`).
- Distribuição por fase: **6 / 8 / 7 / 4 / 4 / 5**; total **34**. As seis fases originais foram preservadas.
- Histórias iniciadas: **1** (`US-001`).
- Histórias concluídas: **0**; validação do frontend realizada, verificações Tauri/Rust e Windows ainda pendentes.

## Estado de decisões

Decisões arquiteturais e regras do pedido de consolidação registradas. Refinamentos técnicos restantes: enumerações finais de status/prioridade/tipo de série; configuração Pomodoro por ciclo/sessão e vínculo à atividade; estratégia interna de materialização das ocorrências (sujeita à chave e regras aceitas); mecanismo de exportação PDF; metas de acessibilidade, desempenho e volume histórico. Métricas, lembretes, distribuição, recorrência e restauração têm critérios de produto definidos.

## Verificações da US-001

- `corepack pnpm build`: passou (typecheck Vue/TypeScript e build de produção Vite).
- `corepack pnpm install --frozen-lockfile --offline`: passou; `pnpm-lock.yaml` está sincronizado e não exige downloads para reinstalação a partir do cache.
- `node --check` no monitor de inicialização frontend: passou.
- Smoke HTTP do servidor Vite: página inicial serviu conteúdo em português e fallback visível.
- `cargo fmt --check` e `cargo metadata --locked --no-deps`: passaram.
- `cargo check --locked`: bloqueado porque o ambiente WSL não possui linker C (`cc`); também faltam `webkit2gtk-4.1` e `rsvg2`.
- Execução Tauri e validação em Windows 10 22H2/Windows 11 x64: pendentes; ambiente disponível é Ubuntu 24.04 em WSL2.

## Próximo passo

Concluir validação Tauri/Rust em ambiente com os pré-requisitos nativos; validar execução em Windows 10 22H2 e Windows 11 x64. US-002 só começa após autorização própria.

