# ADR-003: Recorrência própria e edição de ocorrências

- **Estado:** Aceita
- **Contexto:** atividades e tarefas podem repetir, mantendo ocorrências individuais e histórico. O escopo inicial inclui repetição diária, semanal e em dias específicos da semana.
- **Decisão:** implementar mecanismo próprio limitado, sem conformidade integral com RFC 5545, para recorrência diária, semanal e em dias específicos da semana. Aplicar regras separadas a séries de atividade e de tarefa; tarefas podem não possuir hora.
- **Identidade:** cada série tem `series_id` estável. Cada ocorrência usa chave composta pela série e data/hora civil originalmente prevista (`original_local_key`), antes de ajuste DST ou edição. A PK é UUIDv5 do namespace fixo, `series_id` e chave; constraint composta única é proteção adicional. Cancelamento fica como tombstone, impedindo regeneração.
- **Edição:** individual grava exceção pela chave original; “esta e próximas” fecha o segmento vigente antes da chave e cria novo segmento no mesmo `series_id`; “toda a série” substitui regras futuras. Reconciliar ocorrências futuras não excepcionais sem duplicar chaves. Ocorrências passadas, status, sessões e snapshots históricos não são reescritos.
- **Regras:** diário, semanal (um ou mais dias) e dias específicos. Segmentos possuem intervalos efetivos ordenados e sem sobreposição. Atividades e tarefas têm estrutura paralela; tarefa sem hora usa chave de data com marcador de dia inteiro.
- **Consequências:** a chave original mantém identidade mesmo com horário efetivo deslocado por DST ou ocorrência movida. Gerador determinístico e operações transacionais são testáveis.
- **Limites:** regras avançadas de RFC 5545 não são suportadas na v1.0.
- **Impacto:** Fases 2 e 3; `US-010`, `US-031`, `US-015` e `US-034`.

