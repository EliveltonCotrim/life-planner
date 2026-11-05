# LIFE PLANNER
## Documentação Funcional e Técnica
**Versão:** 1.0  
**Tipo:** Aplicativo desktop  
**Plataforma inicial:** Windows 10/11  
**Arquitetura:** Desktop local-first  
**Status:** Especificação inicial do projeto

---

# 1. Visão geral do projeto

## 1.1. Objetivo

O Life Planner é um aplicativo desktop de organização pessoal, planejamento de rotina e produtividade.

Seu objetivo é permitir que o usuário organize compromissos, atividades, tarefas e horários em um único ambiente, possibilitando acompanhar o que foi planejado e o que realmente foi realizado.

O sistema não será limitado a estudos ou a uma área específica da vida.

O usuário poderá organizar atividades de diferentes categorias, como:

- Estudos.
- Trabalho.
- Academia.
- Corrida.
- Saúde.
- Lazer.
- Atividades pessoais.
- Afazeres domésticos.
- Projetos.

A aplicação deverá proporcionar uma experiência simples, intuitiva e eficiente, permitindo organizar a rotina diária e semanal sem complexidade desnecessária.

## 1.2. Objetivos específicos

1. Permitir a criação de múltiplos cronogramas.
2. Organizar atividades por dias e horários.
3. Permitir a criação de categorias personalizadas.
4. Gerenciar tarefas e listas de afazeres diários.
5. Configurar lembretes e notificações.
6. Registrar o tempo efetivamente dedicado às atividades.
7. Disponibilizar cronômetro e Pomodoro.
8. Acompanhar indicadores de produtividade.
9. Gerenciar revisões e compromissos futuros.
10. Permitir backup e restauração dos dados.
11. Exportar cronogramas e relatórios em PDF.

---

# 2. Conceitos fundamentais

O sistema deverá trabalhar com quatro conceitos principais.

## 2.1. Cronograma

Representa uma organização da rotina.

Exemplos:
- Minha rotina semanal.
- Rotina de trabalho.
- Rotina de férias.
- Planejamento de estudos.

O usuário poderá manter vários cronogramas e selecionar qual deseja utilizar.

## 2.2. Categoria

Representa a classificação de uma atividade.

Exemplos:
- Estudos.
- Trabalho.
- Saúde.
- Pessoal.
- Lazer.
- Outros.

As categorias serão personalizáveis, permitindo definir nome, cor e ícone.

## 2.3. Atividade

Representa algo que possui um horário reservado na agenda.

Exemplos:
- Academia: segunda-feira, 06h às 07h.
- Trabalho: segunda-feira, 08h às 12h.
- Programação: segunda-feira, 19h às 21h.

Uma atividade poderá ser recorrente ou pontual.

## 2.4. Tarefa

Representa algo que precisa ser realizado, mas que não necessariamente possui horário definido.

Exemplos:
- Regar as plantas.
- Organizar documentos.
- Fazer compras.
- Estudar determinado assunto.
- Resolver uma pendência.

As tarefas poderão ser associadas a uma data, prioridade e lembrete.

As tarefas são globais e poderão ser associadas opcionalmente a um cronograma; uma tarefa também poderá existir sem cronograma.

**Diferença fundamental:** atividades organizam o tempo na agenda; tarefas organizam aquilo que precisa ser feito.

---

# 3. Requisitos funcionais

## RF01 — Gerenciamento de cronogramas

O sistema deverá permitir:

- Criar cronogramas.
- Editar cronogramas.
- Excluir cronogramas.
- Duplicar cronogramas.
- Arquivar cronogramas.
- Definir cronograma principal.
- Definir datas de início e término.
- Visualizar cronogramas anteriores.

Cada cronograma deverá possuir suas próprias atividades e configurações.

## RF02 — Gerenciamento de categorias

O sistema deverá permitir:

- Cadastrar categorias.
- Editar categorias.
- Excluir categorias sem dependências.
- Definir cores personalizadas.
- Definir ícones.
- Ativar e desativar categorias.

Categorias iniciais sugeridas:

| Categoria | Cor |
|---|---|
| Estudos | Azul |
| Trabalho | Roxo |
| Saúde | Verde |
| Pessoal | Amarelo |
| Lazer | Rosa |
| Outros | Cinza |

## RF03 — Gerenciamento de atividades

Cada atividade deverá possuir:

- Nome.
- Descrição.
- Categoria.
- Cronograma associado.
- Horário inicial.
- Horário final.
- Duração prevista.
- Dias de recorrência.
- Data de início.
- Data de término opcional.
- Lembrete opcional.
- Status.

