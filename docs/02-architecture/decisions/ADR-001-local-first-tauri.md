# ADR-001: Aplicação desktop local-first

- **Estado:** Aceita pelo escopo do PRD
- **Contexto:** produto Windows 10/11, offline, sem login/sync na versão inicial.
- **Decisão:** Vue 3/TypeScript no WebView Tauri 2; regras e persistência local em Rust; SQLite via SQLx; comandos Tauri como fronteira interna. Todas as operações SQLite do backend Rust utilizam exclusivamente SQLx; o frontend não acessa o banco diretamente.
- **Consequências:** sem serviço remoto obrigatório; backup/restore e migrations são responsabilidade local; recursos de Windows exigem validação em ambiente real.
- **Alternativas:** aplicação web remota ou banco no frontend, incompatíveis com os requisitos explícitos de desktop/local-first e separação da lógica sensível.

