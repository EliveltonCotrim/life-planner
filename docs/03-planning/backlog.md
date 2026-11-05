# Backlog executável

IDs estáveis: épicos `EP-01`…`EP-06`, histórias `US-001`…`US-034`. Prioridade: `P0` fundação/bloqueante, `P1` núcleo, `P2` extensão da primeira versão. Estimativas são pontos relativos, sem conversão para dias.

## Fase 1 — Fundação

### EP-01 — Base desktop e persistência
**Objetivo:** estabelecer aplicação desktop local, navegação e armazenamento confiável.

#### US-001 — Iniciar o Life Planner no Windows
- **História:** Como usuário, quero abrir o aplicativo desktop para acessar meu espaço de organização local.
- **Aceite:** app abre a janela principal; bootstrap não registra comandos de domínio nem concede permissões de plugins e a capability da janela não habilita APIs não usadas; falhas de montagem frontend mostram mensagem segura e registram diagnóstico técnico; falhas fatais de inicialização Tauri/Rust registram diagnóstico local e mostram mensagem nativa genérica no Windows; recursos do app não dependem de rede.
- **Tarefas técnicas:** criar scaffold Tauri 2/Vue 3/TypeScript/Vite com pnpm e locks versionados; configurar entrypoint Rust e janela; definir capability mínima; diferenciar fallback/diagnóstico frontend e backend; documentar comandos de desenvolvimento/build.
- **Prioridade:** P0; **Estimativa:** 5; **Dependências:** —; **Fase:** 1; **Status:** Em andamento.

#### US-002 — Navegar pelo layout principal
- **História:** Como usuário, quero acessar os módulos pela navegação principal.
- **Aceite:** layout lateral contém rotas previstas; rota atual é identificável; módulos ainda não entregues mostram estado “em planejamento” sem simular operações concluídas.
- **Tarefas técnicas:** configurar Vue Router; implementar shell/layout e rotas; definir estados vazios; criar tokens básicos de estilo.
- **Prioridade:** P0; **Estimativa:** 3; **Dependências:** US-001; **Fase:** 1; **Status:** Não iniciada.

#### US-003 — Selecionar tema da interface
- **História:** Como usuário, quero escolher tema claro, escuro ou sistema para adequar a interface à minha preferência.
- **Aceite:** tema aplica-se a todas as telas; opção sistema acompanha a preferência do Windows e suas alterações; valor é persistido pelo serviço de US-006; sem valor salvo, usa sistema.
- **Tarefas técnicas:** seletor UI; tokens Tailwind/shadcn-vue; store de aparência; conexão com API de preferências; contraste nos três modos.
- **Prioridade:** P1; **Estimativa:** 3; **Dependências:** US-001, US-006; **Fase:** 1; **Status:** Não iniciada.

#### US-004 — Persistir banco local com migrations
- **História:** Como usuário, quero que meus dados sejam armazenados localmente e sobrevivam ao fechamento do app.
- **Aceite:** banco é criado no diretório de dados da aplicação; conexão habilita FKs; migrations aplicam de forma versionada; falha de migration não sobrescreve silenciosamente o banco.
- **Tarefas técnicas:** configurar SQLite/SQLx; definir pool e caminho via Tauri; criar migration inicial e tabela de migrations; transação/bootstrap e erro seguro.
- **Prioridade:** P0; **Estimativa:** 5; **Dependências:** US-001; **Fase:** 1; **Status:** Não iniciada.

#### US-005 — Validar contratos do backend
- **História:** Como equipe, queremos uma fronteira consistente entre frontend e Rust para reduzir regras divergentes.
- **Aceite:** comando Tauri de diagnóstico demonstra DTO serializado e erro previsível; frontend não acessa SQLite; comandos não expõem detalhes SQL.
- **Tarefas técnicas:** padrão DTO/erro; wrapper tipado de `invoke`; comando diagnóstico; testes de contrato sem persistência de preferências.
- **Prioridade:** P0; **Estimativa:** 3; **Dependências:** US-001, US-004; **Fase:** 1; **Status:** Não iniciada.