Funcionalidades:

- Criar atividades.
- Editar atividades.
- Excluir atividades.
- Duplicar atividades.
- Arrastar e reorganizar atividades na agenda.
- Copiar atividades entre dias.
- Criar atividades recorrentes.
- Criar atividades pontuais.
- Consultar histórico.

O sistema deverá identificar conflitos de horários.

## RF04 — Planejamento semanal

O sistema deverá apresentar uma agenda visual organizada por dias da semana e horários.

Funcionalidades:

- Visualização semanal.
- Navegação entre semanas.
- Identificação visual por categoria.
- Cadastro rápido de atividades.
- Edição dos horários.
- Reorganização por arrastar e soltar.
- Consulta de carga horária planejada.
- Identificação de horários livres.

O planejamento será baseado em horários fixos definidos manualmente pelo usuário.

## RF05 — Gerenciamento de tarefas diárias

O sistema deverá possuir um módulo independente para gerenciamento de afazeres.

Cada tarefa poderá possuir:

- Título.
- Descrição.
- Data.
- Prioridade.
- Categoria opcional.
- Prazo opcional.
- Horário opcional.
- Lembrete.
- Status.
- Recorrência.

Funcionalidades:

- Criar tarefas.
- Editar tarefas.
- Excluir tarefas.
- Marcar tarefas como concluídas.
- Reagendar tarefas.
- Criar tarefas recorrentes.
- Visualizar tarefas por dia.
- Visualizar tarefas atrasadas.
- Filtrar por prioridade e status.

As tarefas deverão permitir repetição diária, semanal ou em dias específicos da semana.

Tarefas não concluídas não deverão ser consideradas automaticamente realizadas quando sua data passar.

O usuário deverá poder reagendá-las.

## RF06 — Cronômetro

O sistema deverá disponibilizar um cronômetro para registrar o tempo efetivamente dedicado às atividades.

Funcionalidades:

- Iniciar.
- Pausar.
- Retomar.
- Finalizar.
- Associar a uma atividade.
- Registrar duração.
- Consultar histórico de sessões.

O cronômetro deverá preservar sua referência temporal mesmo que o aplicativo seja minimizado ou temporariamente suspenso.

O tempo deverá ser calculado utilizando referências temporais confiáveis, não apenas incrementando segundos enquanto a interface estiver aberta.

Após encerramento inesperado, a sessão será marcada como interrompida e o período em que o aplicativo ficou encerrado não será contabilizado automaticamente. O usuário poderá retomar o tempo acumulado ou finalizar a sessão.

## RF07 — Pomodoro

O sistema deverá disponibilizar um temporizador baseado na técnica Pomodoro.

Configuração inicial:

- 25 minutos de foco.
- 5 minutos de descanso.
- 15 minutos de descanso longo após quatro ciclos.

O usuário poderá personalizar os intervalos.

Funcionalidades:

- Iniciar ciclo.
- Pausar ciclo.
- Retomar ciclo.
- Finalizar ciclo.
- Controlar quantidade de ciclos.
- Notificar início e término das etapas.
- Registrar sessões concluídas.

## RF08 — Notificações e lembretes

O sistema deverá disponibilizar notificações nativas do Windows.

Tipos de notificações:

- Início de atividade.
- Lembrete de tarefa.
- Tarefa atrasada.
- Início e término de Pomodoro.
- Intervalos de descanso.
- Revisões programadas.

O usuário poderá personalizar antecedência e habilitar ou desabilitar notificações por categoria.

O aplicativo deverá possuir integração com a bandeja do sistema (System Tray), permitindo permanecer em segundo plano.

A inicialização automática com o Windows deverá ser opcional.

Fechar a janela manterá o processo ativo na bandeja. Notificações funcionarão enquanto o processo estiver ativo; sair explicitamente encerrará notificações. Não haverá integração com o Agendador de Tarefas do Windows na v1.0.

Após o retorno de suspensão, notificar individualmente lembretes vencidos há até 30 minutos. Lembretes mais antigos serão marcados como expirados, continuarão consultáveis e não alterarão o estado de tarefas atrasadas. Notificações semelhantes poderão ser agrupadas.

## RF09 — Gerenciamento de revisões

O sistema deverá permitir cadastrar revisões manualmente.

Cada revisão poderá possuir:

- Título.
- Descrição.
- Data.
- Horário opcional.
- Categoria.
- Atividade ou conteúdo relacionado.
- Status.
- Lembrete.

