# Especificação de requisitos — Life Planner

**Fonte:** [PRD.md](PRD.md), versão 1.0  
**Estado:** derivado do PRD e atualizado pelas ADRs aceitas; refinamentos de implementação estão identificados na seção 5.  
**Convenção:** `REQ-Fxx` funcional, `REQ-NFxx` não funcional, `RNxx` regra de negócio. Este documento organiza e esclarece o PRD, sem ampliar silenciosamente o escopo.

## 1. Escopo do produto

Aplicativo desktop pessoal, local-first e offline para Windows 10/11, destinado a organizar cronogramas, atividades com horários, tarefas, revisões e registros de tempo. Não há contas, sincronização ou serviços externos no escopo inicial.

## 2. Requisitos funcionais

| ID | Requisito verificável | Origem |
|---|---|---|
| REQ-F01 | Criar, editar, duplicar, arquivar, excluir e selecionar cronogramas; definir principal e datas de vigência; consultar anteriores. | RF01 |
| REQ-F02 | Criar, editar, ativar/desativar e excluir categorias globais sem dependências; definir nome, cor e ícone. Categorias inativas não podem ser usadas em novos registros; histórico preserva informação da categoria. | RF02, ADR-002 |
| REQ-F03 | Criar, editar, duplicar e excluir atividades com nome, descrição, categoria, cronograma, horário, recorrência, vigência, lembrete e status. | RF03 |
| REQ-F04 | Criar atividades recorrentes ou pontuais usando recorrência própria diária, semanal ou por dias da semana; permitir editar uma ocorrência, esta e próximas, ou a série; geração determinística; preservar ocorrências e históricos passados. | RF03, RN04, RN09–10, ADR-003 |
| REQ-F05 | Detectar sobreposição de atividades no mesmo cronograma; adjacentes e cronogramas diferentes não conflitam; canceladas e ocorrências históricas concluídas não bloqueiam. Avisar e exigir confirmação para salvar. | RF03, RN05, ADR-006 |
| REQ-F06 | Exibir agenda semanal navegável, horários, atividades por categoria, carga planejada e horários livres; permitir cadastro rápido e alteração/reordenação/copiar entre dias. | RF04 |
| REQ-F07 | Criar/editar/arquivar/excluir tarefas globais com título, descrição, data, prioridade, categoria opcional, vínculo opcional a cronograma, prazo/horário opcionais, lembrete, status e recorrência. Tarefa pode não ter data nem cronograma. | RF05, ADR-011 |
| REQ-F08 | Listar tarefas por dia e atrasadas, filtrar por prioridade/status, concluir e reagendar; tarefas vencidas permanecem não concluídas. | RF05, RN08 |
| REQ-F09 | Registrar sessões com início, pausa, retomada e fim; opcionalmente relacionar a atividade; persistir estado e segmentos de pausa; continuar durante suspensão; ao encerrar processo, interromper e não contar tempo offline; na abertura, permitir retomar acumulado ou finalizar. | RF06, RN11–13, ADR-005 |
| REQ-F10 | Executar Pomodoro configurável com foco, descanso, descanso longo após ciclos, controle de ciclo e registro de sessões concluídas. | RF07 |
| REQ-F11 | Notificar início de atividade, lembretes de tarefas/revisões, tarefas atrasadas e etapas Pomodoro/descanso; permitir antecedência e ativação por categoria. Com processo ativo, notificar no horário. Ao retornar de suspensão, notificar individualmente itens vencidos há até 30 minutos; itens mais antigos ficam expirados e consultáveis; similares podem ser agrupados sem perder seus registros individuais. | RF08, ADR-004 |
| REQ-F12 | Fechar janela mantém app na bandeja; sair explicitamente encerra notificações; inicialização com Windows é opcional. | RF08, ADR-004 |
| REQ-F13 | Criar, editar, excluir, concluir e consultar revisões futuras/atrasadas, com título, descrição, data/hora opcional, categoria, vínculo e lembrete. | RF09 |
| REQ-F14 | Exibir atividades do dia, próximas atividades, métricas separadas de tempo registrado, planejado concluído, cumprimento da agenda e cumprimento de tarefas, além de horas planejadas e estados/contagens das tarefas. Fórmulas centralizadas no backend. | RF10, ADR-007 |
| REQ-F15 | Consultar relatórios diário, semanal e mensal, horas por categoria, métricas separadas de planejado/registrado, estado das tarefas, atividades não realizadas e histórico de cronogramas. | RF11, ADR-007 |
| REQ-F16 | Criar backup manual e automático diário; executar automático na abertura se passou ao menos um dia desde o último backup automático válido (manual não reinicia o intervalo); destino padrão em pasta própria nos dados do app, com destino alternativo; reter sete automáticos sem remover manuais; validar integridade, FKs e schema antes de restore; sem nuvem. | RF12, ADR-009 |
| REQ-F17 | Exportar agenda semanal, tarefas diárias e relatórios/resumo mensal/emissões por categoria para PDF imprimível. | RF13 |
| REQ-F18 | Configurar tema claro/escuro/sistema, notificações, Pomodoro, diretório de backup, inicialização e preferências básicas. Português é fixo, sem seletor de idioma; horários de funcionamento estão fora da v1.0. | RF14 |