#### US-006 — Persistir preferências do aplicativo
- **História:** Como usuário, quero que as preferências básicas sejam armazenadas e recuperadas ao reabrir o aplicativo.
- **Aceite:** backend fornece leitura/gravação tipada da preferência de tema usada por US-003; default `system`; valor persiste após reinício; chave/valor inválidos e falhas retornam erro estruturado; esta história não implementa o seletor visual.
- **Tarefas técnicas:** tabela `app_settings`; repository/casos de uso SQLx; comandos tipados; validação Rust; store consome API sem duplicar persistência.
- **Prioridade:** P1; **Estimativa:** 3; **Dependências:** US-004, US-005; **Fase:** 1; **Status:** Não iniciada.

## Fase 2 — Organização da rotina

### EP-02 — Cronogramas, categorias e agenda
**Objetivo:** permitir planejar atividades em cronogramas e consultar uma semana.

#### US-007 — Gerenciar cronogramas
- **História:** Como usuário, quero criar e organizar cronogramas distintos para separar rotinas.
- **Aceite:** criar/editar/duplicar/arquivar; um principal ativo; datas válidas; selecionar cronograma mostra atividades próprias; excluir só conforme política de histórico aprovada.
- **Tarefas técnicas:** schema/repository/use cases; formulário e lista; duplicação transacional; índice parcial de principal; regras de arquivo/exclusão.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-004, US-005; **Fase:** 2; **Status:** Não iniciada.

#### US-008 — Gerenciar categorias
- **História:** Como usuário, quero criar categorias com cor e ícone para reconhecer minhas atividades.
- **Aceite:** criar/editar/ativar/desativar categoria global; atividades, tarefas e revisões podem referenciá-la; inativa não pode ser atribuída em novos registros/ocorrências; registros existentes preservam referência e snapshots de nome/cor/ícone; exclusão definitiva é recusada com referências.
- **Tarefas técnicas:** schema global; validação de cor/ícone e estado ativo; repository e comandos; UI; copiar nome/cor/ícone usados para snapshots históricos de ocorrências e revisões.
- **Prioridade:** P1; **Estimativa:** 3; **Dependências:** US-004, US-005; **Fase:** 2; **Status:** Não iniciada.

#### US-009 — Criar atividades pontuais
- **História:** Como usuário, quero reservar um horário no cronograma para uma atividade.
- **Aceite:** título, cronograma, datas/horas e categoria são validados; fim posterior ao início no mesmo dia; atividade não atravessa meia-noite; hora pontual inexistente por DST é rejeitada e hora duplicada usa a primeira; resolução em Rust pelo fuso Windows; conflito ativo gera aviso e confirmação.
- **Tarefas técnicas:** schema série/segmentos/ocorrências; casos de uso CRUD; formulário Zod e validação backend; resolver hora segundo ADR-005; testes DST/constraints.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-007, US-008, ADR-005; **Fase:** 2; **Status:** Não iniciada.

#### US-010 — Gerar recorrências de atividades
- **História:** Como usuário, quero repetir atividades diariamente, semanalmente ou em dias definidos.
- **Aceite:** mecanismo próprio diário/semanal/dias da semana; identidade única `(series_id, original_local_key)`; geração idempotente; cancelamentos deixam tombstone; horas DST aplicam ADR-005; passado e histórico permanecem.
- **Tarefas técnicas:** `series_id` estável e segmentos versionados; chave original civil separada da hora efetiva/UTC; materialização sob demanda/horizonte; reconcile transacional; snapshots; testes de idempotência, DST e cancelamento.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-009, ADR-003, ADR-005; **Fase:** 2; **Status:** Não iniciada.

#### US-011 — Consultar agenda semanal
- **História:** Como usuário, quero visualizar atividades por dia e horário para entender minha semana.
- **Aceite:** semana navegável mostra ocorrências do cronograma no fuso Windows; cores/snapshots de categoria preservados; carga e horários livres calculados pelos intervalos exibidos; cancelados/concluídos tratados conforme ADR-006.
- **Tarefas técnicas:** consulta de ocorrências por intervalo; componentes grade semanal; navegação de semana; cálculo de carga e horários livres; testes de fronteira de dia/semana.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-009, US-010; **Fase:** 2; **Status:** Não iniciada.

#### US-012 — Reorganizar atividades e detectar conflitos
- **História:** Como usuário, quero mover/copiar atividades na semana e receber aviso de sobreposição.
- **Aceite:** arrastar, reorganizar e copiar salva; sobreposição de ocorrências ativas no mesmo cronograma exibe aviso e só salva após confirmação explícita; adjacência e cronogramas distintos não conflitam; canceladas e ocorrências históricas concluídas não bloqueiam; backend revalida antes do commit.
- **Tarefas técnicas:** regra de interseção em Rust; operação transacional de movimentação/cópia e revalidação; confirmação na UI; testes de limites, concorrência e status excluídos.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-011, ADR-006; **Fase:** 2; **Status:** Não iniciada.

