# Arquitetura proposta

**Estado:** arquitetura-base; ADR-001 a ADR-011 aceitos.  
**Objetivo:** aplicativo Windows local-first, simples de empacotar, validar e evoluir por módulos.

## 1. Visão de execução

```text
Vue 3 + TypeScript (WebView)
  páginas e módulos → stores Pinia → serviços tipados de domínio
                                  ↓ invoke/eventos Tauri
Tauri 2 / Rust
  comandos de entrada → casos de uso → domínio/regras → repositórios SQLx
                                              ↓
                                           SQLite local

Adaptadores nativos Rust: notificações, bandeja, arquivos/backup, exportação PDF,
integração de inicialização do Windows. Acesso restrito por capabilities Tauri.
```

O frontend apresenta estado e coleta entradas; regras que afetam integridade, conflitos, recorrências, cálculos e persistência pertencem ao Rust. Todas as operações SQLite do backend Rust usam exclusivamente SQLx (queries, migrations e transações); não usar rusqlite, chamadas SQLite diretas ou plugins de banco paralelos. Vue não conecta ao SQLite e só acessa dados por comandos/eventos Tauri. A interface continua utilizável offline.

## 2. Fronteiras e responsabilidades

### Frontend (`src/`)

- `app/`: montagem Vue, rotas, layout global, inicialização e stores de shell/configuração.
- `modules/<domínio>/`: páginas, componentes específicos, formulários, tipos de view e stores para dashboard, schedules, categories, activities/calendar, tasks, timers/pomodoro, reminders, reviews, reports, settings e backups.
- `components/ui/`: componentes base shadcn-vue; `components/shared/`: elementos compartilhados sem regra de domínio.
- `services/tauri/`: wrappers tipados de `invoke` e eventos; um wrapper por domínio, sem SQL.
- `composables/`: estado/interação reutilizável de UI (não repositórios).
- `types/`: contratos compartilhados quando necessários; evitar duplicar tipos gerados manualmente sem necessidade.
- Zod valida formulários para feedback rápido; validação autoritativa no backend.
- Pinia contém estado de tela/cache e preferências transitórias, não é fonte de verdade persistente.

### Backend (`src-tauri/src/`)

- `commands/`: camada fina Tauri: desserialização, autorização/capabilities aplicáveis, chamada ao caso de uso, serialização de erro estável.
- `application/`: casos de uso e transações; coordena repositórios e serviços.
- `domain/`: entidades/valores e regras puras (conflito, ocorrência, estados e métricas quando definidos).
- `infrastructure/db/`: pool SQLx, queries/repositórios e migrations; transações e constraints SQLite.
- `infrastructure/platform/`: notificações, bandeja/inicialização, relógio, arquivos, PDF e backup.
- `main.rs`: bootstrap, estado gerenciado (pool/configuração), registro de comandos e plugins.

Evitar abstrair cada tabela em camadas genéricas. Criar módulos somente quando há regra/responsabilidade distinta. Tauri commands são API interna; manter DTOs estáveis, pequenos, com datas em ISO-8601 e erros identificáveis.

## 3. Fluxo e contratos

Fluxo normal: componente → store/action → serviço de domínio → `invoke` → comando → caso de uso → repositório/SQLite → DTO de resposta. Mutação valida no backend e faz commit antes de responder. Consultas de agenda retornam ocorrências no intervalo solicitado. Eventos nativos (timer/notificação) atualizam stores sem substituir a persistência.

Definir convenção de erros antes dos primeiros comandos: código (`validation`, `not_found`, `conflict`, `storage`, `platform`), mensagem segura para usuário e detalhes estruturados apenas para diagnóstico. Não retornar erros SQL brutos.

## 4. Estratégia de dados e tempo

