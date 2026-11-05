# Instruções do projeto — Life Planner

## Contexto

Life Planner é um aplicativo desktop pessoal, local-first e offline para Windows 10 22H2 e Windows 11 x64. O produto organiza cronogramas, atividades, tarefas, revisões, tempo e produtividade. O roadmap aprovado possui seis fases; a implementação deve seguir o backlog e as decisões documentadas.

## Leitura obrigatória antes de implementar

Antes de implementar uma história, consulte os documentos aplicáveis e confirme suas relações:

- `docs/01-product/PRD.md`
- `docs/01-product/requirements.md`
- `docs/02-architecture/architecture.md`
- `docs/02-architecture/database.md`
- `docs/02-architecture/decisions/README.md` e os ADRs relevantes em `docs/02-architecture/decisions/`
- `docs/03-planning/roadmap.md`
- `docs/03-planning/backlog.md`
- `docs/03-planning/progress.md`

O PRD define o produto; requisitos detalham regras verificáveis; arquitetura, modelo de dados e ADRs definem as fronteiras e decisões aceitas; roadmap, backlog e progress definem sequência e estado do trabalho. Em caso de conflito, não improvise uma regra: descreva a divergência e solicite aprovação antes de alterar escopo ou decisão arquitetural.

## Stack oficial

- Tauri 2
- Vue 3, TypeScript e Vite
- Tailwind CSS e shadcn-vue
- Pinia
- Rust
- SQLite com SQLx
- Vitest e Playwright, conforme a camada e as histórias

Não introduza tecnologia ou dependência alternativa sem justificativa e aprovação quando afetar decisões arquiteturais.

## Fronteiras da aplicação

- **Frontend Vue:** páginas, componentes, formulários, estado de apresentação e chamadas tipadas à API interna Tauri. O frontend nunca abre SQLite, lê/escreve o arquivo do banco ou implementa uma segunda fonte de verdade para regras de domínio.
- **Comandos Tauri:** fronteira fina de entrada/saída; validam formato/DTO, encaminham a casos de uso e retornam erros estáveis. Não concentram persistência nem regras complexas.
- **Serviços/casos de uso Rust:** coordenam operações de domínio, validações e transações.
- **Domínio Rust:** regras de negócio puras e modelos necessários aos módulos, sem dependência da interface.
- **Repositories Rust:** consultas e mutações SQLite parametrizadas, usando SQLx.
- **Banco:** SQLite local, migrations versionadas, integridade referencial e constraints conforme `database.md`.

SQLx é o único mecanismo de acesso ao SQLite pelo backend Rust, incluindo queries, transações e migrations. Não usar rusqlite, acesso SQLite direto ou plugin de banco paralelo. Todas as regras que protegem integridade, recorrência, conflitos, tempo e métricas devem ser autoritativas no backend.

## Organização, nomenclatura e tipagem

- Siga a estrutura modular e as responsabilidades estabelecidas em `architecture.md`; evite camadas genéricas ou abstrações sem necessidade concreta.
- Mantenha módulos por domínio e separe comandos, aplicação, domínio e infraestrutura no Rust.
- Use nomes descritivos e consistentes com o domínio e o schema existente; tabelas e colunas SQLite usam `snake_case`.
- Tipos e DTOs devem ser explícitos e serializáveis na fronteira Tauri. Datas/instantes seguem os formatos e regras de tempo da arquitetura; não converta horário civil em UTC no frontend.
- Validação no frontend pode melhorar feedback, mas a validação autoritativa fica no backend.
- Trate falhas com os códigos estáveis da arquitetura (`validation`, `not_found`, `conflict`, `storage`, `platform`) quando aplicáveis. Não exponha erros SQL brutos ao usuário; preserve contexto útil para diagnóstico.
- Mantenha capabilities Tauri mínimas e evite permissões de filesystem/shell além do necessário à história.

## Testes e qualidade

- Toda regra de negócio implementada deve ter testes adequados à camada: testes unitários Rust para regras puras; integração SQLite para migrations/repositories; Vitest para frontend isolado; Playwright para fluxos de UI quando aplicável.
- Cubra limites, estados inválidos, falhas e regressões relevantes aos critérios de aceite. Não substitua testes por validação visual manual quando a regra for automatizável.
- Não declare uma história concluída se os critérios de aceite ou testes correspondentes não forem verificados. Registre limitações de ambiente quando não for possível executar uma validação.

## Fluxo de trabalho e autorização

- Antes de implementar qualquer história, apresente um plano pequeno e revisável com escopo, arquivos prováveis, testes e riscos; aguarde aprovação explícita desse plano.
- Implemente somente a história explicitamente autorizada. Não avance para histórias dependentes ou posteriores sem autorização própria.
- Não altere decisões dos ADRs, requisitos aprovados ou escopo do produto sem aprovação explícita do responsável pelo produto. Quando uma lacuna impedir o trabalho, apresente alternativas e impactos antes de decidir.
- Mantenha alterações limitadas à história aprovada; não inclua refatorações ou funcionalidades não previstas.
- Ao concluir trabalho autorizado, atualize documentação afetada, backlog (critérios/status/dependências quando necessário) e `progress.md`. Preserve rastreabilidade entre PRD, requisitos, ADRs, arquitetura, banco e histórias.
- Não marque histórias como iniciadas/concluídas antes que o trabalho correspondente realmente comece/termine.