#### US-033 — Arquivar ou excluir entidades da rotina
- **História:** Como usuário, quero remover cronogramas, categorias ou atividades sem apagar histórico relacionado.
- **Aceite:** cobre integridade de cronogramas, categorias e atividades da Fase 2; com histórico/dependências são arquivadas; exclusão física só sem referências; FKs impedem remoção referenciada; sem cascata que apague ocorrências/sessões; arquivadas somem de seletores novos e permanecem acessíveis no histórico.
- **Tarefas técnicas:** aplicar ADR-008 nos casos de uso da Fase 2; FKs `RESTRICT`; flags de arquivo e snapshots; testes de integridade referencial.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-007, US-008, US-009, ADR-008; **Fase:** 2; **Status:** Não iniciada.

#### US-031 — Editar ocorrência e série recorrente
- **História:** Como usuário, quero alterar uma ocorrência de atividade, esta e as próximas, ou toda a série sem modificar histórico passado.
- **Aceite:** as três opções se aplicam a séries de atividade; uma cria override pela chave original; esta e próximas fecha segmento antes da chave e abre segmento novo no mesmo `series_id`; série inteira substitui regras futuras; passado, conclusão e sessões não mudam; cancelamentos não regeneram.
- **Tarefas técnicas:** modelar segmentos/exceções/tombstones; operações transacionais; preservar snapshots/status/conclusões; reconciliar lembretes futuros; testar edições/idempotência.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-010, ADR-003; **Fase:** 2; **Status:** Não iniciada.

## Fase 3 — Tarefas e notificações

### EP-03 — Tarefas e lembretes
**Objetivo:** organizar tarefas com datas e entregar lembretes no limite de execução aprovado.

#### US-013 — Gerenciar tarefas
- **História:** Como usuário, quero registrar tarefas com prioridade e data opcional.
- **Aceite:** criar/editar/arquivar/excluir tarefa; título obrigatório; tarefa global com cronograma opcional e categoria opcional; pode não ter data; inativas não podem ser usadas em nova tarefa/ocorrência; status persiste; exclusão/arquivo respeita ADR-008.
- **Tarefas técnicas:** modelo de série/segmentos/ocorrências; schedule nullable; snapshot de cronograma/categoria; casos de uso SQLx; formulários e validação frontend/backend.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-004, US-005, US-008; **Fase:** 3; **Status:** Não iniciada.

#### US-014 — Consultar e reagendar tarefas
- **História:** Como usuário, quero ver tarefas do dia/atrasadas e reagendar as pendentes.
- **Aceite:** filtros por data/prioridade/status e cronograma funcionam; lista global inclui tarefas com/sem cronograma; vencimento não conclui tarefa; reagendamento atualiza futuro sem reescrever histórico; contadores refletem ocorrências.
- **Tarefas técnicas:** consultas indexadas; telas por dia/atrasadas; ação reagendar; acessibilidade de estados e testes de datas.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-013, ADR-005; **Fase:** 3; **Status:** Não iniciada.

#### US-015 — Repetir tarefas
- **História:** Como usuário, quero repetir tarefas diariamente ou em dias selecionados.
- **Aceite:** regra diária/semanal/dias específicos gera ocorrências determinísticas com chave baseada na data/hora original (ou marcador de dia inteiro); geração não duplica; tarefa vencida não conclui automaticamente; passado preservado.
- **Tarefas técnicas:** reutilizar gerador de série; segments/ocorrências/exception JSON no mesmo padrão de atividade; horários resolvidos segundo ADR-005; tombstone; testes de chave/idempotência.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-010, US-013, ADR-003, ADR-005; **Fase:** 3; **Status:** Não iniciada.

#### US-016 — Configurar lembretes
- **História:** Como usuário, quero receber lembretes de atividade/tarefa/revisão conforme antecedência escolhida.
- **Aceite:** configuráveis por categoria e antecedência; cobre início de atividade, tarefa, tarefa atrasada, revisão, foco/descanso Pomodoro; processo ativo entrega no horário; fechar janela mantém bandeja; sair interrompe; estados pending/delivered/expired/cancelled são persistidos.
- **Tarefas técnicas:** schema de reminder com exatamente um FK de alvo; scheduler Rust; permissões/capabilities nativas; UI de antecedência/categoria; testes de relógio/eventos.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-009, US-013, ADR-004, ADR-005; **Fase:** 3; **Status:** Não iniciada.

