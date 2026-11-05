# ADR-009: Política de backup

- **Estado:** Aceita
- **Contexto:** aplicação local precisa oferecer cópia e restauração sem armazenamento em nuvem.
- **Decisão:** suportar backup manual e automático diário. Na abertura, criar backup automático se já passou pelo menos um dia desde o último backup automático válido; backup manual não reinicia esse intervalo. Destino padrão é pasta própria dentro do diretório de dados; permitir escolher outra pasta. Reter sete automáticos; rotação nunca remove manuais. Sem nuvem.
- **Criação:** executar backup consistente por meio do SQLite `VACUUM INTO` pela conexão SQLx exclusiva, ou mecanismo equivalente exposto por SQLx que inclua o estado lógico consolidado mesmo em WAL. Escrever em arquivo temporário novo, validar e só então publicar com nome final. Não copiar somente o arquivo principal enquanto SQLite estiver aberto.
- **Restauração:** pedir confirmação; validar em arquivo temporário `PRAGMA integrity_check = 'ok'`, ausência de linhas em `PRAGMA foreign_key_check` e compatibilidade da versão/migrations do schema. Bloquear gravações, fechar pool/conexões SQLx, mover o banco atual para rollback, tratar/remover WAL/SHM somente após o fechamento, trocar o arquivo, abrir novo pool e validar. Se qualquer etapa falhar, restaurar o arquivo anterior e reabrir o banco original. Não apagar rollback antes da abertura bem-sucedida.
- **Consequências:** registrar tipo, data, integridade, versão de schema e caminho; falha de destino não afeta banco ativo e deve ser informada. Verificar espaço antes da criação; remover arquivo temporário incompleto após falha, preservando apenas o resumo do erro.
- **Referências técnicas:** [SQLite VACUUM INTO](https://www.sqlite.org/lang_vacuum.html), [SQLite PRAGMA integrity_check e foreign_key_check](https://www.sqlite.org/pragma.html#pragma_integrity_check), [SQLx SqliteConnection](https://docs.rs/sqlx/latest/sqlx/sqlite/struct.SqliteConnection.html).
- **Impacto:** Fase 6; `US-026` a `US-028`, `US-029`.

