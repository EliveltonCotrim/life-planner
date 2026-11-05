# ADR-002: Categorias globais

- **Estado:** Aceita
- **Contexto:** categorias classificam atividades, tarefas e revisões; o produto possui múltiplos cronogramas e tarefas globais ou opcionalmente vinculadas a cronogramas (ADR-011).
- **Decisão:** categorias são globais e compartilhadas entre todos os cronogramas. Atividades, tarefas e revisões podem referenciá-las. Categorias inativas não podem ser usadas em novos registros; histórico mantém as informações de categoria. Exclusão definitiva só é permitida sem referências.
- **Consequências:** `categories` não possui `schedule_id`; FK restringe exclusão referenciada; listagens de histórico incluem categorias inativas. Ocorrências e registros históricos preservam snapshot de nome, cor e ícone usados à época, além da referência quando válida.
- **Alternativas consideradas:** categorias por cronograma e categorias globais copiadas ao duplicar cronograma. Rejeitadas por fragmentar classificação e complicar tarefas/revisões globais.
- **Impacto:** Fase 2; `US-008`, `US-009`, `US-013`, `US-022`, `US-024`.