#### US-017 — Usar bandeja e inicialização opcional
- **História:** Como usuário, quero manter o app acessível na bandeja e optar por iniciar com Windows.
- **Aceite:** fechar janela mantém processo na bandeja; menu permite reabrir e sair explicitamente; sair encerra notificações; startup é opt-in/reversível; nenhum agendamento é delegado ao Task Scheduler.
- **Tarefas técnicas:** validar API Tauri em Windows; menu tray; preferência de startup; capabilities; integração NSIS; testes de fechar, reabrir, sair e inicialização.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-001, ADR-004, ADR-010; **Fase:** 3; **Status:** Não iniciada.

#### US-032 — Reconciliar lembretes após suspensão
- **História:** Como usuário, quero que o app verifique lembretes vencidos quando o Windows sair da suspensão.
- **Aceite:** no resume, pending com atraso de até 30 minutos é notificado individualmente; maior atraso marca expired sem toast individual; tarefa permanece pendente/atrasada; itens do mesmo tipo de alvo podem ser agrupados em resumo com contagem/títulos, mantendo estados individuais; nenhuma entrega duplicada.
- **Tarefas técnicas:** detectar resume; reconciliar persistent scheduler; agrupar atividade/tarefa/revisão separadamente; registrar entrega/expiração/group id; testes 30:00/30:01, duplicidade, suspensão longa.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-016, US-017, ADR-004; **Fase:** 3; **Status:** Não iniciada.

#### US-034 — Editar ocorrência e série de tarefas
- **História:** Como usuário, quero editar somente uma ocorrência de tarefa, esta e as próximas, ou futuras da série.
- **Aceite:** as três operações seguem ADR-003 separadamente das atividades; exceção preserva original_local_key; “esta e próximas” divide segmentos no mesmo `series_id`; editar série altera ocorrências futuras; histórico passado/conclusões preservados; mudança de cronograma/categoria em série só afeta futuras; cancelamento permanece tombstone.
- **Tarefas técnicas:** operações SQLx transacionais para task segments; exception snapshots; reconciliação de lembretes futuros; testes de divisão de série e idempotência.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-015, ADR-003; **Fase:** 3; **Status:** Não iniciada.

## Fase 4 — Controle de tempo

### EP-04 — Cronômetro e Pomodoro
**Objetivo:** registrar tempo efetivo sem depender de atualizações contínuas da interface.

#### US-018 — Cronometrar atividade
- **História:** Como usuário, quero iniciar, pausar, retomar e finalizar uma sessão associada opcionalmente a uma atividade.
- **Aceite:** persistir `running/paused/interrupted/completed`, acumulado ativo, checkpoint e segmentos de pausa; minimização/tray não altera tempo; suspensão observada conta como tempo ativo se rodando; encerrar processo marca interrompido e não conta período offline; na próxima abertura o usuário pode retomar do acumulado ou finalizar; crash recupera apenas até último checkpoint.
- **Tarefas técnicas:** schema de segmentos e checkpoints; checkpoint periódico máximo de 5 segundos e em transições/suspend/resume; eventos Windows; recuperação de instância anterior; UI para decisão retomar/finalizar; testes de pause/crash/suspensão.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-009, ADR-005; **Fase:** 4; **Status:** Não iniciada.

#### US-019 — Configurar e executar Pomodoro
- **História:** Como usuário, quero executar ciclos de foco/descanso configuráveis.
- **Aceite:** padrões iniciais são configuráveis; pausas/retomada/finalização funcionam; descanso longo segue ciclo configurado; etapa concluída notifica conforme política; apenas ciclos concluídos são contados conforme decisão.
- **Tarefas técnicas:** fechar semântica de ciclo e recuperação; modelo de sessões/ciclos; máquina de estados; notificações; UI e testes de relógio.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-016, US-018; **Fase:** 4; **Status:** Não iniciada.

#### US-020 — Consultar histórico de tempo
- **História:** Como usuário, quero consultar sessões registradas e tempo por atividade.
- **Aceite:** histórico pode ser filtrado por período/atividade; duração contabiliza pausas; sessões sem atividade ficam identificáveis; soma reproduzível.
- **Tarefas técnicas:** consultas e agrupamentos; tela de histórico; regras de associação; testes agregados.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-018; **Fase:** 4; **Status:** Não iniciada.