SQLite em diretório de dados da aplicação, caminho obtido pela API de paths do Tauri. Habilitar foreign keys em cada conexão, migrations versionadas e transações para operações compostas. SQLx é o único cliente SQLite, incluindo migrations e backup SQL; operações de arquivo do swap/rename são feitas pelo backend Rust depois de fechar o pool. Consultas parametrizadas; evitar SQL dinâmico com concatenação. Uma instância local, sem servidor.

Usar fuso horário local do Windows, sem fuso independente por cronograma. Resolver datas civis, recorrência e instantes no backend Rust. Persistir data/hora local original prevista, data/hora efetiva resolvida e instantes em UTC. Atividades não atravessam a meia-noite na v1.0. Em recorrência, hora local inexistente desloca para o primeiro horário válido seguinte; hora duplicada escolhe a primeira ocorrência; atividade pontual em hora inexistente falha validação. Ocorrências passadas mantêm instantes UTC resolvidos; futuras são resolvidas pelo fuso local vigente.

## 5. Recorrência e identidade

Atividades e tarefas têm identidade estável de série e segmentos/versionamentos de regra com intervalo de validade. A chave lógica da ocorrência é `(series_id, original_local_key)`, onde `original_local_key` representa a data/hora civil originalmente prevista; tarefa sem hora usa marcador `ALL_DAY`. A PK é obrigatoriamente UUIDv5 de namespace fixo, série e chave; constraint composta única também é obrigatória. Horários ajustados por DST e exceções mantêm a chave original.

Uma edição individual atualiza snapshot/override da ocorrência original. “Esta e próximas” fecha o segmento anterior antes da chave selecionada e abre um novo segmento no mesmo `series_id`; “toda a série” atualiza as regras futuras da série. Ocorrências e sessões passadas ficam imutáveis. Cancelamentos de ocorrência ficam como tombstone para impedir que geração futura a recrie. A geração é idempotente e determinística.

## 6. Cronômetro e continuidade

Estado persistido: `running`, `paused`, `interrupted` ou `completed`; tempo ativo acumulado; início do segmento ativo; checkpoint máximo de cinco segundos; identificador da instância do processo e segmentos de pausa com início/fim. Timer continua durante janela minimizada, tray e suspensão. Ao receber eventos de suspensão/retomada, backend contabiliza intervalo suspenso se timer estava rodando. Encerramento normal interrompe sessão e grava ponto de parada. Em crash, startup marca sessões running da instância anterior como interrompidas e conta somente até o último checkpoint. Checkpoint periódico máximo de cinco segundos e em transições/suspend/resume. Usuário pode retomar acumulado ou finalizar; tempo após o processo encerrar não entra.

## 7. Métricas de produtividade

Períodos usam datas locais, início inclusivo e fim exclusivo. Atividade elegível é ocorrência não cancelada do período iniciada até o instante de referência; período histórico inclui todas as ocorrências não canceladas, período futuro nenhuma. Tempo planejado concluído soma a duração planejada das ocorrências elegíveis marcadas realizadas. Cumprimento da agenda divide essa duração pela duração planejada total elegível. Tempo registrado soma duração de sessões concluídas, descontando pausas, atribuindo cada sessão concluída integralmente ao período de seu início local, sem repartir a duração entre períodos; não é somado ao tempo planejado.

Cumprimento de tarefas divide ocorrências concluídas pelo número de ocorrências de tarefa com data prevista no período; séries contam uma por ocorrência, canceladas são excluídas e tarefas sem data ficam fora do denominador, exibidas separadamente. Tarefas com data futura dentro do período permanecem previstas. Denominador zero retorna `sem dados/não calculável`. Um único serviço de aplicação Rust alimenta dashboard e relatórios.

## 8. Ciclo de vida nativo

Fechar a janela mantém o processo na bandeja do Windows. Notificações funcionam enquanto o processo está ativo; sair explicitamente interrompe futuras entregas. Inicialização automática é opcional. Ao retornar de suspensão, reconciliar lembretes: vencidos há até 30 minutos são entregues; mais antigos ficam `expired` e consultáveis. Tarefas relacionadas continuam pendentes/atrasadas. Agrupar itens semelhantes por tipo de alvo em um resumo nativo, preservando registros individuais no app. A v1.0 não integra com o Agendador de Tarefas do Windows. Validar em Windows 10 22H2 e 11 x64.