Funcionalidades:

- Cadastrar revisão.
- Editar revisão.
- Excluir revisão.
- Marcar como concluída.
- Visualizar revisões futuras.
- Visualizar revisões atrasadas.

As revisões não serão geradas automaticamente na versão inicial.

## RF10 — Dashboard

A página inicial deverá apresentar um resumo da rotina.

Indicadores:

- Atividades do dia.
- Próximas atividades.
- Horas planejadas.
- Horas realizadas.
- Tarefas pendentes.
- Tarefas concluídas.
- Tarefas atrasadas.
- Percentual de cumprimento diário.

O dashboard e os relatórios deverão separar tempo registrado, tempo planejado concluído, cumprimento da agenda e cumprimento de tarefas. As fórmulas, períodos e critérios de elegibilidade são definidos na especificação de requisitos e ADR-007.

## RF11 — Relatórios

O sistema deverá disponibilizar relatórios por período.

Relatórios previstos:

- Produtividade diária.
- Produtividade semanal.
- Produtividade mensal.
- Horas por categoria.
- Horas planejadas versus realizadas.
- Tarefas concluídas e pendentes.
- Atividades não realizadas.
- Histórico de cronogramas.

## RF12 — Backup e restauração

O sistema deverá permitir:

- Criar backup manual.
- Configurar backup automático.
- Escolher diretório de armazenamento.
- Consultar histórico de backups.
- Restaurar backup.
- Validar integridade do arquivo.

A restauração deverá exigir confirmação antes de substituir os dados atuais.

Backups automáticos serão gerados na abertura se tiver passado pelo menos um dia desde o último backup automático válido. Backup manual não reinicia esse intervalo. O destino padrão será pasta própria no diretório de dados, com opção de escolher outro diretório. Reter sete backups automáticos; a rotação não remove backups manuais.

## RF13 — Exportação PDF

O sistema deverá permitir exportar:

- Cronograma semanal.
- Lista diária de tarefas.
- Relatório de produtividade.
- Resumo mensal.
- Relatório por categoria.

Os documentos deverão apresentar informações organizadas e adequadas para impressão.

## RF14 — Configurações gerais

O sistema deverá permitir configurar:

- Tema claro, escuro ou conforme o sistema.
- Preferências básicas disponíveis na v1.0.
- Notificações.
- Configurações do Pomodoro.
- Diretório de backup.
- Inicialização com Windows.
- Preferências de visualização.

O idioma será português fixo na v1.0, sem seletor de idioma. Horários de funcionamento ficam fora do escopo da v1.0.

---

# 4. Regras de negócio

| Código | Regra |
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

Regras complementares aprovadas; os códigos RN01–RN16 acima permanecem inalterados:

| Código | Regra |
|---|---|
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

---

# 5. Requisitos não funcionais

| Código | Requisito |
|---|---|
| RNF01 | Aplicativo desktop para Windows 10 22H2 e Windows 11, arquitetura x64. |
| RNF02 | Funcionamento offline. |
| RNF03 | Interface em português. |
| RNF04 | Tema claro, escuro e sistema. |
| RNF05 | Persistência local utilizando SQLite. |
| RNF06 | Interface intuitiva e responsiva. |
| RNF07 | Arquitetura modular. |
| RNF08 | Validação dos dados no backend. |
| RNF09 | Proteção contra perda de dados. |
| RNF10 | Testes automatizados. |
| RNF11 | Backup e restauração confiáveis. |
| RNF12 | Instalador NSIS; dados preservados fora do diretório de instalação; atualização manual; assinatura digital exigida em distribuição pública. |

---

# 6. Arquitetura tecnológica

## 6.1. Stack

| Camada | Tecnologia |
|---|---|
| Desktop | Tauri 2 |
| Frontend | Vue 3 |
| Linguagem frontend | TypeScript |
| Build | Vite |
| Estilização | Tailwind CSS |
| Componentes | shadcn-vue |
| Estado | Pinia |
| Backend nativo | Rust |
| Banco de dados | SQLite |
| Acesso a dados | SQLx (exclusivo para operações SQLite do backend Rust) |
| Validação frontend | Zod |
| Testes frontend | Vitest |
| Testes de interface | Playwright |
| Testes backend | Rust Tests |

## 6.2. Arquitetura em camadas

A aplicação será dividida em:

**Frontend**
- Páginas.
- Componentes.
- Stores.
- Formulários.
- Validações de interface.
- Visualizações.

