# ADR-011: Tarefas globais com cronograma opcional

- **Estado:** Aceita
- **Contexto:** o PRD descrevia tarefas como módulo independente, mas a relação inicial do banco dizia que cada cronograma possui tarefas. É necessário permitir listas gerais e, quando desejado, tarefas associadas à rotina.
- **Decisão:** tarefas são globais e podem existir sem cronograma. Uma tarefa pode ter vínculo opcional com um cronograma. A categoria também é global e opcional, conforme ADR-002.
- **Recorrência:** a série herda o vínculo opcional; ocorrências preservam o cronograma associado no momento em que foram planejadas. Alterar vínculo aplica-se às ocorrências futuras segundo ADR-003; histórico passado mantém snapshots.
- **Consequências:** o vínculo opcional fica no segmento da série (`task_series_segments.schedule_id` nullable), permitindo alterá-lo para ocorrências futuras sem modificar histórico. `task_occurrences` preserva o vínculo em `schedule_id_snapshot`. Listas globais incluem tarefas vinculadas e não vinculadas; filtro por cronograma seleciona as vinculadas.
- **Impacto:** Fase 3; `US-013` a `US-015`, `US-021`, `US-023` e relatórios.
