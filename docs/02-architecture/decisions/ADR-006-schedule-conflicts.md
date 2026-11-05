# ADR-006: Detecção e confirmação de conflitos

- **Estado:** Aceita
- **Contexto:** o sistema deve identificar conflitos no mesmo cronograma, mas permitir agendas flexíveis.
- **Decisão:** detectar sobreposição temporal entre ocorrências ativas no mesmo cronograma. Intervalos adjacentes não conflitam; cronogramas diferentes não são comparados. Atividades canceladas e ocorrências históricas concluídas não bloqueiam novos horários. Exibir aviso e exigir confirmação explícita para salvar uma alteração conflitante.
- **Consequências:** validação transacional no backend; a verificação deve ser refeita no momento da gravação. SQLite não tem constraint de exclusão de intervalos, então a regra não depende apenas do schema.
- **Impacto:** Fase 2; `US-009` a `US-012`.