**Camada de aplicação**
- Casos de uso.
- Regras de negócio.
- Validações.
- Serviços.
- Orquestração das operações.

**Infraestrutura**
- Repositórios.
- Persistência SQLite.
- Notificações.
- Gerenciamento de arquivos.
- Backup.
- Exportação PDF.

O frontend deverá se comunicar com o backend por comandos Tauri.

Todas as operações SQLite do backend Rust deverão utilizar exclusivamente SQLx. O frontend Vue não acessará o banco diretamente.

As regras de negócio importantes não deverão ficar espalhadas pelos componentes de interface.

## 6.3. Estrutura de diretórios

```text
life-planner/
│
├── src/
│   ├── app/
│   │   ├── router/
│   │   └── stores/
│   │
│   ├── components/
│   │   ├── ui/
│   │   ├── layout/
│   │   └── shared/
│   │
│   ├── modules/
│   │   ├── dashboard/
│   │   ├── schedules/
│   │   ├── categories/
│   │   ├── activities/
│   │   ├── tasks/
│   │   ├── calendar/
│   │   ├── timers/
│   │   ├── pomodoro/
│   │   ├── reminders/
│   │   ├── reviews/
│   │   ├── reports/
│   │   └── settings/
│   │
│   ├── services/
│   ├── composables/
│   ├── types/
│   └── styles/
│
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   ├── application/
│   │   ├── domain/
│   │   ├── infrastructure/
│   │   └── main.rs
│   │
│   ├── migrations/
│   ├── capabilities/
│   └── tauri.conf.json
│
├── tests/
│
├── package.json
└── README.md
```

---

# 7. Modelagem inicial do banco de dados

O banco deverá utilizar SQLite.

## 7.1. Entidades principais

| Tabela | Descrição |
|---|---|
| schedules | Cronogramas |
| categories | Categorias |
| activities | Definições das atividades |
| activity_series_segments | Versões das regras/detalhes das séries de atividades |
| activity_occurrences | Ocorrências das atividades |
| daily_tasks | Definições das tarefas |
| task_series_segments | Versões das regras/detalhes das séries de tarefas |
| task_occurrences | Ocorrências das tarefas |
| reminders | Lembretes |
| timer_sessions | Sessões de cronômetro |
| timer_pause_segments | Segmentos de pausa do cronômetro |
| pomodoro_sessions | Sessões Pomodoro |
| reviews | Revisões |
| study_courses | Cursos opcionais |
| study_contents | Conteúdos opcionais |
| app_settings | Configurações |
| backup_history | Histórico de backups |

## 7.2. Relacionamentos

- Um cronograma possui várias atividades.
- Uma categoria pode classificar várias atividades.
- Uma atividade pode gerar várias ocorrências.
- Tarefas são globais, podem existir sem cronograma e podem ser associadas opcionalmente a um cronograma.
- Uma tarefa pode gerar várias ocorrências.
- Uma ocorrência pode possuir vários lembretes.
- Uma atividade pode possuir várias sessões de cronômetro.
- Uma atividade pode possuir revisões relacionadas.

## 7.3. Estratégia de recorrência

As definições de atividades e tarefas recorrentes deverão ser separadas de suas ocorrências.

Exemplo:

Atividade:
- Nome: Academia.
- Recorrência: segunda, quarta e sexta.
- Horário: 06h às 07h.

Ocorrências:
- Segunda-feira, 06h às 07h.
- Quarta-feira, 06h às 07h.
- Sexta-feira, 06h às 07h.

Cada ocorrência deverá possuir seu próprio status e histórico.

Alterações na recorrência deverão preservar as ocorrências anteriores e permitir exceções em datas específicas.

Cada série possui identidade estável e segmentos versionados. A identidade da ocorrência deriva unicamente da série e da data/hora civil originalmente prevista; ajustes de DST ou edições não alteram essa chave. Geração idempotente não recria ocorrências canceladas. Para editar, o usuário pode escolher somente a ocorrência, esta e as próximas, ou as ocorrências futuras da série, sem modificar ocorrências/sessões passadas.

---

# 8. Interface e navegação

A navegação principal deverá conter:

1. Dashboard.
2. Cronogramas.
3. Agenda semanal.
4. Categorias.
5. Atividades.
6. Tarefas.
7. Cronômetro.
8. Pomodoro.
9. Revisões.
10. Relatórios.
11. Configurações.

## Diretrizes visuais