## 3. Requisitos não funcionais

| ID | Requisito | Como verificar / lacuna |
|---|---|---|
| REQ-NF01 | Suporte Windows 10 22H2 e Windows 11 x64. | Validar em instalações limpas e upgrades. |
| REQ-NF02 | Operação principal sem internet. | Executar fluxos suportados sem rede; recursos externos não previstos. |
| REQ-NF03 | Interface em português fixo na v1.0. | Sem seletor de idioma na v1.0. |
| REQ-NF04 | Tema claro, escuro e sistema. | Tema sistema acompanha preferência do Windows. |
| REQ-NF05 | Persistência local SQLite. | Banco e migrations; definir localização e política de migração. |
| REQ-NF06 | Interface responsiva e intuitiva. | Sem métricas ou tamanhos-alvo definidos; validar em resoluções Windows a decidir. |
| REQ-NF07 | Arquitetura modular com validação no backend. | Regras críticas e persistência executadas em Rust; contratos definidos. |
| REQ-NF08 | Proteção contra perda de dados e restauração íntegra. | Testes de migração, backup, validação e restauração; metas de recuperação pendentes. |
| REQ-NF09 | Testes automatizados. | Escopo e cobertura mínima não especificados. Stack: Vitest, Playwright, Rust tests. |
| REQ-NF10 | Instalador NSIS, updates manuais, dados fora da instalação e sem remoção silenciosa no uninstall; assinatura digital obrigatória para distribuição pública. | Windows 10 22H2/11 x64; ADR-010. |
| REQ-NF11 | Todas as operações SQLite no backend Rust usam exclusivamente SQLx; Vue não abre conexão nem acessa arquivo SQLite. | ADR-001, arquitetura em camadas. |

## 4. Regras de negócio consolidadas

Os códigos RN01–RN16 abaixo são idênticos aos originais de PRD.md. Regras aprovadas acrescentadas nesta especificação usam prefixo `LP-RN` para não colidir com identificadores de origem.