## 9. Segurança e confiabilidade

- Capabilities Tauri mínimas para cada janela; restringir plugins e APIs de filesystem/shell.
- Caminhos de backup escolhidos pelo usuário através de dialogs; não aceitar caminhos arbitrários do renderer sem validação.
- Queries parametrizadas, FK ligadas, constraints de domínio no schema e validação Rust.
- Backup: executar SQLite `VACUUM INTO` via SQLx em arquivo temporário novo; resulta em snapshot lógico consistente incluindo commits WAL. Validar antes de publicar, pois interrupção durante a operação pode deixar arquivo incompleto. Nunca copiar somente arquivo principal com WAL ativo. Destino padrão em pasta própria no diretório de dados; destino alternativo configurável. Automático na abertura se passou um dia desde último automático válido; manual não reinicia intervalo; sete automáticos retidos, manuais fora da rotação.
- Restore: confirmação; validar cópia temporária com `integrity_check='ok'`, zero linhas em `foreign_key_check` e versão de migrations suportada. Backups antigos podem ser migrados na cópia antes da troca; backups de versão futura são recusados. Bloquear mutações, checkpoint WAL e fechar pool/conexões; mover main/WAL/SHM para rollback; renomear cópia validada no mesmo volume; abrir novo pool somente depois; executar validação final. Em falha, fechar candidato, restaurar conjunto anterior e reabrir. Manter marcador de restore para recuperar rollback após crash entre renames; apagar rollback só após sucesso.
- Segredos não são previstos; dados pessoais permanecem no dispositivo. Criptografia em repouso não está no PRD e exige decisão própria.

## 10. Testes e qualidade por camada

Sem implementar nesta etapa. Plano: Rust unit tests para regras puras; testes de integração SQLite temporário/migrations/repositórios; Vitest para componentes e stores isolados; Playwright para fluxos desktop suportados e UI em builds apropriados. Testes de Windows para notificações, tray, startup e instalador. Contratos e regras críticas devem ser testáveis sem abrir janela.

## 11. Estrutura sugerida

Usar a estrutura proposta no PRD com estes ajustes de responsabilidade:

```text
src/app/{router,stores}/
src/components/{ui,layout,shared}/
src/modules/<domínio>/{components,pages,stores,schemas}/
src/services/tauri/<domínio>.ts
src/composables/  src/types/  src/styles/
src-tauri/src/{commands,application,domain,infrastructure}/
src-tauri/src/infrastructure/{db,platform}/
src-tauri/migrations/  src-tauri/capabilities/
```

Manter módulos `activities` e `calendar` distintos em UI, mas compartilhar os casos de uso e modelo de ocorrências no backend. `study_courses` e `study_contents` são opcionais no próprio PRD, mas não têm requisito funcional associado; ficam fora da primeira entrega até decisão.

## 12. Distribuição Windows

Alvo v1.0: Windows 10 22H2 e Windows 11, x64; instalador NSIS e atualização manual. Dados ficam fora do diretório de instalação e upgrade preserva o banco. Desinstalação não remove dados silenciosamente; eventual remoção deve ser escolha explícita. Assinatura digital é obrigatória para distribuição pública, não para desenvolvimento.

## 13. Sequenciamento de arquitetura

Fase 1 valida boot, janela/capabilities, tema claro/escuro/sistema, persistência de preferências, conexão SQLx e migrations. Fase 2 valida recorrência, DST e agenda. Fase 3 prova notificações/tray/startup e reconciliação após suspensão. Fase 4 prova recuperação de timers. Fase 5 consome agregadores ADR-007 idênticos no dashboard/relatórios. Fase 6 fecha backup/restore com WAL, rollback, NSIS e testes em Windows 10 22H2/11 x64.

