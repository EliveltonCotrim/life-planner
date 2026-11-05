# Modelo de dados SQLite

**Estado:** modelo lógico atualizado pelas ADRs aceitas; questões restantes estão indicadas como refinamentos. Nomes em `snake_case`; PKs `TEXT` UUID; timestamps de instantes em UTC ISO-8601 (`TEXT`); datas civis `YYYY-MM-DD`; horário civil `HH:MM`. Habilitar `PRAGMA foreign_keys=ON` para toda conexão.

## 1. Entidades e colunas propostas

| Tabela | Colunas principais | Constraints e observações |
|---|---|---|
| `app_settings` | `key TEXT PK`, `value_json TEXT NOT NULL`, `updated_at` | Chaves conhecidas versionadas pela aplicação; dados simples, não usar tabela para estado de domínio. |
| `schedules` | `id PK`, `name`, `description`, `starts_on DATE NULL`, `ends_on DATE NULL`, `is_archived BOOL`, `is_default BOOL`, `created_at_utc`, `updated_at_utc`, `archived_at_utc NULL` | `CHECK(ends_on IS NULL OR starts_on IS NULL OR ends_on >= starts_on)`; no máximo um default não arquivado via índice parcial; referências históricas restringem exclusão física. |
| `categories` | `id PK`, `name`, `color`, `icon NULL`, `is_active BOOL`, `created_at_utc`, `updated_at_utc`, `archived_at_utc NULL` | Global. Novos registros só podem referenciar categoria ativa; histórico pode referenciar categoria inativa/arquivada. FK `RESTRICT`; exclusão física apenas sem referências. |
| `activities` | `id PK` (stable `series_id`), `schedule_id FK`, `created_at_utc`, `archived_at_utc NULL` | Identidade lógica da série de atividade. FK ao cronograma `RESTRICT`; arquivamento preserva segmentos e ocorrências. |
| `activity_series_segments` | `id PK` (`segment_id`), `activity_id FK`, `effective_from_key`, `effective_until_key NULL`, `title`, `description NULL`, `category_id FK`, `kind`, `starts_on DATE`, `ends_on DATE NULL`, `start_time`, `end_time`, `recurrence_rule_json NULL`, `created_at_utc` | Regras versionadas da mesma série; intervalos de chaves originais não se sobrepõem; `kind` once/recurring; `end_time > start_time`; sem cruzar meia-noite. Categoria ativa exigida ao criar/alterar futuro. |
| `activity_occurrences` | `id PK` UUIDv5 de `series_id + original_local_key`, `activity_id FK`, `segment_id FK`, `original_local_key TEXT`, `original_local_date DATE`, `original_local_time`, `effective_local_date DATE`, `effective_start_time`, `effective_end_time`, `starts_at_utc`, `ends_at_utc`, `status`, snapshots de título/descrição/categoria (nome/cor/ícone), `is_exception BOOL`, `exception_data_json NULL`, `completed_at_utc NULL`, `created_at_utc`, `updated_at_utc` | `UNIQUE(activity_id, original_local_key)` é garantia adicional; chave usa data/hora civil originalmente prevista, nunca ajuste DST ou movimento de exceção. Cancelamento mantém tombstone. `status` inclui cancelada/concluída. |
| `daily_tasks` | `id PK` (stable `series_id`), `created_at_utc`, `archived_at_utc NULL` | Identidade lógica global da série de tarefa. |
| `task_series_segments` | `id PK` (`segment_id`), `task_id FK`, `effective_from_key`, `effective_until_key NULL`, `title`, `description NULL`, `priority`, `category_id FK NULL`, `schedule_id FK NULL`, `due_time NULL`, `kind`, `recurrence_rule_json NULL`, `created_at_utc` | Definição versionada; cronograma e categoria opcionais; tarefa sem cronograma permitida. Frequência diária/semanal/dias específicos. Mudanças futuras seguem ADR-003. |
| `task_occurrences` | `id PK` UUIDv5 de `series_id + original_local_key`, `task_id FK`, `segment_id FK`, `original_local_key TEXT`, `original_local_date DATE`, `original_local_time NULL`, `due_date DATE`, `due_time NULL`, `due_at_utc NULL`, `schedule_id_snapshot FK NULL`, `status`, snapshots de título/descrição/prioridade/categoria, `is_exception BOOL`, `exception_data_json NULL`, `completed_at_utc NULL`, `created_at_utc`, `updated_at_utc` | `UNIQUE(task_id, original_local_key)`; para tarefa sem hora a chave usa marcador `ALL_DAY`. Overrides/cancelamentos usam o mesmo modelo de exceção das atividades; tombstone impede regeneração. Vencimento não conclui automaticamente. |
| `reminders` | `id PK`, exatamente um de `activity_occurrence_id FK NULL`, `task_occurrence_id FK NULL`, `review_id FK NULL`, `offset_minutes`, `scheduled_at_utc`, `status`, `delivered_at_utc NULL`, `expired_at_utc NULL`, `group_id NULL`, `created_at_utc`, `updated_at_utc` | Estados `pending`, `delivered`, `expired`, `cancelled`; `CHECK` exige um alvo. Ao resume: pending vencido até 30 min é entregue uma vez; mais antigo expira e permanece consultável. Task permanece pendente. Agrupamento por tipo de alvo não apaga estados individuais. |
| `timer_sessions` | `id PK`, `process_instance_id`, `activity_occurrence_id FK NULL`, `status`, `accumulated_active_seconds`, `active_segment_started_at_utc NULL`, `last_checkpoint_at_utc NULL`, `interrupted_at_utc NULL`, `started_at_utc`, `ended_at_utc NULL`, `note NULL`, `created_at_utc`, `updated_at_utc` | Estados `running`, `paused`, `interrupted`, `completed`; uma running/paused por app via índice parcial; startup marca running de instância anterior interrupted; checkpoint <=5s; nunca acumula tempo pós-processo. |
| `timer_pause_segments` | `id PK`, `timer_session_id FK`, `started_at_utc`, `ended_at_utc NULL` | Histórico explícito de pausas; no máximo um segmento aberto por sessão; pause duration é soma de segmentos fechados. FK `RESTRICT`. |
| `pomodoro_sessions` | `id PK`, `activity_occurrence_id FK NULL`, `focus_seconds`, `short_break_seconds`, `long_break_seconds`, `long_break_every`, `completed_cycles`, `status`, `started_at NULL`, `ended_at NULL`, `created_at`, `updated_at` | Guardar snapshot de configuração usada; modelagem por ciclo ou por sessão precisa decidir. |
| `reviews` | `id PK`, `title`, `description NULL`, `local_date DATE`, `local_time NULL`, `scheduled_at_utc NULL`, `category_id FK NULL`, snapshots de categoria, `activity_occurrence_id FK NULL`, `related_content TEXT NULL`, `status`, `reminder_offset_minutes NULL`, `completed_at_utc NULL`, `created_at_utc`, `updated_at_utc`, `archived_at_utc NULL` | Revisões manuais; `scheduled_at_utc` nulo quando não há hora; snapshots preservam categoria. Associação de conteúdo mantém-se texto opcional conforme PRD. |
| `backup_history` | `id PK`, `path`, `kind ('manual'|'automatic')`, `created_at_utc`, `size_bytes NULL`, `status ('valid'|'failed'|'rotated')`, `integrity_result NULL`, `schema_version`, `error_summary NULL` | Retém sete automáticos; manuais nunca removidos pela rotação. Histórico registra sucesso/falha e status após rotação. |