#### US-021 — Comparar planejado e realizado
- **História:** Como usuário, quero comparar tempo reservado com tempo efetivamente registrado.
- **Aceite:** usa período local com início inclusivo/fim exclusivo; agenda elegível são ocorrências não canceladas do período que já começaram (período histórico inclui todas, futuro nenhuma); numerador conta concluídas; planejado concluído soma durações planejadas concluídas; cumprimento da agenda divide pelo planejado elegível; tarefas elegíveis são ocorrências datadas do período, recorrentes contam individualmente, canceladas excluídas e tarefas sem data fora/contadas à parte; zero retorna sem dados; tempo registrado nunca se soma a planejado.
- **Tarefas técnicas:** implementar funções agregadoras Rust únicas; consultar snapshots/ocorrências e sessões; consistência de filtros de período; testes com ocorrência futura/cancelada, tarefa sem data e recorrência.
- **Prioridade:** P2; **Estimativa:** 5; **Dependências:** US-010, US-011, US-013, US-014, US-015, US-020, US-034, ADR-007; **Fase:** 4; **Status:** Não iniciada.

## Fase 5 — Produtividade

### EP-05 — Revisões, métricas e relatórios
**Objetivo:** resumir execução da rotina e exportar resultados legíveis.

#### US-022 — Gerenciar revisões
- **História:** Como usuário, quero cadastrar manualmente revisões futuras e marcá-las concluídas.
- **Aceite:** criar/editar/concluir/arquivar/excluir; lista futura/atrasada consistente; vínculo opcional válido; categorias globais e inativas não podem ser atribuídas a nova revisão; snapshots históricos preservados; exclusão/arquivo conforme ADR-008; revisões não são geradas automaticamente.
- **Tarefas técnicas:** schema/queries/use cases; snapshots de categoria; política de archive/delete e FKs `RESTRICT`; formulário/listas; testes de estados/referências.
- **Prioridade:** P1; **Estimativa:** 5; **Dependências:** US-004, US-005, US-008; **Fase:** 5; **Status:** Não iniciada.

#### US-023 — Visualizar dashboard
- **História:** Como usuário, quero ver resumo diário de atividades, tarefas e cumprimento.
- **Aceite:** apresenta atividades do dia e próximas, horas planejadas, tempo registrado, planejado concluído, cumprimento da agenda, tarefas pendentes/concluídas/atrasadas e cumprimento de tarefas; tarefas sem data aparecem separadas; usa fórmulas do ADR-007 iguais às dos relatórios; denominador zero mostra sem dados.
- **Tarefas técnicas:** implementar agregadores centralizados em Rust; DTO dashboard; cards/gráficos; testes compartilhados com relatórios.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-011, US-014, US-020, US-021; **Fase:** 5; **Status:** Não iniciada.

#### US-024 — Consultar relatórios por período
- **História:** Como usuário, quero consultar produtividade diária, semanal e mensal e horas por categoria.
- **Aceite:** filtros de período retornam totais coerentes; concluídas/pendentes/atrasadas e atividades não realizadas usam definições aprovadas; histórico de cronogramas acessível.
- **Tarefas técnicas:** especificar fórmulas; queries parametrizadas; telas e gráficos; verificar limites de performance em dataset representativo.
- **Prioridade:** P2; **Estimativa:** 8; **Dependências:** US-021, US-022, US-023; **Fase:** 5; **Status:** Não iniciada.

#### US-025 — Exportar PDF
- **História:** Como usuário, quero exportar agenda, tarefas e relatórios em formato imprimível.
- **Aceite:** exportações previstas permitem escolher destino; conteúdo corresponde ao filtro/tela; páginas não cortam informação essencial; falha é informada.
- **Tarefas técnicas:** escolher estratégia/lib; gerar layout para agenda/tarefas/relatórios; diálogos de arquivo; testes visuais e com múltiplas páginas.
- **Prioridade:** P2; **Estimativa:** 8; **Dependências:** US-011, US-014, US-024; **Fase:** 5; **Status:** Não iniciada.

## Fase 6 — Finalização

### EP-06 — Proteção de dados e release
**Objetivo:** permitir recuperar dados e entregar instalação confiável para Windows.

