# ADR-008: Histórico, arquivamento e exclusão

- **Estado:** Aceita
- **Contexto:** registros com ocorrências, sessões ou outras referências precisam manter integridade e histórico.
- **Decisão:** arquivar entidades que possuam histórico ou dependências; preservar ocorrências passadas. Permitir exclusão definitiva somente quando não houver dependências. Aplicar restrições referenciais e não usar exclusão em cascata indiscriminadamente.
- **Consequências:** operações de remoção consultam referências; estados arquivados são omitidos de seletores de novos registros, mas podem ser consultados no histórico. Exclusão sem referências pode ser física.
- **Impacto:** Fases 2–6; histórias de cronogramas, categorias, atividades, tarefas, revisões e backups.