## 2. Relacionamentos

```text
schedules 1 ── N activities 1 ── N activity_series_segments 1 ── N activity_occurrences
schedules 0..1 ── N task_series_segments; daily_tasks (series) 1 ── N task_series_segments 1 ── N task_occurrences
categories 1 ── N activity/task segments and reviews (global, ADR-002)
activity_occurrences 1 ── N timer_sessions ── N timer_pause_segments
activity_occurrences / task_occurrences / reviews 1 ── N reminders (exactly one target per reminder)
activity_occurrences 1 ── N pomodoro_sessions (association optional)
activity_occurrences 1 ── N reviews (optional)
```

## 3. Integridade, exclusão e índices

- Todas as tabelas de domínio usam PK UUID gerado no backend; FKs indexadas explicitamente.
- Usar `CHECK` para booleanos (`IN (0,1)`), prioridade/status/kind, duração não negativa, `end_time > start_time` e consistência temporal onde aplicável.
- Índices: `activity_occurrences(activity_id, original_local_key) UNIQUE`, `activity_occurrences(effective_local_date, starts_at_utc)`, `task_occurrences(task_id, original_local_key) UNIQUE`, `task_occurrences(due_date,status,priority_snapshot)`, timers por status, lembretes por `(status,scheduled_at_utc)`, revisões por `(local_date,status)`.
- Detecção de conflito compara sobreposição estrita de ocorrências ativas no mesmo cronograma; adjacência não conflita. Cronogramas diferentes não são comparados. Canceladas e históricas concluídas não bloqueiam. Aviso exige confirmação explícita para salvar. Não criar constraint SQL de exclusão: regra transacional Rust, conforme ADR-006.
- Usar `RESTRICT` em referências históricas; arquivar entidades com histórico/dependências; exclusão física só sem referências. Não definir `ON DELETE CASCADE` indiscriminadamente, conforme ADR-008.
- Migrations devem ser aditivas quando possível e preservar dados; backup pré-migração é sugestão operacional a validar.