- Interface moderna e minimalista.
- Tema claro, escuro ou conforme o sistema.
- Cores consistentes por categoria.
- Navegação lateral.
- Dashboard com indicadores.
- Agenda semanal visual.
- Formulários simples.
- Ações rápidas para adicionar atividades e tarefas.
- Notificações discretas.

A experiência deverá priorizar a facilidade de utilização e reduzir a quantidade de etapas para cadastrar e concluir tarefas.

---

# 9. Plano de desenvolvimento

O desenvolvimento será organizado em seis fases.

## Fase 1 — Fundação

- Criar projeto Tauri.
- Configurar Vue 3 e TypeScript.
- Configurar Tailwind CSS.
- Implementar layout principal.
- Implementar tema claro, escuro ou conforme o sistema.
- Configurar SQLite.
- Criar migrations.
- Implementar configurações gerais.

## Fase 2 — Organização da rotina

- Gerenciamento de cronogramas.
- Gerenciamento de categorias.
- Cadastro de atividades.
- Atividades recorrentes.
- Agenda semanal.
- Reorganização de horários.
- Identificação de conflitos.

## Fase 3 — Tarefas e notificações

- Cadastro de tarefas.
- Listas diárias.
- Tarefas recorrentes.
- Prioridades.
- Prazos.
- Lembretes.
- Notificações Windows.
- Gerenciamento de tarefas atrasadas.

## Fase 4 — Controle de tempo

- Cronômetro.
- Pomodoro.
- Registro de sessões.
- Associação de sessões às atividades.
- Histórico de tempo.
- Comparação entre tempo planejado e realizado.

## Fase 5 — Produtividade

- Dashboard.
- Indicadores.
- Revisões.
- Relatórios.
- Gráficos.
- Exportação PDF.

## Fase 6 — Finalização

- Backup manual.
- Backup automático.
- Restauração.
- Testes de integração.
- Testes de interface.
- Otimização.
- Instalador Windows.
- Documentação de instalação.

---

# 10. Critérios gerais de aceite

A primeira versão será considerada funcional quando:

1. For possível criar e gerenciar vários cronogramas.
2. For possível cadastrar categorias personalizadas.
3. For possível criar atividades com horários e recorrências.
4. A agenda semanal apresentar corretamente as atividades.
5. O sistema identificar conflitos de horários.
6. For possível cadastrar tarefas diárias e recorrentes.
7. As tarefas puderem ser concluídas e reagendadas.
8. Os lembretes funcionarem conforme as configurações.
9. O cronômetro registrar corretamente o tempo.
10. O Pomodoro funcionar com intervalos configuráveis.
11. Os registros históricos forem preservados.
12. Os relatórios apresentarem dados consistentes.
13. A exportação PDF funcionar.
14. O backup puder ser criado e restaurado.
15. O aplicativo funcionar offline.

---

# 11. Fora do escopo inicial

Para evitar complexidade desnecessária, não serão implementados inicialmente:

- Sincronização em nuvem.
- Aplicativo Android ou iOS.
- Acesso multiusuário.
- Login e autenticação.
- Integração com Google Calendar.
- Integração com serviços externos.
- Inteligência artificial para planejamento automático.
- Distribuição automática de atividades por horários.

Esses recursos poderão ser considerados em versões futuras.

---

# 12. Diretrizes de desenvolvimento

O desenvolvimento deverá seguir os seguintes princípios:

- Código modular.
- Separação entre interface, regras de negócio e persistência.
- Componentes reutilizáveis.
- Tipagem forte com TypeScript.
- Validação de dados no frontend e backend.
- Migrations versionadas.
- Testes automatizados.
- Tratamento adequado de erros.
- Persistência segura.
- Compatibilidade com Windows.
- Documentação das decisões arquiteturais.

A implementação deverá priorizar funcionalidades completas, evitando criar telas que não estejam conectadas às operações reais do sistema.

---

# 13. Considerações finais

O Life Planner será um sistema de organização pessoal abrangente, cujo principal objetivo é centralizar a rotina do usuário.

A aplicação deverá permitir organizar o tempo por meio de atividades, administrar afazeres por meio de tarefas e acompanhar a execução utilizando cronômetros, Pomodoro, relatórios e indicadores.

A arquitetura local-first com Tauri, Vue, Rust e SQLite permitirá construir um aplicativo desktop independente de servidores externos.

**Diretriz principal do produto:** simplicidade na utilização, flexibilidade na organização e confiabilidade no armazenamento dos dados.

**Versão do documento:** 1.0  
**Projeto:** Life Planner  
**Status:** Especificação funcional e técnica inicial.
