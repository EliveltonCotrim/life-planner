# ADR-010: Distribuição e atualização Windows

- **Estado:** Aceita
- **Contexto:** a v1.0 precisa de instalação Windows simples, mantendo dados locais fora do pacote do aplicativo.
- **Decisão:** suporte a Windows 10 22H2 e Windows 11, x64. Utilizar instalador NSIS inicialmente. Atualizações da v1.0 serão manuais, via nova versão do instalador. Armazenar dados do usuário fora do diretório de instalação. Desinstalação não remove dados silenciosamente. Assinatura digital é requisito para distribuição pública e não é obrigatória no ambiente de desenvolvimento.
- **Consequências:** upgrade não deve sobrescrever/remover banco; documentar preservação e localização dos dados. Caso a desinstalação ofereça remoção de dados, deve ser uma opção explícita e confirmada.
- **Impacto:** Fase 6; `US-029`, `US-030`; bandeja/startup de `US-017` deve ser validada no instalador NSIS.