| ID | Regra |
|---|---|
| RN01 | O usuário poderá manter vários cronogramas. |
| RN02 | Cada cronograma terá atividades independentes. |
| RN03 | As categorias serão personalizáveis. |
| RN04 | Uma atividade poderá ser recorrente ou pontual. |
| RN05 | O sistema deverá identificar conflitos de horários no mesmo cronograma. |
| RN06 | Atividades e tarefas serão entidades independentes. |
| RN07 | Tarefas poderão existir sem horário definido. |
| RN08 | Tarefas pendentes não serão marcadas automaticamente como concluídas. |
| RN09 | Atividades realizadas deverão preservar seu histórico. |
| RN10 | Alterações em recorrências futuras não deverão modificar ocorrências anteriores. |
| RN11 | O cronômetro poderá ser associado a uma atividade. |
| RN12 | Sessões concluídas deverão gerar registros históricos. |
| RN13 | O tempo realizado deverá ser comparável ao tempo planejado. |
| RN14 | Revisões serão cadastradas manualmente. |
| RN15 | O sistema deverá funcionar sem conexão com a internet. |
| RN16 | A restauração de backup deverá validar a integridade dos dados. |
| LP-RN01 | Tarefas são globais e possuem vínculo opcional a cronograma. |
| LP-RN02 | Recorrências próprias suportam frequência diária, semanal e dias específicos; edição abrange ocorrência, esta e próximas, ou ocorrências futuras da série, sem reescrever histórico passado. |
| LP-RN03 | A identidade de cada ocorrência é determinística e única pela série e data/hora civil originalmente prevista. |
| LP-RN04 | Horas recorrentes inexistentes por DST avançam ao primeiro horário válido seguinte; horas duplicadas usam a primeira ocorrência; atividade pontual em hora inexistente é rejeitada; instantes históricos permanecem em UTC. |
| LP-RN05 | Cronômetro conta durante suspensão, mas tempo em que o aplicativo fica encerrado não é contado; encerramento inesperado interrompe a sessão para decisão do usuário. |
| LP-RN06 | Métricas usam ocorrências/tarefas elegíveis no período; atividades futuras e canceladas são excluídas; tarefas sem data ficam fora do denominador e recorrências contam por ocorrência. |
| LP-RN07 | Ao retomar de suspensão, lembretes vencidos há até 30 minutos são notificáveis; os mais antigos expiram e continuam identificáveis. |
| LP-RN08 | Backup automático diário considera o último automático válido, retém sete automáticos e nunca remove manuais pela rotação. |
| LP-RN09 | Distribuição v1.0 suporta Windows 10 22H2/Windows 11 x64 por instalador NSIS; updates manuais; dados fora da instalação; assinatura é obrigatória para distribuição pública. |
| LP-RN10 | Tema oferece claro/escuro/sistema; idioma é português fixo e horários de funcionamento ficam fora da v1.0. |
| LP-RN11 | Categorias são globais; inativas não podem ser usadas em novos registros; histórico preserva nome, cor e ícone; exclusão física só sem referências. |
| LP-RN12 | Conflitos são sobreposições estritas no mesmo cronograma; adjacência/diferentes cronogramas não conflitam; canceladas e históricas concluídas não bloqueiam; salvar conflito requer confirmação. |

## 5. Ambiguidades, inconsistências e lacunas

| Ref. | Questão | Impacto | Tratamento proposto |
|---|---|---|---|
| A01 | Política de materialização (sob demanda ou horizonte) e eventual tamanho de lote não são regras de produto. | Volume e consultas. | Escolher em implementação, respeitando identidade e idempotência de ADR-003. |
| A02 | A configuração Pomodoro por sessão/ciclo e vínculo de sessão com atividade não estão definidos pelo PRD. | Fase 4 e histórico Pomodoro. | Fechar nos critérios de US-019 antes da implementação. |
| A03 | Política de distribuição PDF (motor, paginação e renderização) continua escolha técnica. | Exportação. | Selecionar na Fase 5, mantendo saída imprimível e offline. |
| A04 | Metas de acessibilidade, desempenho e volume de histórico não foram quantificadas. | Validação não funcional. | Definir metas verificáveis antes de release público. |

## 6. Decisões aceitas

- ADR-001 a ADR-011 aprovados: arquitetura local-first e SQLx exclusivo; categorias globais; tarefas globais com cronograma opcional; recorrência própria com três escopos; fuso Windows/DST; notificações/timer; conflitos confirmados; fórmulas; arquivamento; backup; distribuição NSIS.

Detalhes residuais listados na seção de ambiguidades são escolhas técnicas ou regras ainda não definidas pelo PRD e devem ser fechados nos critérios das histórias correspondentes.

## 7. Dependências de produto

Fundação de app, persistência e configurações antecedem domínio. Cronogramas/categorias precedem atividades e agenda. Definições/ocorrências precedem lembretes, relatórios e histórico. Tarefas precedem seus lembretes e métricas. Sessões temporais precedem horas realizadas. Dados de atividades, tarefas e sessões precedem dashboard, relatórios e PDF. Backup depende do formato e localização estáveis do banco.