#### US-026 — Criar backup manual e consultar histórico
- **História:** Como usuário, quero criar cópia de segurança e saber quando ela foi gerada.
- **Aceite:** usuário escolhe local; backup captura snapshot consistente do SQLite mesmo com WAL ativo; validação de integridade e schema ocorre antes de publicar; histórico distingue backup manual/automático; banco ativo permanece íntegro; rotação nunca remove backup manual.
- **Tarefas técnicas:** `VACUUM INTO` via SQLx para arquivo temporário; validação antes da publicação; file dialog; schema de histórico; registro de versão; testes de falha e WAL.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-004; **Fase:** 6; **Status:** Não iniciada.

#### US-027 — Restaurar backup
- **História:** Como usuário, quero restaurar backup validado após confirmar substituição dos dados atuais.
- **Aceite:** confirmação mostra consequência; antes da troca, cópia temporária passa `integrity_check`, `foreign_key_check` sem violações e validação de compatibilidade do schema; conexões fecham antes do swap e WAL/SHM são tratados como conjunto; banco anterior permanece disponível para rollback; pool reabre somente após troca; falha restaura e reabre banco anterior; sucesso apresenta os dados restaurados.
- **Tarefas técnicas:** validar/migrar cópia temporária compatível; bloquear gravações; checkpoint e fechamento do pool; swap recuperável com rollback e marcador; tratar main/WAL/SHM; validar após abertura; testar corrupção, interrupção e versões incompatíveis.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-026; **Fase:** 6; **Status:** Não iniciada.

#### US-028 — Configurar backup automático
- **História:** Como usuário, quero configurar cópias automáticas para reduzir risco de perda de dados.
- **Aceite:** na abertura, criar backup se passou ao menos um dia desde último automático válido; manual não reinicia o intervalo; default em pasta própria no diretório de dados; usuário pode escolher outra; manter sete automáticos; nunca rotacionar manuais; falha informada; sem nuvem.
- **Tarefas técnicas:** comparar timestamps de automáticos válidos; bootstrap; rotação por tipo; default path; histórico mantém status `rotated`; testes de relógio/falhas/preservação manual.
- **Prioridade:** P2; **Estimativa:** 5; **Dependências:** US-026, US-017; **Fase:** 6; **Status:** Não iniciada.

#### US-029 — Validar release Windows
- **História:** Como usuário, quero instalar e atualizar o app com dados preservados.
- **Aceite:** instalador NSIS executa em Windows 10 22H2 e Windows 11 x64; upgrade manual preserva banco fora da instalação; uninstall não remove dados silenciosamente e eventual remoção é opt-in confirmada; assinatura digital é exigida no artefato público; build dev pode não ser assinado.
- **Tarefas técnicas:** target x64 e validação em versões alvo; configurar NSIS; pipeline separado dev/publicação; assinar artefato público; testes clean install/upgrade/uninstall/tray/startup; confirmar preservação do caminho de dados.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-001, US-004, US-027, US-017; **Fase:** 6; **Status:** Não iniciada.

#### US-030 — Fechar qualidade e documentação de release
- **História:** Como equipe, queremos confiança nos fluxos completos antes da primeira versão.
- **Aceite:** testes automatizados de todas as histórias predecessoras passam; checklist Windows 10 22H2/11 x64 registrado; instalação/update/restore e backup verificados; documentação atualizada; nenhuma falha conhecida de perda de dados.
- **Tarefas técnicas:** integração Rust/SQLite; fluxos Playwright; smoke tests Windows; otimização baseada em medições; documentação de suporte e release notes.
- **Prioridade:** P1; **Estimativa:** 8; **Dependências:** US-001–US-029 e US-031–US-034; **Fase:** 6; **Status:** Não iniciada.

## Resumo de dependências críticas

`US-001 → US-004 → US-005 → US-006 → US-003`; `US-007 + US-008 → US-009 → US-010 → US-031 → US-011 → US-012`; `US-010 → US-013 → US-014/US-015 → US-034`; `US-016 + US-017 → US-032`; `US-018 → US-020`; `US-011 + US-013 + US-014 + US-015 + US-020 → US-021 → US-023 → US-024 → US-025`; `US-026 → US-027`; `US-026 + US-017 → US-028`; `US-017 + US-027 → US-029`; US-030 depende de todas as histórias anteriores listadas, exceto de si mesma.

## Primeira história pronta para implementação

**US-001 — Iniciar o Life Planner no Windows** está em andamento sob autorização explícita. Nenhuma outra história foi iniciada; US-002 permanece não iniciada.

