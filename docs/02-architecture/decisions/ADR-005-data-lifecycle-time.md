# ADR-005: Fuso horário local, DST e continuidade temporal

- **Estado:** Aceita
- **Contexto:** agenda, recorrências, lembretes e timers dependem de uma referência temporal consistente.
- **Fuso:** usar o fuso local configurado no Windows, sem fuso por cronograma. Resolver datas civis/instantes centralmente no backend Rust. Atividades não atravessam a meia-noite; hora final deve ser posterior à inicial.
- **Persistência:** armazenar data/hora civil original e instante UTC resolvido. Ocorrências históricas preservam UTC; ocorrências futuras usam o fuso local vigente quando resolvidas.
- **Horário de verão:** recorrência em hora inexistente desloca ao primeiro horário local válido seguinte; hora duplicada usa a primeira ocorrência cronológica. Atividade pontual em hora inexistente é rejeitada. A chave de recorrência continua baseada no horário original, independentemente do ajuste DST.
- **Cronômetro:** continua durante minimização, tray e suspensão; pausas não contam. Em encerramento normal ou inesperado, sessão ativa fica `interrupted`; tempo após o fim do processo não conta. Na próxima abertura, oferecer retomar do acumulado ou finalizar. Crash recupera somente até o último checkpoint; intervalo de checkpoint limitado a cinco segundos. Persistir pausas como segmentos. Se o processo recebe suspend/resume, contabilizar o intervalo suspenso quando o timer estava rodando.
- **Consequências:** fuso alterado afeta resolução futura, mas não reescreve UTC histórico. No crash, o máximo não contabilizado dentro do processo é o intervalo desde o último checkpoint, até cinco segundos.
- **Impacto:** Fases 2–5; US-009–US-012, US-015–US-016, US-018–US-024.
