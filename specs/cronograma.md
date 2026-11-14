# Curso de Rust — cronograma de estudo e prática

## Objetivo e fontes

Aprender Rust construindo, testando e explicando programas com autonomia. O [Rust Book](https://doc.rust-lang.org/book/) orienta os conceitos e os projetos; [Rust by Example](https://doc.rust-lang.org/rust-by-example/) amplia os exemplos; [Rustlings](https://github.com/rust-lang/rustlings/) oferece exercícios de correção e implementação. Usar o [mapa oficial de tópicos](https://github.com/rust-lang/rustlings/blob/main/exercises/README.md) para selecionar exercícios compatíveis com os pré-requisitos de cada aula. Cobrir um tópico não significa que todo exercício dessas fontes já foi realizado.

O percurso tem **22 módulos e 176 aulas**, além de **11 revisões práticas**. Não existe cota fixa de aulas por módulo. M01–M17 formam o núcleo com projeto de integração; M18 acrescenta async e M19–M22 são aprofundamentos que podem ser escolhidos conforme o objetivo. Esses conteúdos continuam disponíveis no percurso completo, mas não bloqueiam a entrega do servidor.

Conceitos podem voltar em problemas novos, refatorações e revisões. Cada retomada deve indicar a nova habilidade ou comparação que será praticada; não é necessário repetir a explicação inteira.

## Preservação do trabalho existente e retomada

- Atualizar este documento não autoriza alterar, renomear, mover ou recriar projetos já feitos pelo aluno. Os arquivos existentes em `aulas/` permanecem como histórico de aprendizagem, mesmo quando a sequência futura muda.
- Os identificadores e diretórios das aulas antigas de M01–M04 foram mantidos. As descrições podem melhorar para novos alunos. Foram acrescentadas M01-A10 e M02-A09; não presumir que o aluno atual já as concluiu.
- Neste checkout há projetos até M04-A03; a existência da pasta não comprova conclusão. Retomar da última aula confirmada pelo aluno, sem mandar refazer os módulos iniciais. Antes de novas integrações, oferecer os complementos de ferramentas (M01-A10) e testes (M02-A09), em suas novas pastas; uma demonstração de domínio pode dispensar conteúdo já conhecido.
- Usar R01 e R02 como diagnóstico breve de conteúdos anteriores quando necessário. Corrigir lacunas com exercícios novos, sem editar os projetos históricos. A próxima integração de slices, M04-A05, usa tipos concretos.
- Para alunos que tenham avançado na versão anterior: M11 mantém lifetimes básicos; tipos associados passam a M10-A14; os demais assuntos avançados de M11 passam a M19. O antigo M17 passa a M16; o servidor antigo M20 passa a M17; async antigo M16 passa a M18; macros antigas M18 passam a M20 e sua documentação a M09-A06; unsafe antigo M19 passa a M21 e manutenção/desempenho a M22. Identificar equivalência pelo conteúdo, sem migrar pastas automaticamente.

## Ritmo, esforço e critérios de avanço

Cada bloco corresponde a **45–60 minutos de estudo ativo**, incluindo implementação e verificação pelo aluno. A coluna **Nível / blocos** combina dificuldade estimada e esforço: **B** = básico, **I** = intermediário, **A** = avançado; o número é a quantidade inicial de blocos. Uma aula pode ocupar vários encontros. São estimativas de planejamento, não prazos ou evidência de domínio; recalibrar depois das primeiras cinco aulas e sempre que houver dificuldade recorrente.

O núcleo M01–M17 soma **241 blocos de aulas + 18 de revisões**. O percurso completo soma **327 + 25 blocos**, aproximadamente **264–352 horas**, antes de recuperação de lacunas. Não converter automaticamente isso em uma data de conclusão. Como exemplo adaptável, quatro blocos semanais podem reservar três para conteúdo/prática e um para recuperação ativa; quando chegar uma revisão formal, ela ocupa os blocos previstos de estudo, sem se somar como obrigação extra.

No início de cada encontro, pedir uma previsão ou explicação curta sobre um conceito anterior, sem código pronto. Ao final de cada módulo, aplicar o critério de domínio indicado. Para avançar, o aluno deve:

1. Entregar o comportamento descrito na aula e verificar casos normais, limites e erros relevantes; a partir de M02-A09, usar testes quando houver lógica testável.
2. Explicar as decisões centrais e resolver uma pequena variação sem receber a solução completa. Compilar ou copiar um exemplo, isoladamente, não comprova domínio.
3. Ler os diagnósticos, aplicar formatação e tratar avisos pertinentes. Usar `cargo check`, `cargo test` e Clippy conforme o projeto, sem tornar todos os comandos uma obrigação mecânica em todo exemplo.
4. Registrar o que foi feito, como foi verificado e o que ainda exige ajuda. O aluno pode manter esse registro em notas ou no README; o tutor não escreve nos arquivos por ele.

Se um critério não for alcançado, propor um exercício focalizado, dar dicas graduais e verificar novamente antes de aumentar a complexidade. Não reiniciar o módulo inteiro por um erro pontual. Nas revisões, consultar assinaturas e documentação é permitido; começar sem a solução do tutor e discutir a solução depois da tentativa.

## Roteiros didáticos obrigatórios de cada aula

Cada aula possui um roteiro detalhado em [specs/roteiros](roteiros/README.md), acessível pelo link **roteiro** na própria linha da tabela. O tutor deve ler o [padrão didático](roteiros/README.md) e o roteiro específico antes de conduzir a aula. A coluna de conteúdo é um resumo; não substitui o roteiro.

Os roteiros fixam: conceitos a explicar, programa guiado a escrever na conversa, dados e comportamento, percurso do código a demonstrar, contraste/erro obrigatório, casos com resultados esperados e exercício independente. A explicação deve ser **didática e detalhada**, definindo os termos, explicando sintaxe, tipos e decisões e acompanhando a execução com valores concretos. Não basta listar tópicos ou entregar código sem explicar por que funciona.

**O agente escreve o exemplo guiado na conversa; o aluno escreve nos arquivos e executa os comandos.** Fornecer o código completo necessário ao exemplo em etapas explicadas, sem omitir a lógica principal. O exercício independente continua sem gabarito antecipado. Nomes, dados, regras e casos obrigatórios são comuns a todos os tutores; analogias e ritmo podem variar. Os roteiros reduzem diferenças de conteúdo sem prometer respostas idênticas entre modelos.

Para usar o curso com outro tutor, compartilhar este arquivo **junto da pasta `specs/roteiros/`**, preservando os caminhos relativos, ou compartilhar o repositório. Enviar somente o resumo do cronograma deixa de fora as instruções detalhadas. Exemplo de pedido de aula:

> Quero estudar M04-A04. Leia `specs/cronograma.md`, `specs/roteiros/README.md` e o roteiro M04-A04 em `specs/roteiros/m04.md`. Explique didaticamente e em detalhes, escreva o exemplo guiado na conversa em etapas e siga os casos obrigatórios. Eu digito nos arquivos e executo os comandos. Não antecipe o gabarito do exercício independente nem avance automaticamente para outra aula.

## Regras do tutor e formato de cada aula

1. **O aluno escreve o código e executa os comandos.** O agente não cria projetos, instala ferramentas/dependências, altera manifests, executa formatação/correções nem escreve nos arquivos do aluno. Apresenta os comandos e orientações em etapas curtas para o aluno executar. Manter este cronograma, quando solicitado, é uma tarefa de planejamento e não autoriza modificar as aulas.
2. **Validação pelo agente só após pedido.** Quando solicitada, pode revisar o código compartilhado e executar somente verificações autorizadas, como `cargo check`, `cargo test` ou Clippy sem `--fix`. Não aplica correções. Comandos que alteram fonte, como `cargo fmt` e `cargo fix`, continuam a cargo do aluno. Ler este cronograma ou fazer sua manutenção não exige executar projetos Rust.
3. **Abertura da aula:** informar módulo, aula, título, objetivo, pré-requisitos, nível/esforço e nome/caminho do projeto. Para aluno novo, confirmar a raiz local do curso; neste checkout ela é `/home/amsterdan/developer/learn-rust`. Usar o caminho real do aluno, absoluto e sem placeholders. Se faltar Rust, orientar a instalação oficial antes do primeiro projeto.
4. **Criação de projetos:** cada aula usa um diretório independente, exceto o projeto contínuo explicitamente indicado no M17. A pasta é `aulas/mNN/aNN-<slug>`, sem repetir `mNN-` no nome. Em geral ela contém um pacote; M09-A08 e M20-A07–A09 usam um workspace local com membros próprios. Mostrar `cargo new --bin --edition 2024 --vcs none` ou a variante `--lib`, com caminho absoluto, e um `cd` absoluto. Para workspaces, explicitar também o manifest da raiz e os caminhos de cada membro. Não reinicializar Git dentro de cada aula. Se a pasta já existir, inspecionar o contexto de retomada e não sobrescrevê-la nem recriá-la.
5. **Projetos contínuos:** apenas a primeira etapa cria o pacote. Nas seguintes, informar o mesmo caminho absoluto, as partes que serão alteradas e o comportamento/testes a preservar. O aluno aplica e verifica cada mudança; nunca reapresentar todo o servidor como uma obrigação de redigitação.
6. **Ensino em três momentos:** começar com exemplo pequeno e guiado; propor uma variação com dicas graduais; encerrar com tarefa curta sem solução pronta. No exercício independente, mostrar primeiro o enunciado, entradas/saídas e critérios de aceite. Oferecer assinatura ou esqueleto apenas quando ajudar, e liberar solução completa depois da tentativa ou se o aluno pedir. Pedir previsões antes de compilar exemplos válidos e inválidos.
7. **Autoria nos exercícios:** o tutor deve escrever na conversa o código do exemplo guiado definido no roteiro e explicá-lo em etapas; pode também explicar uma correção solicitada. Não entregar antecipadamente todos os arquivos e a solução do desafio. Se uma integração independente precisar de código-base, fornecer apenas a infraestrutura já conhecida em partes pequenas; o aluno escreve a lógica nova. Não depender de arquivos ocultos, de cópia de outra pasta ou de um workspace compartilhado entre aulas.
8. **Escopo e retomadas:** **Novo** introduz conceito; **Integração** combina conhecimentos; **Expansão** aprofunda um aspecto. Uma retomada pode comparar soluções ou refatorar código, desde que declare seu objetivo novo. Usar poucas frases para relembrar pré-requisitos. A coluna de conteúdo/entrega e o roteiro específico formam o contrato da aula; o roteiro detalha a explicação e o código obrigatórios.
9. **Ferramentas e dependências:** usar Rust 2024 e stable como padrão. Antes de propor crate, runtime, rede, compilador C ou nightly, explicar o propósito, consultar a documentação correspondente e informar versão usada e alterações exatas no manifest para o aluno digitar. A edição não fixa a versão do compilador: registrar o toolchain usado. Snippets nightly ficam identificados e separados do projeto stable; o curso não exige publicar pacotes ou serviços externos.
10. **Exercícios das fontes:** selecionar exercícios Rustlings e exemplos do Book/Rust by Example pelo tema da aula, registrar o identificador escolhido e conferir pré-requisitos. Não supor que a ordem inteira do Rustlings coincide com a deste curso. Os arquivos originais dos exercícios podem ser editados pelo aluno, sob a mesma regra de autoria; eles não substituem a entrega prática da aula.
11. **Encerramento e ritmo:** informar como o aluno verifica a entrega, pedir uma explicação ou variação e registrar lacunas na conversa. Não avançar automaticamente para outra aula; aguardar o aluno. Nas aulas iniciais, explicar o uso mínimo de uma API antes de aprofundar sua implementação: usar `println!`, `parse`, atributos de teste ou macros prontas não exige saber implementá-los.

Exemplo de abertura de um pacote novo neste checkout:

```sh
cargo new --bin --edition 2024 --vcs none /home/amsterdan/developer/learn-rust/aulas/m01/a01-primeiro-programa
cd /home/amsterdan/developer/learn-rust/aulas/m01/a01-primeiro-programa
```

## Revisões práticas

As revisões têm identificadores próprios e não entram na contagem de aulas. As pastas em `revisoes/` são projetos novos criados pelo aluno com as mesmas regras de comandos absolutos, autoria e validação; R08 usa o servidor existente. R09–R11 acompanham apenas os aprofundamentos escolhidos; o momento indicado é sugerido e pode ser adaptado sem exigir os opcionais omitidos.

Além dos pré-requisitos indicados, uma aula pressupõe os fundamentos necessários dos módulos anteriores; a coluna por aula destaca a sequência imediata. Os aprofundamentos M18–M22 têm entradas próprias e não exigem concluir todos os opcionais anteriores.

| Revisão | Momento | Blocos | Entrega e critério |
|---|---|---|---|
| R01 | Após M02; `revisoes/r01-logica-e-testes` | 2 | Resolver uma classificação inédita com funções e laços; entregar três testes e explicar uma falha provocada. |
| R02 | Após M04; `revisoes/r02-ownership-e-emprestimos` | 2 | Implementar consultas com `&str`/`&[i32]` e justificar movimento, empréstimo e eventual cópia; explicar um erro de borrow checker. |
| R03 | Após M06; `revisoes/r03-modelagem-de-estados` | 2 | Modelar uma entidade com struct, enum e `Option`, executar comandos predefinidos e testar uma transição inválida. |
| R04 | Após M08; `revisoes/r04-entrada-e-validacao` | 2 | Resolver um novo problema de entrada interativa com validação, erros distintos e testes da lógica pura. |
| R05 | Após M10; `revisoes/r05-biblioteca-e-erros` | 2 | Criar biblioteca pequena com API pública, um trait, erro próprio e doc test; justificar a conversão usada por `?`. |
| R06 | Após M13; `revisoes/r06-busca-e-refatoracao` | 3 | Resolver uma busca inédita e comparar laço/iteradores com os mesmos testes; explicar captura, consumo e referências. |
| R07 | Após M15; `revisoes/r07-coordenacao-de-trabalho` | 3 | Coordenar jobs com canal ou `Arc<Mutex<_>>`, testar o resultado sem sleeps arbitrários e justificar o modelo de ownership. |
| R08 | Após M17; Projeto do M17 | 2 | Demonstrar e explicar o servidor do M17; alterar uma rota sem solução pronta e preservar testes/shutdown. Usar o próprio projeto, sem criar outro servidor. |
| R09 | Após M18; `revisoes/r09-tarefas-assincronas` | 3 | Coordenar tarefas simuladas locais, limitar a espera e tratar encerramento; explicar bloqueio e cancelamento. Não exigir os internals de A10–A11. |
| R10 | Após M20; `revisoes/r10-abstracoes-e-macros` | 2 | Escolher um problema pequeno de API ou macro, implementar e justificar a abstração. Cobrar apenas os aprofundamentos efetivamente cursados. |
| R11 | Após M22; `revisoes/r11-aprofundamento-escolhido` | 2 | Apresentar uma investigação de unsafe/FFI, compatibilidade ou desempenho conforme a trilha escolhida, com evidências e limites. Não exigir módulos opcionais não cursados. |

---

## Módulo 01 — Toolchain, Cargo e fundamentos da linguagem (10 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m01.md).

**Fontes:** Rust Book §§1 e 3.1–3.2; Rust by Example: Hello World, Primitives, Variable Bindings, Types; Rustlings: intro, variables, primitive_types.

**Pré-requisitos:** Nenhum.

**Critério de domínio:** Criar e executar um pacote; escolher tipos básicos e interpretar um diagnóstico sem apenas silenciá-lo.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M01-A01 `m01-a01-primeiro-programa` [roteiro](roteiros/m01.md#m01-a01) | Entrada do módulo | B / 1 | **Novo:** instalar Rust com `rustup` se necessário; distinguir `rustc` e Cargo; compilar e executar um `Hello, world!` mínimo. |
| M01-A02 `m01-a02-pacote-cargo` [roteiro](roteiros/m01.md#m01-a02) | M01-A01 | B / 1 | **Novo:** usar `cargo new`, reconhecer `Cargo.toml`, `src/main.rs` e `target/`; executar `cargo run`, `cargo check` e `cargo build` no mesmo pacote. |
| M01-A03 `m01-a03-saida-formatada` [roteiro](roteiros/m01.md#m01-a03) | M01-A02 | B / 1 | **Novo:** usar `println!`, `{}` e argumentos posicionais/nomeados com tipos padrão; distinguir saída para pessoas de `Debug` (`{:?}`); imprimir um recibo com argumentos literais. Usar `Display` existente é permitido; implementá-lo para um tipo próprio fica no M10-A09. |
| M01-A04 `m01-a04-variaveis` [roteiro](roteiros/m01.md#m01-a04) | M01-A03 | B / 1 | **Novo:** declarar com `let`, anotar tipos e usar `mut`; produzir um programa com estado imutável e uma atualização deliberadamente mutável. |
| M01-A05 `m01-a05-shadowing-e-escopo` [roteiro](roteiros/m01.md#m01-a05) | M01-A04 | B / 1 | **Novo:** distinguir shadowing, mutabilidade e escopo de bloco; transformar uma entrada em etapas usando o mesmo nome sem confundir os mecanismos. |
| M01-A06 `m01-a06-constantes` [roteiro](roteiros/m01.md#m01-a06) | M01-A05 | B / 1 | **Novo:** declarar `const`, anotar tipo e diferenciar constante de binding; calcular e exibir um valor configurado por constante. |
| M01-A07 `m01-a07-tipos-numericos` [roteiro](roteiros/m01.md#m01-a07) | M01-A06 | B / 1 | **Novo:** inteiros, floats, sinais, tamanhos, literais, inferência e overflow; escolher tipos explícitos para um conversor simples. |
| M01-A08 `m01-a08-bool-e-char` [roteiro](roteiros/m01.md#m01-a08) | M01-A07 | B / 1 | **Novo:** `bool`, `char` e Unicode; distinguir caractere, escalar Unicode e byte sem entrar ainda em UTF-8 de `String`. |
| M01-A09 `m01-a09-tuplas-e-arrays` [roteiro](roteiros/m01.md#m01-a09) | M01-A08 | B / 1 | **Novo:** tuplas, arrays fixos, acesso e desestruturação simples; deixar slices para M04. |
| M01-A10 `m01-a10-formatacao-clippy-e-diagnosticos` [roteiro](roteiros/m01.md#m01-a10) | M01-A09 | B / 1 | **Novo:** o aluno usa `cargo fmt`, `cargo check` e `cargo clippy`, identifica arquivo/linha e corrige um aviso simples. Distinguir formatação de verificação; consultar `rustc --explain` quando houver código de erro. O agente mostra os comandos e a interpretação, sem executar correções. |

## Módulo 02 — Funções, expressões, controle de fluxo e primeiros testes (9 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m02.md).

**Fontes:** Rust Book §§3.3–3.5; Rust by Example: Expressions, Flow of Control, Functions; Rustlings: functions, if. Rust Book §11.1 para testes iniciais.

**Pré-requisitos:** M01 completo.

**Critério de domínio:** Resolver uma classificação ou cálculo com funções, condições e laços; escrever testes que falhem quando o resultado é alterado.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M02-A01 `m02-a01-assinaturas-de-funcoes` [roteiro](roteiros/m02.md#m02-a01) | Entrada do módulo | B / 1 | **Novo:** declarar e chamar funções, parâmetros tipados e convenções de nomes; separar cálculos em duas funções pequenas. |
| M02-A02 `m02-a02-expressoes-e-retornos` [roteiro](roteiros/m02.md#m02-a02) | M02-A01 | B / 1 | **Novo:** distinguir expressão de statement, retorno explícito e expressão final; escrever funções que retornam valores sem `return` desnecessário. |
| M02-A03 `m02-a03-operadores` [roteiro](roteiros/m02.md#m02-a03) | M02-A02 | B / 1 | **Novo:** operadores aritméticos, comparação, lógica e precedência; calcular e validar uma expressão com tipos compatíveis. |
| M02-A04 `m02-a04-if-como-expressao` [roteiro](roteiros/m02.md#m02-a04) | M02-A03 | B / 1 | **Novo:** `if`/`else`, condições booleanas e ramos do mesmo tipo; classificar entradas em categorias. |
| M02-A05 `m02-a05-loop-e-break` [roteiro](roteiros/m02.md#m02-a05) | M02-A04 | B / 1 | **Novo:** `loop`, `break` com valor e labels; interromper uma busca quando a condição for satisfeita. |
| M02-A06 `m02-a06-while` [roteiro](roteiros/m02.md#m02-a06) | M02-A05 | B / 1 | **Novo:** laço condicionado, atualização de contador e encerramento; simular uma contagem regressiva. |
| M02-A07 `m02-a07-for-e-intervalos` [roteiro](roteiros/m02.md#m02-a07) | M02-A06 | B / 1 | **Novo:** `for`, intervalos abertos/fechados e iteração sobre array; somar valores sem índices manuais. |
| M02-A08 `m02-a08-fizzbuzz` [roteiro](roteiros/m02.md#m02-a08) | M02-A07 | B / 1 | **Integração:** combinar funções, operadores, `if` e laços em FizzBuzz; nenhuma sintaxe nova. |
| M02-A09 `m02-a09-primeiros-testes` [roteiro](roteiros/m02.md#m02-a09) | M02-A08 | B / 2 | **Novo + integração:** `#[test]`, `assert!`, `assert_eq!`, `assert_ne!` e `cargo test`; testar funções com um caso normal e dois casos de limite. Usar funções de teste no mesmo arquivo, sem exigir ainda `mod`, `use` ou biblioteca. Introduzir o atributo apenas no contexto necessário; a organização de suítes fica no M12. |

## Módulo 03 — Ownership e movimentos (6 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m03.md).

**Fontes:** Rust Book §4.1; Rust by Example: Scoping Rules; Rustlings: move_semantics.

**Pré-requisitos:** M02 completo e revisão R01.

**Critério de domínio:** Prever quais valores continuam utilizáveis após uma chamada e justificar movimento ou cópia; usar `clone` por necessidade explícita.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M03-A01 `m03-a01-escopo-e-drop` [roteiro](roteiros/m03.md#m03-a01) | Entrada do módulo | I / 2 | **Novo:** validade, escopo e descarte automático; introduzir `String::from` como forma de criar texto possuído e um modelo inicial de stack/heap. Explicar em que ponto cada valor deixa de poder ser usado; a API completa de strings fica no M07. |
| M03-A02 `m03-a02-copy-e-move` [roteiro](roteiros/m03.md#m03-a02) | M03-A01 | I / 2 | **Novo:** diferença entre cópia de tipos `Copy` e movimento de `String`; prever quais bindings continuam válidos após atribuição. |
| M03-A03 `m03-a03-clone-explicito` [roteiro](roteiros/m03.md#m03-a03) | M03-A02 | I / 1 | **Novo:** usar `clone` quando uma cópia de dados é intencional e identificar o custo de duplicação; comparar com movimento. |
| M03-A04 `m03-a04-ownership-em-funcoes` [roteiro](roteiros/m03.md#m03-a04) | M03-A03 | I / 2 | **Novo:** passar e devolver ownership por funções; escrever chamadas sem perder valores que ainda serão necessários. |
| M03-A05 `m03-a05-ownership-em-agregados` [roteiro](roteiros/m03.md#m03-a05) | M03-A04 | I / 2 | **Novo:** acompanhar ownership em tuplas e arrays que contêm `String`; localizar movimentos do agregado e de componentes desestruturados sem depender de structs ainda não ensinadas. |
| M03-A06 `m03-a06-modelo-de-transferencia` [roteiro](roteiros/m03.md#m03-a06) | M03-A05 | I / 2 | **Integração:** construir um fluxo de pedidos que transfere, devolve ou clona dados conforme a intenção; justificar cada decisão de ownership. |

## Módulo 04 — Referências, borrowing e slices (5 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m04.md).

**Fontes:** Rust Book §§4.2–4.3; Rust by Example: Scoping Rules; Rustlings: primitive_types e move_semantics.

**Pré-requisitos:** M03 completo.

**Critério de domínio:** Consultar e alterar dados por empréstimo; explicar um conflito de referências e aceitar slices concretos sem clonar a entrada.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M04-A01 `m04-a01-borrowing-imutavel` [roteiro](roteiros/m04.md#m04-a01) | Entrada do módulo | I / 2 | **Novo:** referência compartilhada e empréstimo sem movimento; escrever uma função que consulta `&String`. `&T` é apenas notação descritiva nesta etapa; não definir funções genéricas antes do M10. |
| M04-A02 `m04-a02-borrowing-mutavel` [roteiro](roteiros/m04.md#m04-a02) | M04-A01 | I / 2 | **Novo:** `&mut T` e regra de exclusividade; atualizar dado por referência sem manter empréstimos conflitantes. |
| M04-A03 `m04-a03-regras-e-referencias-pendentes` [roteiro](roteiros/m04.md#m04-a03) | M04-A02 | I / 2 | **Novo:** convivência de referências, escopo do empréstimo e rejeição de dangling references; explicar a mensagem do compilador em um caso inválido. |
| M04-A04 `m04-a04-slices-de-string-e-array` [roteiro](roteiros/m04.md#m04-a04) | M04-A03 | I / 2 | **Novo:** `&str`, slices de arrays, intervalos e limites; obter trechos sem copiar a sequência. |
| M04-A05 `m04-a05-api-de-slices` [roteiro](roteiros/m04.md#m04-a05) | M04-A04 | I / 2 | **Integração:** criar funções com `&str` e slices concretos, como `&[i32]`; aceitar tanto arrays quanto trechos desses arrays. Entregar uma consulta de texto e uma agregação numérica sem cópias desnecessárias. Não introduzir `fn<T>` ou assinaturas com `&[T]`. |

## Módulo 05 — Structs e métodos (8 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m05.md).

**Fontes:** Rust Book §5; Rust by Example: Custom Types; Rustlings: structs.

**Pré-requisitos:** M04 completo e revisão R02.

**Critério de domínio:** Modelar dados e manter invariantes por métodos; testar comportamento público sem depender de impressões no terminal.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M05-A01 `m05-a01-structs-nomeadas` [roteiro](roteiros/m05.md#m05-a01) | Entrada do módulo | I / 1 | **Novo:** definir, instanciar e acessar campos de uma struct; modelar uma entidade com dados relacionados. |
| M05-A02 `m05-a02-atualizar-structs` [roteiro](roteiros/m05.md#m05-a02) | M05-A01 | I / 2 | **Novo:** mutabilidade da instância, field init shorthand e sintaxe de atualização; prever movimentos de campos reaproveitados. |
| M05-A03 `m05-a03-tuple-e-unit-structs` [roteiro](roteiros/m05.md#m05-a03) | M05-A02 | I / 1 | **Novo:** tuple structs e unit-like structs; escolher entre formas nomeadas e posicionais para um tipo pequeno. |
| M05-A04 `m05-a04-debug-derive` [roteiro](roteiros/m05.md#m05-a04) | M05-A03 | I / 1 | **Novo:** atributo `derive(Debug)` e formatação `{:?}`/`{:#?}`; inspecionar uma struct. Não ensinar ainda macros personalizadas. |
| M05-A05 `m05-a05-metodos` [roteiro](roteiros/m05.md#m05-a05) | M05-A04 | I / 1 | **Novo:** `impl`, métodos com `&self` e chamada por ponto; calcular uma propriedade a partir dos campos. |
| M05-A06 `m05-a06-metodos-mutaveis` [roteiro](roteiros/m05.md#m05-a06) | M05-A05 | I / 2 | **Novo:** métodos com `&mut self` e receiver; implementar uma alteração que mantenha o estado válido. |
| M05-A07 `m05-a07-funcoes-associadas` [roteiro](roteiros/m05.md#m05-a07) | M05-A06 | I / 1 | **Novo:** funções associadas e `Self`; criar construtor sem reapresentar métodos de instância. |
| M05-A08 `m05-a08-retangulo` [roteiro](roteiros/m05.md#m05-a08) | M05-A07 | I / 2 | **Integração:** definir struct, construtor e métodos para área/comparação de retângulos; não incluir enums nem traits próprios. |

## Módulo 06 — Enums, Option e match básico (6 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m06.md).

**Fontes:** Rust Book §6; Rust by Example: Custom Types, Flow of Control; Rustlings: enums, options.

**Pré-requisitos:** M05 completo.

**Critério de domínio:** Representar estados válidos com enums; tratar ausência e todos os casos de uma operação sem strings mágicas.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M06-A01 `m06-a01-enums` [roteiro](roteiros/m06.md#m06-a01) | Entrada do módulo | I / 1 | **Novo:** definir enums, variantes e valores; modelar estados mutuamente exclusivos sem strings mágicas. |
| M06-A02 `m06-a02-dados-em-variantes` [roteiro](roteiros/m06.md#m06-a02) | M06-A01 | I / 1 | **Novo:** variantes com campos nomeados ou posicionais; associar dados à variante correta. |
| M06-A03 `m06-a03-option` [roteiro](roteiros/m06.md#m06-a03) | M06-A02 | I / 1 | **Novo:** `Option<T>`, `Some` e `None`; representar ausência sem valor sentinela. |
| M06-A04 `m06-a04-match-exaustivo` [roteiro](roteiros/m06.md#m06-a04) | M06-A03 | I / 2 | **Novo:** `match` com enums e `Option`, extração simples e exaustividade; tratar todos os casos de uma operação. |
| M06-A05 `m06-a05-if-let-e-let-else` [roteiro](roteiros/m06.md#m06-a05) | M06-A04 | I / 2 | **Novo:** usar `if let` para um caso e `let...else` para saída antecipada; não cobrir ainda a taxonomia completa de padrões. |
| M06-A06 `m06-a06-enum-de-comandos` [roteiro](roteiros/m06.md#m06-a06) | M06-A05 | I / 2 | **Integração:** representar comandos pré-definidos por enum e despachá-los com `match`; reforçar tipos de domínio e exaustividade, sem introduzir parsing de texto. |

## Módulo 07 — Coleções da biblioteca padrão (7 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m07.md).

**Fontes:** Rust Book §8; Rust by Example: Std library types; Rustlings: vecs, strings, hashmaps.

**Pré-requisitos:** M06 completo e revisão R03.

**Critério de domínio:** Escolher `Vec`, `String` ou `HashMap` conforme a tarefa, tratar acessos inválidos e explicar o efeito de um empréstimo da coleção.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M07-A01 `m07-a01-vec-criar-acessar` [roteiro](roteiros/m07.md#m07-a01) | Entrada do módulo | I / 1 | **Novo:** criar, adicionar e acessar `Vec<T>` por índice ou `get`; tratar ausência retornada por `get`. |
| M07-A02 `m07-a02-vec-mutacao-e-ownership` [roteiro](roteiros/m07.md#m07-a02) | M07-A01 | I / 2 | **Novo:** remover/alterar elementos e percorrer por referência compartilhada ou mutável; preservar o vetor quando ele ainda será usado. Adiar a comparação sistemática `iter`/`iter_mut`/`into_iter` para M13. |
| M07-A03 `m07-a03-string-e-utf8` [roteiro](roteiros/m07.md#m07-a03) | M07-A02 | I / 2 | **Novo:** `String` versus `str`, UTF-8, bytes, `len` e por que índice numérico não significa caractere. |
| M07-A04 `m07-a04-operacoes-com-string` [roteiro](roteiros/m07.md#m07-a04) | M07-A03 | I / 1 | **Novo:** `push`, `push_str`, `format!`, concatenação e iteração por `chars`; construir uma linha com tipos padrão já formatáveis. Implementar `Display` para tipo próprio fica para M10. |
| M07-A05 `m07-a05-hashmap` [roteiro](roteiros/m07.md#m07-a05) | M07-A04 | I / 1 | **Novo:** criar, ler, atualizar, substituir e remover valores em `HashMap<K,V>`. |
| M07-A06 `m07-a06-entry-e-contagem` [roteiro](roteiros/m07.md#m07-a06) | M07-A05 | I / 2 | **Novo:** API `entry`, `or_insert` e `or_default`; contar palavras sem uma busca seguida de inserção duplicada. |
| M07-A07 `m07-a07-placar-de-jogos` [roteiro](roteiros/m07.md#m07-a07) | M07-A06 | I / 2 | **Integração:** agregar resultados de partidas em um `HashMap` e apresentar um placar; aplicar coleções em um domínio distinto da contagem de palavras. |

## Módulo 08 — Erros, conversões primitivas e entrada interativa (8 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m08.md).

**Fontes:** Rust Book §9 e passagens de §§2 e 12; Rust by Example: Error handling e casts em Conversion; Rustlings: error_handling e options. Traits de conversão aparecem no M10, depois de traits básicos.

**Pré-requisitos:** M07 completo.

**Critério de domínio:** Receber texto do usuário, converter e validar dados, distinguir falha esperada de bug e propagar erros sem `unwrap` indiscriminado.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M08-A01 `m08-a01-panic-e-backtrace` [roteiro](roteiros/m08.md#m08-a01) | Entrada do módulo | I / 1 | **Novo:** panic, backtrace e falhas irrecuperáveis; distinguir bug/invariante quebrada de erro esperado. Não usar `panic!` para validar entrada normal. |
| M08-A02 `m08-a02-result` [roteiro](roteiros/m08.md#m08-a02) | M08-A01 | I / 1 | **Novo:** `Result<T,E>`, `Ok`, `Err` e `match`; devolver falha recuperável em vez de abortar. |
| M08-A03 `m08-a03-propagacao-com-interrogacao` [roteiro](roteiros/m08.md#m08-a03) | M08-A02 | I / 2 | **Novo:** propagar `Result` com `?` quando o tipo de erro de retorno é compatível; eliminar tratamento repetitivo sem apagar a falha. A conversão automática entre erros é praticada no M10-A15. |
| M08-A04 `m08-a04-erros-proprios` [roteiro](roteiros/m08.md#m08-a04) | M08-A03 | I / 2 | **Novo:** enum de erro próprio como tipo de retorno; separar categorias de falha sem implementar ainda traits de formatação/erro nem usar trait objects. |
| M08-A05 `m08-a05-compor-option-e-result` [roteiro](roteiros/m08.md#m08-a05) | M08-A04 | I / 2 | **Novo:** converter ausência em erro com `ok_or`, inspecionar `Option`/`Result` e combinar operações com `match` e `?`. Se usar um combinador, escolher uma função já conhecida; métodos que exigem closures ficam para M13. |
| M08-A06 `m08-a06-casts` [roteiro](roteiros/m08.md#m08-a06) | M08-A05 | I / 1 | **Novo:** casts numéricos com `as`, truncamento e verificações de intervalo; não confundir cast com conversão validada. |
| M08-A07 `m08-a07-stdin-e-parse` [roteiro](roteiros/m08.md#m08-a07) | M08-A06 | I / 2 | **Novo:** `std::io::stdin`, `read_line`, `trim` e `parse::<u32>()` como API pronta; receber uma entrada, distinguir erro de leitura de erro de formato e permitir nova tentativa. Usar caminhos completos sem antecipar a teoria de módulos ou a implementação de `FromStr`. |
| M08-A08 `m08-a08-cadastro-validado` [roteiro](roteiros/m08.md#m08-a08) | M08-A07 | I / 3 | **Integração:** criar um cadastro interativo em memória com struct, enum de erro, entrada textual e validação de idade; separar leitura e lógica pura, testar limites e devolver mensagens compreensíveis. Sem dependências externas nem persistência em arquivo. |

## Módulo 09 — Módulos, crates, Cargo e documentação (10 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m09.md).

**Fontes:** Rust Book §§7 e 14; Rust by Example: Modules, Crates, Cargo; Rustlings: modules. Rust Book §14.2 para rustdoc e doc tests.

**Pré-requisitos:** M08 completo e revisão R04.

**Critério de domínio:** Organizar biblioteca e binário, importar APIs, declarar dependências e manter um exemplo de documentação executável.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M09-A01 `m09-a01-arvore-de-modulos` [roteiro](roteiros/m09.md#m09-a01) | Entrada do módulo | I / 1 | **Novo:** `mod`, árvore, escopo e caminhos entre módulos dentro de um único arquivo. |
| M09-A02 `m09-a02-caminhos-e-visibilidade` [roteiro](roteiros/m09.md#m09-a02) | M09-A01 | I / 2 | **Novo:** caminhos `crate`, `self`, `super`, público/privado e privacidade de campos; explicar por que um item não é acessível. |
| M09-A03 `m09-a03-use-e-reexport` [roteiro](roteiros/m09.md#m09-a03) | M09-A02 | I / 1 | **Novo:** `use`, aliases, `pub use` e imports glob; organizar API pública evitando imports glob indiscriminados. |
| M09-A04 `m09-a04-modulos-em-arquivos` [roteiro](roteiros/m09.md#m09-a04) | M09-A03 | I / 2 | **Novo:** mover módulos para arquivos e subdiretórios mantendo caminhos e visibilidade. |
| M09-A05 `m09-a05-pacote-binario-e-biblioteca` [roteiro](roteiros/m09.md#m09-a05) | M09-A04 | I / 2 | **Novo:** pacote, crate, múltiplos binários e `src/lib.rs`; criar biblioteca e binário de demonstração no mesmo pacote. |
| M09-A06 `m09-a06-rustdoc-e-doc-tests` [roteiro](roteiros/m09.md#m09-a06) | M09-A05 | I / 2 | **Novo:** comentários `///`, seções de exemplos/erros e `cargo doc`; documentar uma função pública de `src/lib.rs` e validar seu exemplo com `cargo test --doc`. Apresentar também o Playground como consulta. Não exigir macros personalizadas para usar a documentação. |
| M09-A07 `m09-a07-dependencias-e-lockfile` [roteiro](roteiros/m09.md#m09-a07) | M09-A06 | I / 1 | **Novo:** adicionar crate externo no `Cargo.toml`, papel do `Cargo.lock`, faixas de versão e consulta à documentação. |
| M09-A08 `m09-a08-workspaces` [roteiro](roteiros/m09.md#m09-a08) | M09-A07 | I / 2 | **Novo:** workspace Cargo com membros e dependência local; identificar quando workspace ajuda e quando um projeto independente basta. |
| M09-A09 `m09-a09-perfis-install-e-publicacao` [roteiro](roteiros/m09.md#m09-a09) | M09-A08 | I / 2 | **Novo:** perfis debug/release, `cargo install`, comandos Cargo estendidos e metadados para publicar; preparar o pacote sem publicar nada externamente. |
| M09-A10 `m09-a10-jogo-de-adivinhacao` [roteiro](roteiros/m09.md#m09-a10) | M09-A09 | I / 3 | **Novo + integração:** criar o jogo de adivinhação do Book usando entrada padrão e tratamento de erros do M08. Introduzir `rand` e `Ordering` como APIs prontas, com dependência e documentação identificadas; combinar `parse`, `match` e loop. Usar `{}` para números/textos e `Debug` quando adequado; não exigir a implementação de `Display`. |

## Módulo 10 — Generics, traits, conversões e erros próprios (15 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m10.md).

**Fontes:** Rust Book §§10.1–10.2; Rust by Example: Generics, Traits, Conversion; Rustlings: generics, traits, conversions. Rust Book §20.2 para tipos associados; documentação de `std::error::Error`.

**Pré-requisitos:** M09 completo; erros do M08.

**Critério de domínio:** Criar uma abstração genérica pequena e um erro próprio com `Debug`, `Display`, `Error` e conversão usada por `?`.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M10-A01 `m10-a01-funcoes-genericas` [roteiro](roteiros/m10.md#m10-a01) | Entrada do módulo | I / 2 | **Novo:** parâmetros de tipo em funções; substituir duas funções duplicadas por uma implementação genérica. |
| M10-A02 `m10-a02-structs-e-enums-genericos` [roteiro](roteiros/m10.md#m10-a02) | M10-A01 | I / 1 | **Novo:** tipos e enums genéricos com um ou mais parâmetros; usar valores de tipos diferentes sem duplicar definições. |
| M10-A03 `m10-a03-impl-generico` [roteiro](roteiros/m10.md#m10-a03) | M10-A02 | I / 1 | **Novo:** `impl<T>`, métodos disponíveis para todos os tipos e métodos restritos a um tipo específico. |
| M10-A04 `m10-a04-definir-e-implementar-trait` [roteiro](roteiros/m10.md#m10-a04) | M10-A03 | I / 2 | **Novo:** definir trait, implementar em tipo próprio e aplicar regra de coerência/orphan rule em casos básicos. |
| M10-A05 `m10-a05-metodos-padrao` [roteiro](roteiros/m10.md#m10-a05) | M10-A04 | I / 1 | **Novo:** método padrão de trait e sobrescrita; compartilhar comportamento sem herança de classes. |
| M10-A06 `m10-a06-bounds-e-where` [roteiro](roteiros/m10.md#m10-a06) | M10-A05 | I / 2 | **Novo:** trait bounds, mais de um bound e cláusula `where`; restringir uma função genérica ao comportamento exigido. |
| M10-A07 `m10-a07-impl-trait-entrada` [roteiro](roteiros/m10.md#m10-a07) | M10-A06 | I / 1 | **Novo:** `impl Trait` em parâmetro e sua relação com genéricos; escolher a forma conforme a assinatura. |
| M10-A08 `m10-a08-impl-trait-retorno` [roteiro](roteiros/m10.md#m10-a08) | M10-A07 | I / 2 | **Novo:** `impl Trait` no retorno para ocultar um tipo concreto; demonstrar que todos os caminhos de retorno devem produzir o mesmo tipo concreto oculto, ainda que tipos diferentes implementem o mesmo trait. Contrastar duas ramificações válidas e um exemplo rejeitado pelo compilador. |
| M10-A09 `m10-a09-display-e-tostring` [roteiro](roteiros/m10.md#m10-a09) | M10-A08 | I / 2 | **Novo:** implementar `Display` e usar `ToString`; formatar tipo próprio para pessoas e relacionar o método gerado a partir de `Display`. |
| M10-A10 `m10-a10-from-e-into` [roteiro](roteiros/m10.md#m10-a10) | M10-A09 | I / 1 | **Novo:** implementar `From` e usar `Into` para conversão infalível; conectar os traits padrão às regras de coerência. |
| M10-A11 `m10-a11-tryfrom-e-tryinto` [roteiro](roteiros/m10.md#m10-a11) | M10-A10 | I / 2 | **Novo:** implementar conversão validada com `TryFrom`/`TryInto` e preservar a informação de falha. |
| M10-A12 `m10-a12-fromstr-e-parse` [roteiro](roteiros/m10.md#m10-a12) | M10-A11 | I / 2 | **Novo:** implementar `FromStr` e consumir `parse`; relacionar a saída a `Result` aprendido no M08. |
| M10-A13 `m10-a13-asref-asmut` [roteiro](roteiros/m10.md#m10-a13) | M10-A12 | I / 2 | **Novo:** usar `AsRef`/`AsMut` para expor vistas emprestadas de tipos diferentes; distinguir essa conversão de referência de `From`/`Into`. |
| M10-A14 `m10-a14-tipos-associados` [roteiro](roteiros/m10.md#m10-a14) | M10-A13 | I / 2 | **Novo:** tipos associados e diferença para parâmetros genéricos de trait; implementar uma abstração pequena. |
| M10-A15 `m10-a15-error-display-e-propagacao` [roteiro](roteiros/m10.md#m10-a15) | M10-A14 | I / 3 | **Integração + expansão:** retomar o enum de erro do M08, derivar `Debug`, implementar `Display` e `std::error::Error`, e implementar `From` para um erro de parsing. Entregar uma função que usa `?` para converter para o erro do domínio, com testes e documentação. O método `source` e `Box<dyn Error>` ficam no M16-A03. |

## Módulo 11 — Lifetimes na prática (5 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m11.md).

**Fontes:** Rust Book §10.3; Rust by Example: Scoping Rules; Rustlings: lifetimes.

**Pré-requisitos:** M10 completo e revisão R05; borrowing do M04.

**Critério de domínio:** Relacionar a validade de entradas e saídas emprestadas; escolher entre retornar referência e valor possuído sem tentar prolongar dados com uma anotação.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M11-A01 `m11-a01-validade-e-lifetime` [roteiro](roteiros/m11.md#m11-a01) | Entrada do módulo | I / 2 | **Novo:** lifetime como relação entre referências; mostrar que anotação não prolonga a vida do dado. |
| M11-A02 `m11-a02-lifetime-em-funcoes` [roteiro](roteiros/m11.md#m11-a02) | M11-A01 | I / 2 | **Novo:** anotar relações entre parâmetros e retorno; corrigir assinatura que poderia devolver referência inválida. |
| M11-A03 `m11-a03-struct-com-referencia` [roteiro](roteiros/m11.md#m11-a03) | M11-A02 | I / 2 | **Novo:** armazenar referência em struct com lifetime explícito e garantir que o dado viva o bastante. |
| M11-A04 `m11-a04-elision-e-static` [roteiro](roteiros/m11.md#m11-a04) | M11-A03 | I / 2 | **Novo:** regras de elision e significado/contextos de `'static`; não tratar `'static` como solução universal. |
| M11-A05 `m11-a05-api-emprestada-e-ownership` [roteiro](roteiros/m11.md#m11-a05) | M11-A04 | I / 2 | **Integração:** escrever uma consulta que devolve um trecho da entrada e uma alternativa que devolve dado possuído. Testar casos de ausência e comparar as assinaturas, justificando lifetime e custo de cópia. HRTBs, subtipagem e detalhes de DST ficam no M19. |

## Módulo 12 — Testes aprofundados e programa de linha de comando (9 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m12.md).

**Fontes:** Rust Book §§11–12; Rust by Example: Testing; Rustlings: tests.

**Pré-requisitos:** M11 completo; testes básicos do M02-A09, biblioteca/documentação do M09 e erros próprios do M10.

**Critério de domínio:** Entregar um CLI que trata argumentos e I/O, separa lógica testável e documenta mensagens de uso e falhas.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M12-A01 `m12-a01-casos-de-limite-e-resultados` [roteiro](roteiros/m12.md#m12-a01) | Entrada do módulo | I / 2 | **Expansão:** planejar testes de sucesso, ausência, limites e erros de domínio; verificar `Option`/`Result` e mensagens relevantes. Reutilizar as assertions do M02 sem reensinar a sintaxe; incluir um teste que detecta uma alteração incorreta da lógica. |
| M12-A02 `m12-a02-testes-de-falha` [roteiro](roteiros/m12.md#m12-a02) | M12-A01 | I / 2 | **Novo:** `should_panic`, mensagens e testes para entrada inválida; não usar panic como tratamento de erro em produção. |
| M12-A03 `m12-a03-harness-e-filtros` [roteiro](roteiros/m12.md#m12-a03) | M12-A02 | I / 1 | **Expansão:** aprofundar o harness de `cargo test` com filtros, saída, execução paralela e `ignore`; selecionar e interpretar testes e evitar dependência acidental de estado global entre eles. |
| M12-A04 `m12-a04-testes-unitarios` [roteiro](roteiros/m12.md#m12-a04) | M12-A03 | I / 2 | **Expansão:** módulo `tests`, `#[cfg(test)]`, `use super` e organização junto à implementação; testar uma função privada e separar código de teste. O efeito completo de atributos de configuração fica no M20. |
| M12-A05 `m12-a05-testes-de-integracao` [roteiro](roteiros/m12.md#m12-a05) | M12-A04 | I / 2 | **Novo:** diretório `tests/`, testes externos ao crate e `[dev-dependencies]`; testar API pública. |
| M12-A06 `m12-a06-argumentos-e-ambiente` [roteiro](roteiros/m12.md#m12-a06) | M12-A05 | I / 2 | **Novo:** `std::env::args`, variáveis de ambiente e validação de argumentos; produzir mensagens de uso. |
| M12-A07 `m12-a07-arquivos-e-stderr` [roteiro](roteiros/m12.md#m12-a07) | M12-A06 | I / 2 | **Novo:** abrir, ler e escrever arquivos; lidar com caminhos, UTF-8, erros de I/O, `eprintln!` e código de saída. |
| M12-A08 `m12-a08-paths-e-processos` [roteiro](roteiros/m12.md#m12-a08) | M12-A07 | I / 2 | **Novo:** `Path`/`PathBuf`, operações de filesystem e iniciar um processo filho com `std::process::Command`; tratar saída e falha do processo. |
| M12-A09 `m12-a09-minigrep` [roteiro](roteiros/m12.md#m12-a09) | M12-A08 | I / 3 | **Integração:** entregar o `minigrep` com biblioteca/binário, busca textual por laço explícito, configuração por ambiente e tratamento de erros. Testar busca sensível e insensível a maiúsculas, arquivo inválido e argumentos ausentes. Registrar o comportamento para compará-lo à refatoração do M13-A10. |

## Módulo 13 — Closures, callbacks e iteradores (10 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m13.md).

**Fontes:** Rust Book §13; Rust by Example: Functions; Rustlings: iterators. Rust Book §20.4 para ponteiros de função.

**Pré-requisitos:** M12 completo; tipos associados do M10-A14.

**Critério de domínio:** Escolher o modo de captura, explicar consumo e empréstimo na iteração e refatorar uma busca preservando testes e comportamento.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M13-A01 `m13-a01-sintaxe-de-closures` [roteiro](roteiros/m13.md#m13-a01) | Entrada do módulo | I / 1 | **Novo:** closures, parâmetros, inferência e anotação opcional; substituir uma função local simples. |
| M13-A02 `m13-a02-captura-de-ambiente` [roteiro](roteiros/m13.md#m13-a02) | M13-A01 | I / 2 | **Novo:** captura imutável, mutável e por movimento; identificar qual acesso a closure exige. |
| M13-A03 `m13-a03-fn-fnmut-fnonce` [roteiro](roteiros/m13.md#m13-a03) | M13-A02 | I / 2 | **Novo:** contratos `Fn`, `FnMut` e `FnOnce`; associar cada trait às capturas e chamadas possíveis. |
| M13-A04 `m13-a04-callbacks-e-ponteiros-de-funcao` [roteiro](roteiros/m13.md#m13-a04) | M13-A03 | I / 2 | **Novo:** usar `fn(i32) -> i32` como ponteiro de função e receber uma função nomeada como callback; comparar com um parâmetro genérico limitado por `Fn`. Entregar duas estratégias para o mesmo cálculo; HRTBs e closures retornadas ficam no M19. |
| M13-A05 `m13-a05-iterator-next` [roteiro](roteiros/m13.md#m13-a05) | M13-A04 | I / 1 | **Novo:** consumir um iterador existente com `next`, observar `Some`/`None`, exaustão e laziness; reconhecer `Iterator::Item` com os tipos associados do M10. Não implementar um iterador próprio nesta aula. |
| M13-A06 `m13-a06-iter-iter-mut-into-iter` [roteiro](roteiros/m13.md#m13-a06) | M13-A05 | I / 2 | **Novo:** diferença entre iterar por referência, mutável e por valor; relacionar a ownership sem repetir M07. |
| M13-A07 `m13-a07-map-filter` [roteiro](roteiros/m13.md#m13-a07) | M13-A06 | I / 2 | **Novo:** adaptadores lazy `map`/`filter`; transformar e selecionar dados. |
| M13-A08 `m13-a08-collect-e-fold` [roteiro](roteiros/m13.md#m13-a08) | M13-A07 | I / 2 | **Novo:** `collect`, tipo de destino, `zip`, `enumerate`, `fold` e composição curta; não incluir implementação de iterator customizado. |
| M13-A09 `m13-a09-iterator-proprio` [roteiro](roteiros/m13.md#m13-a09) | M13-A08 | I / 2 | **Novo:** implementar `Iterator` para uma sequência própria e definir item/fim; não incluir benchmark ou alegações de desempenho nesta aula. |
| M13-A10 `m13-a10-minigrep-com-iteradores` [roteiro](roteiros/m13.md#m13-a10) | M13-A09 | I / 3 | **Integração + expansão:** em um pacote novo, reconstruir a lógica de busca do M12-A09 a partir da especificação e de testes de regressão; escrever uma versão com laços e refatorá-la com iteradores. Comparar clareza, lifetimes e uso de referências; preservar resultados e não afirmar ganho de desempenho sem medir. |

## Módulo 14 — Smart pointers e mutabilidade interior (8 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m14.md).

**Fontes:** Rust Book §15; Rust by Example: Std library types; Rustlings: smart_pointers.

**Pré-requisitos:** M13 completo e revisão R06; ownership, lifetimes e traits dos módulos anteriores.

**Critério de domínio:** Justificar valor direto, `Box`, `Rc`, `Cell` ou `RefCell`; identificar um ciclo de referências e usar `Weak` quando necessário.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M14-A01 `m14-a01-box-e-tipo-recursivo` [roteiro](roteiros/m14.md#m14-a01) | Entrada do módulo | I / 2 | **Novo:** `Box<T>`, alocação no heap e tipo recursivo com tamanho conhecido. |
| M14-A02 `m14-a02-deref-e-coercao` [roteiro](roteiros/m14.md#m14-a02) | M14-A01 | I / 2 | **Novo:** `Deref`, operador `*` e deref coercion; implementar wrapper que se comporta como referência. |
| M14-A03 `m14-a03-drop` [roteiro](roteiros/m14.md#m14-a03) | M14-A02 | I / 1 | **Novo:** trait `Drop`, limpeza customizada e `std::mem::drop`; observar momento de descarte. |
| M14-A04 `m14-a04-rc` [roteiro](roteiros/m14.md#m14-a04) | M14-A03 | I / 2 | **Novo:** ownership compartilhado single-thread com `Rc<T>` e contagem forte. |
| M14-A05 `m14-a05-cell-e-refcell` [roteiro](roteiros/m14.md#m14-a05) | M14-A04 | I / 2 | **Novo:** mutabilidade interior com `Cell<T>` e `RefCell<T>`; contrastar checagem estática e runtime. |
| M14-A06 `m14-a06-rc-refcell` [roteiro](roteiros/m14.md#m14-a06) | M14-A05 | I / 2 | **Novo:** combinar compartilhamento e mutação interior em grafo local; restringir exemplo a uma thread. |
| M14-A07 `m14-a07-weak-e-ciclos` [roteiro](roteiros/m14.md#m14-a07) | M14-A06 | I / 2 | **Novo:** `Weak<T>`, referência fraca e prevenção de ciclos que impedem descarte. |
| M14-A08 `m14-a08-cow-e-escolha-de-pointer` [roteiro](roteiros/m14.md#m14-a08) | M14-A07 | I / 2 | **Novo:** `Cow` e critérios de escolha entre valor, `Box`, `Rc` e interior mutability; sem introduzir `Arc<Mutex<T>>`. |

## Módulo 15 — Threads e concorrência (8 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m15.md).

**Fontes:** Rust Book §16; Rust by Example: Std misc; Rustlings: threads.

**Pré-requisitos:** M14 completo; closures do M13.

**Critério de domínio:** Coordenar trabalho com mensagens ou estado compartilhado, aguardar conclusão e explicar limites de `Send`/`Sync` e riscos de locking.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M15-A01 `m15-a01-spawn-e-join` [roteiro](roteiros/m15.md#m15-a01) | Entrada do módulo | I / 2 | **Novo:** `thread::spawn`, `JoinHandle` e `join`; aguardar resultado de trabalho concorrente. |
| M15-A02 `m15-a02-move-em-threads` [roteiro](roteiros/m15.md#m15-a02) | M15-A01 | I / 2 | **Novo:** capturar dados com closure `move`; resolver ownership ao enviar dados para thread. |
| M15-A03 `m15-a03-panics-em-threads` [roteiro](roteiros/m15.md#m15-a03) | M15-A02 | I / 2 | **Novo:** tratar resultado de `join` e distinguir panic da thread de falha do processo. |
| M15-A04 `m15-a04-canais-mpsc` [roteiro](roteiros/m15.md#m15-a04) | M15-A03 | I / 2 | **Novo:** enviar e receber mensagens por canal; tratar encerramento do receiver. |
| M15-A05 `m15-a05-varios-produtores` [roteiro](roteiros/m15.md#m15-a05) | M15-A04 | I / 2 | **Novo:** clonar transmissores e coordenar vários produtores com um consumidor. |
| M15-A06 `m15-a06-mutex` [roteiro](roteiros/m15.md#m15-a06) | M15-A05 | I / 2 | **Novo:** `Mutex<T>`, guard e exclusão mútua em um caso de estado compartilhado. |
| M15-A07 `m15-a07-arc-mutex` [roteiro](roteiros/m15.md#m15-a07) | M15-A06 | I / 2 | **Novo:** combinar `Arc<Mutex<T>>` entre threads; explicar contagem atômica e locking sem repetir `Rc`. |
| M15-A08 `m15-a08-send-sync-e-modelos` [roteiro](roteiros/m15.md#m15-a08) | M15-A07 | I / 2 | **Novo + integração:** auto traits `Send`/`Sync`; comparar message passing e estado compartilhado, e reconhecer deadlock, ordem de locks e mutex poisoning. Entregar uma justificativa do modelo escolhido e um teste do resultado; não construir ainda um pool nem depender de sleeps para provar correção. |

## Módulo 16 — Trait objects, modelagem de estados e padrões (8 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m16.md).

**Fontes:** Rust Book §§18–19; Rust by Example: Custom Types, Flow of Control; Rustlings: enums.

**Pré-requisitos:** M15 completo e revisão R07; `Box` do M14 e erros do M10.

**Critério de domínio:** Escolher enum, genérico ou trait object conforme a necessidade; preservar causas de erro e extrair dados por padrões sem movimentos acidentais.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M16-A01 `m16-a01-encapsulamento-rust` [roteiro](roteiros/m16.md#m16-a01) | Entrada do módulo | I / 1 | **Expansão:** encapsulamento com structs, métodos e privacidade; comparar composição e abstração em Rust com ideias OO. Entregar uma API que preserva uma invariante, reutilizando sintaxe dos módulos anteriores. |
| M16-A02 `m16-a02-trait-objects-e-dispatch` [roteiro](roteiros/m16.md#m16-a02) | M16-A01 | I / 2 | **Novo:** `dyn Trait`, dispatch dinâmico e coleção heterogênea com `Box`; reconhecer requisitos básicos de dyn compatibility e comparar com genéricos. Usar um trait pequeno; tipos sem tamanho conhecido serão aprofundados no M19. |
| M16-A03 `m16-a03-erros-com-contexto-e-source` [roteiro](roteiros/m16.md#m16-a03) | M16-A02 | I / 2 | **Expansão:** usar `Box<dyn std::error::Error>` e implementar `Error::source` em um erro que encapsula uma falha anterior. Entregar uma sequência de causas inspecionável e comparar erro concreto de domínio com erro apagado; reutilizar `Debug`, `Display`, `From` e `?` do M10. |
| M16-A04 `m16-a04-padrao-state` [roteiro](roteiros/m16.md#m16-a04) | M16-A03 | I / 3 | **Novo:** implementar o padrão State e comparar transições por objetos com enum; foco nos trade-offs, não em reensinar traits. |
| M16-A05 `m16-a05-refutabilidade` [roteiro](roteiros/m16.md#m16-a05) | M16-A04 | I / 1 | **Novo:** classificar padrões irrefutáveis e refutáveis; justificar por que cada classe é aceita em certos lugares. |
| M16-A06 `m16-a06-contextos-de-padroes` [roteiro](roteiros/m16.md#m16-a06) | M16-A05 | I / 2 | **Expansão:** aplicar a classificação de padrões em `let`, parâmetros, `for`, `if let` e `match`; introduzir `while let` para consumir casos sucessivos. Reutilizar as demais formas já estudadas e justificar onde cada padrão é aceito. |
| M16-A07 `m16-a07-desestruturacao-completa` [roteiro](roteiros/m16.md#m16-a07) | M16-A06 | I / 2 | **Novo:** padrões de tuplas, structs, enums, referências, arrays/slices, `_` e `..`; extrair partes sem mover dados acidentalmente. |
| M16-A08 `m16-a08-guards-alternativas-e-at` [roteiro](roteiros/m16.md#m16-a08) | M16-A07 | I / 2 | **Novo:** padrões alternativos, guards, intervalos e `@`; não repetir match básico ou `if let`. |

## Módulo 17 — Projeto de integração: servidor web multithread (7 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m17.md).

**Fontes:** Rust Book §21, com conceitos de ownership, erros, traits, canais, threads e testes; Rust by Example: Std misc e Testing.

**Pré-requisitos:** M16 completo; CLI/testes do M12, closures do M13, `Drop` do M14 e canais/threads do M15.

**Critério de domínio:** Demonstrar o servidor local, explicar os jobs do pool e comprovar encerramento ordenado e comportamento das rotas com testes.

**Projeto contínuo:** todas as aulas usam `/home/amsterdan/developer/learn-rust/aulas/m17/a01-servidor-web` neste checkout. Só a A01 cria o pacote; A02–A07 retomam esse caminho. Cada etapa deixa o projeto compilável e registra o comportamento anterior; o aluno pode fazer commits locais para comparar etapas.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M17-A01 `m17-a01-servidor-web` [roteiro](roteiros/m17.md#m17-a01) | Entrada do módulo | I / 2 | **Novo:** criar o único pacote do módulo, `aulas/m17/a01-servidor-web`, com biblioteca/binário; usar `TcpListener`, aceitar uma conexão local e ler bytes. Entregar o primeiro servidor e registrar seu estado inicial. |
| M17-A02 continuação de `m17-a01-servidor-web` [roteiro](roteiros/m17.md#m17-a02) | M17-A01 | I / 2 | **Novo:** no mesmo pacote, ler requisição limitada e montar status, headers e corpo. Entregar uma rota de sucesso e uma rota ausente com testes da lógica pura. |
| M17-A03 continuação de `m17-a01-servidor-web` [roteiro](roteiros/m17.md#m17-a03) | M17-A02 | I / 2 | **Integração:** manter listener e parsing, aceitar conexões sequencialmente e observar o bloqueio. Entregar um loop com limite de conexões para encerramento verificável. |
| M17-A04 continuação de `m17-a01-servidor-web` [roteiro](roteiros/m17.md#m17-a04) | M17-A03 | I / 2 | **Integração:** evoluir o atendimento para uma thread por conexão, discutir custo/limite e preservar os testes existentes. Registrar a diferença para a versão sequencial. |
| M17-A05 continuação de `m17-a01-servidor-web` [roteiro](roteiros/m17.md#m17-a05) | M17-A04 | I / 4 | **Novo:** substituir threads ilimitadas por pool inspirado no Book, com fila por canal, workers e jobs `Box<dyn FnOnce() + Send + 'static>`. Validar quantidade de workers e execução de jobs; não recriar listener ou parsing. |
| M17-A06 continuação de `m17-a01-servidor-web` [roteiro](roteiros/m17.md#m17-a06) | M17-A05 | I / 3 | **Novo + integração:** encerrar o canal, usar `Drop` e aguardar workers; tratar falhas relevantes. Testar execução de jobs e shutdown sem espera arbitrária; manter limites de leitura/conexões do servidor. |
| M17-A07 continuação de `m17-a01-servidor-web` [roteiro](roteiros/m17.md#m17-a07) | M17-A06 | I / 3 | **Integração:** revisar o mesmo projeto, documentar execução e limitações e demonstrar rotas, concorrência e encerramento. Entregar testes e exemplos executáveis. Integrar os conceitos necessários ao servidor; não exigir async, FFI, unsafe ou macros próprias. |

## Módulo 18 — Programação assíncrona (11 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m18.md).

**Fontes:** Rust Book §17; exemplos usam `trpl` conforme o Book e identificam o runtime/dependências subjacentes.

**Pré-requisitos:** M13–M16 completos; M17 é recomendado como experiência de integração, sem ser pré-requisito de async.

**Critério de domínio:** Executar e coordenar tarefas assíncronas, tratar timeout/encerramento e reconhecer quando uma operação bloqueia o runtime.

**Aprofundamento interno:** A10–A11 podem ser adiadas no primeiro percurso. Para avançar após A09, basta cumprir a entrega prática do módulo e a revisão R09; ler `poll`/`Waker` não é condição para começar a usar async. A02 prepara a sintaxe; a primeira execução completa ocorre na A03.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M18-A01 `m18-a01-concorrencia-e-io` [roteiro](roteiros/m18.md#m18-a01) | Entrada do módulo | I / 1 | **Novo:** diferenciar concorrência/paralelismo e trabalho CPU-bound/I/O-bound; escrever uma simulação pequena e escolher um caso em que async ajuda. |
| M18-A02 `m18-a02-future-async-await` [roteiro](roteiros/m18.md#m18-a02) | M18-A01 | I / 2 | **Novo:** future como valor que representa uma computação assíncrona, `async fn`/bloco e `.await`; compor uma operação simples. Explicar que construir a future não basta para executar seu corpo; o executor da próxima aula fará o progresso. |
| M18-A03 `m18-a03-runtime-e-primeira-future` [roteiro](roteiros/m18.md#m18-a03) | M18-A02 | I / 2 | **Novo:** executar async com `trpl::block_on`, adicionar a dependência da aula e consultar runtime; não apresentar Tokio como API de `std`. |
| M18-A04 `m18-a04-funcao-async-de-rede` [roteiro](roteiros/m18.md#m18-a04) | M18-A03 | I / 2 | **Novo:** decompor obtenção de HTML e extração de título em operações async, distinguindo falha de transporte de título ausente. Usar simulação local falível para os casos determinísticos; na demonstração externa com trpl::get 0.3.0, explicar que o helper provoca panic na falha, em vez de retornar Result. Identificar dependência e acesso externo. |
| M18-A05 `m18-a05-spawn-task-e-handle` [roteiro](roteiros/m18.md#m18-a05) | M18-A04 | I / 2 | **Novo:** `spawn_task`, `sleep`, handle e espera pela conclusão; comparar conceitualmente com thread sem misturar APIs. |
| M18-A06 `m18-a06-join-select` [roteiro](roteiros/m18.md#m18-a06) | M18-A05 | I / 3 | **Novo:** usar as operações `join!` e `select` da versão documentada de `trpl`; combinar resultados, compor um timeout e explicar o que acontece com a future perdedora. Tratar cancelamento por descarte sem supor reversão de efeitos já ocorridos. |
| M18-A07 `m18-a07-yield-e-trabalho-bloqueante` [roteiro](roteiros/m18.md#m18-a07) | M18-A06 | I / 2 | **Novo:** mostrar starvation, ceder controle ao runtime e identificar trabalho síncrono que bloqueia tarefas async. |
| M18-A08 `m18-a08-canais-async` [roteiro](roteiros/m18.md#m18-a08) | M18-A07 | I / 2 | **Novo:** enviar/receber mensagens por canal assíncrono; tratar espera e encerramento do receiver. |
| M18-A09 `m18-a09-streams` [roteiro](roteiros/m18.md#m18-a09) | M18-A08 | I / 2 | **Novo:** streams, `Stream`/`StreamExt` e diferença para `Iterator`; consumir uma sequência temporal e tratar seu encerramento. Se necessário, apresentar `std::pin::pin!` apenas como operação segura de uso; a justificativa estrutural fica na A11. |
| M18-A10 `m18-a10-future-poll` [roteiro](roteiros/m18.md#m18-a10) | M18-A09 | A / 3 | **Novo:** entender `Future::Output`, `poll`, `Poll`, `Context` e `Waker` como visão estrutural; reconhecer por que normalmente se usa `.await` em vez de chamar `poll`. |
| M18-A11 `m18-a11-pin-unpin-stream` [roteiro](roteiros/m18.md#m18-a11) | M18-A10 | A / 3 | **Novo:** relacionar `Pin`/`Unpin` às garantias de movimentação de valores e examinar `Stream::poll_next`; conectar a interface de baixo nível às streams da A09. Usar APIs seguras para pinning, sem implementação manual com `unsafe`. |

## Módulo 19 — Tipos, traits e funções avançadas (6 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m19.md).

**Fontes:** Rust Book §§20.2–20.4; Rust by Example: Generics, Traits, Scoping Rules; Rust Reference: trait and lifetime bounds.

**Pré-requisitos:** M10–M11, M13, M14 e M16 completos; não requer M18.

**Critério de domínio:** Justificar uma assinatura avançada a partir de um problema concreto e reconhecer quando uma API mais simples atende à mesma necessidade.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M19-A01 `m19-a01-lifetime-bounds-e-higher-ranked` [roteiro](roteiros/m19.md#m19-a01) | Entrada do módulo | A / 3 | **Novo:** bounds de lifetime, subtipagem e higher-ranked trait bounds (`for<'a>`); comparar um callback válido para qualquer lifetime de entrada com um que fixa uma relação. Usar os ponteiros de função do M13-A04 e closures já conhecidos. |
| M19-A02 `m19-a02-parametros-padrao-e-operadores` [roteiro](roteiros/m19.md#m19-a02) | M19-A01 | A / 2 | **Novo:** parâmetros de tipo padrão e sobrecarga de operador; demonstrar quando a sintaxe padrão ajuda. |
| M19-A03 `m19-a03-coerencia-e-traits-avancados` [roteiro](roteiros/m19.md#m19-a03) | M19-A02 | A / 2 | **Expansão:** sintaxe totalmente qualificada, supertraits e newtype como solução para casos da regra do orphan; aplicar a regra básica de M10 sem reapresentá-la. |
| M19-A04 `m19-a04-alias-e-never` [roteiro](roteiros/m19.md#m19-a04) | M19-A03 | A / 2 | **Novo:** type aliases e never type `!`; nomear tipos compostos e reconhecer funções que não retornam. |
| M19-A05 `m19-a05-dst` [roteiro](roteiros/m19.md#m19-a05) | M19-A04 | A / 3 | **Expansão:** `Sized`, `?Sized`, tipos dimensionados dinamicamente e referências com metadados; conectar `str`, slices e `dyn Trait` já usados. Explicar a necessidade de indireção e evitar pressupor um layout de memória estável para trait objects. |
| M19-A06 `m19-a06-closures-retornadas` [roteiro](roteiros/m19.md#m19-a06) | M19-A05 | A / 3 | **Expansão:** retornar closures com `impl Fn` e com `Box<dyn Fn>` quando necessário; comparar tipos concretos distintos, captura e lifetime do ambiente. Reutilizar ponteiros de função do M13 e dispatch do M16, sem reapresentá-los. |

## Módulo 20 — Atributos e macros (9 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m20.md).

**Fontes:** Rust Book §20.5; Rust by Example: Macros, Attributes; Rustlings: macros. A documentação já foi ensinada no M09-A06.

**Pré-requisitos:** M09, M10 e M16 completos; workspaces do M09-A08. Não exige concluir M18–M19.

**Critério de domínio:** Criar uma macro pequena com interface clara e testar expansão/consumo; explicar a separação entre crate procedural e consumidor.

**Organização de crates:** A07–A09 usam cada uma um diretório de aula independente contendo um workspace local com crate procedural e consumidor. O aluno cria e edita os dois membros; o tutor lista os caminhos absolutos e os manifests necessários. Não presumir arquivos de outra aula.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M20-A01 `m20-a01-atributos` [roteiro](roteiros/m20.md#m20-a01) | Entrada do módulo | A / 2 | **Expansão:** aprofundar os atributos já usados em testes e derives: formas externas/internas, escopo e metadados; reconhecer atributos de crate e distingui-los de metadados Cargo. Mostrar `#![feature(...)]` apenas em snippet de referência nightly, separado do projeto stable. |
| M20-A02 `m20-a02-cfg` [roteiro](roteiros/m20.md#m20-a02) | M20-A01 | A / 2 | **Novo:** `cfg`/`cfg_attr`, compilação condicional e seleção de plataforma; demonstrar dois caminhos sem exigir duas máquinas. |
| M20-A03 `m20-a03-lints` [roteiro](roteiros/m20.md#m20-a03) | M20-A02 | A / 2 | **Novo:** atributos de lint `allow`/`warn`/`deny`/`forbid` e escopo; preferir corrigir aviso a silenciá-lo. |
| M20-A04 `m20-a04-macro-rules-basico` [roteiro](roteiros/m20.md#m20-a04) | M20-A03 | A / 2 | **Novo:** `macro_rules!`, matcher/transcriber e fragmentos; criar macro pequena sem repetição. |
| M20-A05 `m20-a05-macros-repeticao-e-dsl` [roteiro](roteiros/m20.md#m20-a05) | M20-A04 | A / 2 | **Novo:** repetição, separadores, interface variádica e DSL pequena; não reapresentar a sintaxe-base. |
| M20-A06 `m20-a06-higiene-de-macros` [roteiro](roteiros/m20.md#m20-a06) | M20-A05 | A / 2 | **Novo:** expansão, escopo/hygiene e erros de macro; inspecionar uma expansão conceitualmente. |
| M20-A07 `m20-a07-proc-macro-derive` [roteiro](roteiros/m20.md#m20-a07) | M20-A06 | A / 3 | **Novo:** crate procedural macro e derive customizado; declarar dependências e separar crate consumidor. |
| M20-A08 `m20-a08-macro-procedural-de-atributo` [roteiro](roteiros/m20.md#m20-a08) | M20-A07 | A / 3 | **Novo:** criar macro procedural attribute-like em crate separado e consumi-la no binário do mesmo projeto independente. |
| M20-A09 `m20-a09-macro-procedural-funcao` [roteiro](roteiros/m20.md#m20-a09) | M20-A08 | A / 3 | **Novo:** criar macro procedural function-like em crate separado e consumi-la no binário; comparar com `macro_rules!` sem repetir derive. |

## Módulo 21 — Unsafe e FFI (7 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m21.md).

**Fontes:** Rust Book §20.1; Rust by Example: Unsafe Operations; Rust Edition Guide para as regras da edição 2024.

**Pré-requisitos:** M10, M11, M14 e M16 completos; documentação de invariantes do M09. Não exige M18–M20.

**Critério de domínio:** Declarar precondições e invariantes, restringir o trecho inseguro e explicar a fronteira de validade/ownership de uma API.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M21-A01 `m21-a01-unsafe-e-invariantes` [roteiro](roteiros/m21.md#m21-a01) | Entrada do módulo | A / 2 | **Novo:** propósito do `unsafe`, responsabilidades e limite mínimo; listar invariantes antes de abrir bloco inseguro. |
| M21-A02 `m21-a02-raw-pointers` [roteiro](roteiros/m21.md#m21-a02) | M21-A01 | A / 2 | **Novo:** ponteiros crus const/mut, criação e dereferência insegura; comparar garantias com referências. |
| M21-A03 `m21-a03-funcoes-unsafe` [roteiro](roteiros/m21.md#m21-a03) | M21-A02 | A / 3 | **Novo:** declarar/chamar função `unsafe`, documentar precondições e encapsular a operação em API segura quando possível. |
| M21-A04 `m21-a04-static-mut` [roteiro](roteiros/m21.md#m21-a04) | M21-A03 | A / 2 | **Novo:** acessar estado global mutável, aplicar as regras da edição 2024 e avaliar alternativas seguras. |
| M21-A05 `m21-a05-unions` [roteiro](roteiros/m21.md#m21-a05) | M21-A04 | A / 2 | **Novo:** definir union e acessar seus campos em contexto inseguro; registrar como o aluno garante qual campo está válido. |
| M21-A06 `m21-a06-unsafe-traits` [roteiro](roteiros/m21.md#m21-a06) | M21-A05 | A / 3 | **Novo:** implementar unsafe trait e documentar invariantes exigidas; não repetir blocos `unsafe` nem funções inseguras. |
| M21-A07 `m21-a07-ffi-e-repr-c` [roteiro](roteiros/m21.md#m21-a07) | M21-A06 | A / 3 | **Novo:** `extern`, ABI, `repr(C)`, atributo `#[link]`, vínculo com função C e fronteira de validade/ownership. |

## Módulo 22 — Compatibilidade, ferramentas e desempenho (4 aulas)

**Roteiros detalhados:** [explicação e exemplo obrigatório de cada aula](roteiros/m22.md).

**Fontes:** Rust Book §14 e apêndices; Rust by Example: Compatibility, Meta; Rustlings: clippy.

**Pré-requisitos:** M09, M12 e M13 completos; M15 para comparar trabalho concorrente, se escolhido. Não exige M18–M21.

**Critério de domínio:** Interpretar compatibilidade e comandos de manutenção; produzir uma comparação medida e reproduzível, com limites explícitos.

| Aula / projeto | Pré-requisito imediato | Nível / blocos | Conteúdo e entrega |
|---|---|---|---|
| M22-A01 `m22-a01-manutencao-do-toolchain` [roteiro](roteiros/m22.md#m22-a01) | Entrada do módulo | I / 2 | **Expansão:** retomar `rustfmt` e Clippy do M01 e aprofundar `rustup`, componentes, consulta offline e `cargo fix`/rustfix. Planejar atualização em uma cópia de exercício, inspecionar alterações e executar verificações; todos os comandos continuam sendo executados pelo aluno. |
| M22-A02 `m22-a02-edicoes-e-identificadores` [roteiro](roteiros/m22.md#m22-a02) | M22-A01 | I / 2 | **Novo:** edições, compatibilidade e raw identifiers; interpretar aviso de edição e planejar atualização. |
| M22-A03 `m22-a03-benchmarks` [roteiro](roteiros/m22.md#m22-a03) | M22-A02 | I / 3 | **Novo:** medir loop e iteradores com entradas equivalentes, build de release e várias amostras, evitando trabalho eliminado pelo otimizador. Usar um harness estável explicitamente declarado (`harness = false`) ou crate de benchmark documentado; `#[bench]` é apenas referência nightly. Registrar máquina, toolchain, variabilidade e limites das conclusões. |
| M22-A04 `m22-a04-apendices-e-ecossistema` [roteiro](roteiros/m22.md#m22-a04) | M22-A03 | I / 2 | **Integração:** criar um pequeno CLI de consulta que organiza links/notas sobre palavras-chave, operadores, traits deriváveis, ferramentas, edições, traduções, releases e nightly; resumir como Rust é desenvolvido, sem ensinar novamente tópicos dos módulos. |

## Cobertura, retomadas e limites

- **Fundamentos:** Rust Book §§1–10 distribuídos em M01–M11, com testes simples já no M02; entrada interativa no M08 e jogo de adivinhação no M09. Tipos associados entram no M10 antes do trait `Iterator`.
- **Prática e ferramentas:** testes aprofundados e `minigrep` no M12; closures/iteradores e refatoração do mesmo comportamento no M13; formatação/Clippy no M01 e manutenção no M22; rustdoc/doc tests no M09 e uso contínuo nos projetos posteriores.
- **Integração:** smart pointers no M14, threads no M15 e trait objects/padrões no M16 preparam o servidor progressivo do M17. O servidor cobre os conceitos necessários ao seu funcionamento, não todos os módulos do curso.
- **Aprofundamentos:** async no M18, incluindo visão estrutural de futures/pinning nas duas últimas aulas; tipos/traits/funções avançados no M19; atributos/macros no M20; unsafe/FFI no M21; compatibilidade/desempenho e apêndices como consulta no M22. Não usar unsafe ou macros próprias apenas para preencher cobertura no projeto de integração.
- **Rust by Example:** fundamentos, conversões, fluxo, funções, módulos/crates/Cargo, atributos, generics, escopo, traits, macros, erros, biblioteca padrão, concorrência/I/O, testes, unsafe, compatibilidade e meta distribuídos pelos módulos correspondentes. Selecionar exemplos concretos na preparação de cada aula; esta lista não afirma execução de todos os exemplos.
- **Rustlings:** os tópicos intro, variables, functions, if, primitive_types, vecs, move_semantics, structs, enums, strings, modules, hashmaps, options, error_handling, generics, traits, lifetimes, tests, iterators, smart_pointers, threads, macros, clippy e conversions têm correspondência no percurso. A conclusão de exercícios é registrada separadamente; citar um tópico não equivale a concluir a série.
- **Retomadas intencionais:** testes voltam em novos domínios; erros voltam após traits e após trait objects; `minigrep` volta para comparação entre laço e iteradores; o servidor evolui sem reconstrução integral. A dificuldade deve crescer pela nova decisão exigida, não pelo volume de código redigitado.
- **Escopo do servidor:** HTTP limitado e execução local para aprendizagem, sem compromisso de implementação completa do protocolo. Para um objetivo posterior de aplicação real, escolher outra etapa com crates adequados, requisitos e testes próprios; essa etapa não é pré-requisito do curso.

## Referências técnicas para preparar as aulas

- [Rust Book: testes](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) e [documentação com exemplos executáveis](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html).
- [Ferramentas de desenvolvimento](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html), [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) e [Error](https://doc.rust-lang.org/std/error/trait.Error.html).
- [Traits e tipos de retorno](https://doc.rust-lang.org/book/ch10-02-traits.html), [bounds de traits e lifetimes](https://doc.rust-lang.org/reference/trait-bounds.html) e [servidor multithread do Book](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html).

Conferir a documentação da versão usada ao preparar cada aula, sobretudo APIs de crates externos, regras de edição, benchmarks e exemplos de async.
