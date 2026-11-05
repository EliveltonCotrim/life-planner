# Roadmap de desenvolvimento

As seis fases e seus objetivos permanecem os do PRD. Decisões ADR-002 a ADR-011 estão aceitas; as histórias US-031–US-034 detalham o escopo decidido dentro das mesmas fases.

| Fase | Resultado verificável | Pré-requisitos/saída |
|---|---|---|
| 1 — Fundação | Aplicação Tauri/Vue inicia; layout, navegação, tema claro/escuro/sistema, preferências persistentes e SQLite via SQLx. | Local de dados via Tauri; todas operações SQLite pelo SQLx; português fixo, sem horários de funcionamento na v1.0. |
| 2 — Organização da rotina | Cronogramas e categorias globais; atividades pontuais/recorrentes; identidade/split de série; agenda semanal, carga/horários livres, cópia/drag-and-drop, conflitos com confirmação e arquivamento seguro. | Resolver fuso/DST pelo ADR-005; US-031 cobre edição de série de atividade. |
| 3 — Tarefas e notificações | Tarefas globais com cronograma opcional, séries e ocorrências, listas/filtros/reagendamento; lembretes, bandeja/startup e reconciliação após suspensão. | Reutilizar identidade de recorrência; US-034 cobre edição de série de tarefa; janela 30 min/expiração/agrupamento do ADR-004. |
| 4 — Controle de tempo | Cronômetro recuperável, Pomodoro, sessões e histórico; métricas separadas de tempo registrado e planejado concluído. | Persistir pausas/checkpoints; recuperação interrompida; métricas conforme denominadores de ADR-007. |
| 5 — Produtividade | Dashboard com campos previstos, fórmulas centralizadas, revisões, relatórios e PDF. | Reutilizar agregadores backend; português fixo; horas de funcionamento fora da v1.0. |
| 6 — Finalização | Backups manual/automático, restore seguro, testes de integração/UI, otimização, instalador NSIS e documentação. | Diário pela abertura desde último automático válido; 7 automáticos; testar WAL, rollback, migrations e Windows alvo. |

## Marcos de saída sugeridos

- Cada fase encerra com critérios de aceite de suas histórias satisfeitos e migrations verificadas.
- Validar comportamento Windows (notificações/tray/startup/instalador) antes de fechar as fases 3 e 6.
- Gerar instalador somente após restore, migração e teste de regressão em dados de teste.
- Testar mudança de fuso Windows, transições DST, suspensão/retomada e interrupção de processo segundo regras aceitas antes do release.

