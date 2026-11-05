# ADR-007: Fórmulas de produtividade

- **Estado:** Aceita
- **Contexto:** dashboard e relatórios precisam distinguir execução registrada de planejamento concluído.
- **Decisão:** calcular e apresentar separadamente:
  1. **Tempo registrado:** soma da duração de sessões de cronômetro concluídas, descontando pausas.
  2. **Tempo planejado concluído:** soma da duração planejada de atividades marcadas como realizadas.
  3. **Cumprimento da agenda:** tempo planejado concluído dividido pelo tempo planejado elegível.
  4. **Cumprimento de tarefas:** tarefas concluídas divididas pelas tarefas previstas.
- **Regras:** não somar tempo registrado ao planejado concluído. Fórmulas residem no backend e são reutilizadas por dashboard e relatórios. Durações nulas ou denominador zero devem produzir estado não calculável/sem dados, nunca divisão por zero.
- **Elegibilidade:** períodos são intervalos de datas civis locais, início inclusivo e fim exclusivo. Para agenda, entram no denominador ocorrências não canceladas cuja hora de início não seja futura em relação ao instante de referência; em período histórico, todas as ocorrências não canceladas do período são elegíveis; em período futuro, nenhuma é elegível. Numerador contém somente ocorrências elegíveis marcadas como realizadas. Duração planejada é `ends_at_utc - starts_at_utc`.
- **Tarefas:** denominador é o número de ocorrências de tarefa com data prevista dentro do período selecionado, uma unidade por ocorrência; tarefas sem data ficam fora e são mostradas separadamente. Numerador conta as ocorrências elegíveis concluídas. Séries recorrentes contam uma ocorrência por data prevista. Cancelamentos são excluídos. Ocorrências futuras de tarefas com data no período permanecem previstas para a métrica.
- **Período corrente:** agenda exclui atividades que ainda não começaram. Para tarefas, tarefas datadas dentro do período são previstas mesmo que o prazo/hora ainda não tenha chegado; a métrica retrata cumprimento do plano do período, não só tarefas vencidas.
- **Sessões:** cada sessão concluída é atribuída integralmente ao período que contém sua data local de início; pausas são descontadas e a duração não é repartida entre períodos.
- **Consequências:** denominadores são derivados de ocorrências, nunca das definições de séries; os mesmos agregadores Rust alimentam dashboard e relatórios. Denominador zero é `sem dados/não calculável`.
- **Impacto:** Fases 4 e 5; `US-020` a `US-025`.

