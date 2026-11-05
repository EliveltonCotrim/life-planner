# ADR-004: Notificações e aplicativo na bandeja

- **Estado:** Aceita
- **Contexto:** lembretes precisam funcionar com a janela fechada, e o PRD prevê bandeja do sistema e inicialização opcional.
- **Decisão:** fechar a janela mantém o processo ativo na bandeja do Windows. Notificações são entregues enquanto o processo estiver ativo. Sair explicitamente encerra o processo e interrompe futuras notificações. Inicialização automática com Windows é opcional. Não integrar ao Agendador de Tarefas do Windows na v1.0.
- **Ciclo de vida:** estados persistidos: `pending`, `delivered`, `expired`, `cancelled`. Ao retomar do modo suspenso, notificar individualmente lembretes vencidos há até 30 minutos; os mais antigos passam a `expired`, permanecem consultáveis e não geram toast individual. Tarefas relacionadas continuam pendentes/atrasadas conforme sua própria ocorrência. Uma ocorrência vencida só pode ser entregue uma vez.
- **Agrupamento:** em uma reconciliação com vários lembretes semelhantes, agrupar notificações do mesmo tipo de alvo (atividade, tarefa ou revisão) em um resumo nativo com contagem e títulos; manter cada lembrete individual e seu estado no app.
- **Consequências:** persistir vencimento, entrega, expiração e chave de agrupamento; scheduler no processo e reconciliação no resume; controles de bandeja; não prometer entrega após saída explícita.
- **Impacto:** Fase 3; `US-016`, `US-017`, `US-032` e `US-034`; Pomodoro `US-019` também usa notificações.