## 4. Recorrência e ocorrências

O mecanismo próprio suporta diária, semanal (um ou mais dias da semana) e dias específicos, sem intervalos avançados RFC 5545. `activities.id` e `daily_tasks.id` são identidades estáveis `series_id`; tabelas `*_series_segments` guardam versões com intervalos efetivos ordenados sem sobreposição. Chave `original_local_key` canônica: data ISO + hora `HH:MM`; tarefa sem hora usa `YYYY-MM-DD#ALL_DAY`. PK é UUIDv5 de namespace fixo + série + chave; `UNIQUE(series_id, original_local_key)` é garantia adicional. Horário efetivo ajustado por DST/movimento nunca altera chave. Regeração não sobrescreve status/exceção; cancelamento mantém tombstone. “Esta e próximas” fecha segmento anterior antes da chave e cria novo no mesmo `series_id`; toda a série substitui regras futuras; futuras exceções explícitas são preservadas. Reconciliar ocorrências futuras não excepcionais sem duplicar. Aplicar separadamente a atividades e tarefas.

## 5. Dados derivados e lacunas

Dashboard e relatórios consultam ocorrências e sessões; não persistir agregados. Períodos são datas locais, início inclusivo/fim exclusivo. Agenda elegível = ocorrências não canceladas do período iniciadas até o instante de referência; período histórico inclui todas, futuro nenhuma. Tempo planejado concluído = soma da duração planejada das elegíveis concluídas. Cumprimento agenda = esse tempo / duração planejada elegível. Tarefas previstas = ocorrências datadas no período, incluindo datas futuras dentro dele, excluindo canceladas; sem data fica fora e é exibida separadamente. Cumprimento tarefas = ocorrências concluídas / ocorrências previstas. Denominador zero retorna sem dados. Sessão de tempo é atribuída integralmente ao período que contém seu início local. Fórmulas únicas no backend.

## 6. Refinamentos remanescentes do schema

1. Definir configuração exata de Pomodoro por sessão/ciclo e associação a atividade.
2. Enumerar constraints/valores permitidos de enums para status, prioridade e tipo de série na migration inicial.

## 7. Consistência de backup e restauração

Backup ativo deve produzir snapshot SQLite consistente com WAL por `VACUUM INTO` via SQLx, em arquivo temporário novo, validar integridade e só então publicar o arquivo final. O histórico armazena tipo, versão de schema e estado; ao rotacionar arquivo automático, atualizar registro para `rotated`, sem apagar histórico nem manuais.

Restore exige confirmação, validação no temporário (`integrity_check='ok'`, zero linhas de `foreign_key_check`) e schema compatível. Aplicar migrations suportadas na cópia antes do swap; rejeitar schema futuro. Bloquear escrita, executar checkpoint, fechar todas as conexões SQLx e tratar main/WAL/SHM como conjunto. Mover o conjunto atual para rollback, renomear candidato no mesmo volume e abrir pool somente após a substituição. Validar novamente após abrir; em falha fechar candidato, reverter o conjunto e reabrir o banco anterior. Persistir marcador de restore para recuperação após queda entre renames; eliminar rollback apenas após sucesso completo.

