# Curso completo de Rust — cronograma revisado

## Objetivo e fontes

O curso organiza os tópicos do [Rust Book](https://doc.rust-lang.org/book/), do [Rust by Example](https://doc.rust-lang.org/rust-by-example/) e dos exercícios oficiais do [Rustlings](https://github.com/rust-lang/rustlings/). O Rust Book orienta a progressão e os projetos; Rust by Example amplia a prática com exemplos da linguagem e da biblioteca padrão; Rustlings inspira exercícios curtos e indica a relação entre seus tópicos e os capítulos do Book no [mapa oficial](https://github.com/rust-lang/rustlings/blob/main/exercises/README.md).

O curso está dividido em módulos de tamanhos variáveis. Cada aula é um projeto Rust independente, com seu próprio `Cargo.toml` e `src/`. A aula pode reaproveitar ideias anteriores, mas nunca arquivos ou diretórios de projetos anteriores.

### Estrutura das pastas

Guardar cada aula dentro da pasta do módulo correspondente. O diretório do módulo usa `mNN`; o diretório da aula usa `aNN-<slug>`, sem repetir o prefixo do módulo:

```text
aulas/
├── m01/
│   ├── a01-primeiro-programa/
│   ├── a02-pacote-cargo/
│   └── ...
├── m02/
│   ├── a01-assinaturas-de-funcoes/
│   ├── a02-expressoes-e-retornos/
│   └── ...
└── ...
```

Cada diretório `aNN-<slug>` é um pacote independente e contém seu próprio `Cargo.toml` e diretório `src/`.

O cronograma revisado tem **20 módulos e 167 aulas**. A quantidade de aulas varia conforme o tamanho dos tópicos e a necessidade de praticar cada etapa; não existe uma cota fixa por módulo.

## Revisão de escopo

- O plano anterior tinha 200 aulas e obrigava dez aulas por módulo. Removi a cota e reduzi repetições: conceitos são ensinados uma vez; as aplicações posteriores estão identificadas como integração.
- O pattern matching básico fica junto de enums. O módulo avançado de padrões trata contextos e formas de padrões que ainda não foram ensinados, sem reapresentar `match` do zero.
- `Arc<Mutex<T>>` é ensinado em concorrência, não em smart pointers e novamente em threads. O módulo de smart pointers prepara `Rc`, `RefCell` e `Weak`.
- O projeto `minigrep` aparece uma vez, no módulo de CLI e testes. Os iteradores são ensinados em um exemplo próprio, sem uma segunda refatoração do mesmo projeto.
- O servidor web final é dividido em experimentos independentes e um projeto de integração. Em cada pasta nova, o tutor reapresenta o scaffolding mínimo necessário; não presume arquivos de aulas anteriores.
- Mantive aulas de integração quando ajudam a consolidar um conjunto de conceitos. Elas não introduzem a teoria de novo; cada linha abaixo diz o que será reutilizado e o que deve ser entregue.

## Regras do tutor e formato de cada aula

1. **O aluno executa os comandos e escreve o código.** O agente não cria projetos, não executa comandos para iniciar ou editar os projetos e não escreve nos arquivos do aluno. Ele apresenta comandos e código na conversa, em etapas curtas, para o aluno executar e digitar.
2. **Validação só após pedido.** Quando solicitada, o agente pode revisar o código que o aluno compartilhar e executar somente verificações, como `cargo check` ou `cargo test`. Não corrige os arquivos diretamente; explica o problema e mostra a mudança para o aluno aplicar.
3. **Abertura obrigatória de toda aula.** Primeiro, o agente informa módulo, número, título, objetivo e nome do projeto. Se Rust ainda não estiver instalado, apresenta antes as instruções oficiais para o aluno instalar o toolchain. Em seguida, mostra comandos concretos para criar o projeto e entrar nele. Os dois caminhos devem ser absolutos, sem placeholders e sem `cd` relativo. Cada projeto fica dentro da pasta de seu módulo, em `aulas/mNN/aNN-<slug>`, omitindo o prefixo `mNN-` do nome listado na tabela. A raiz deste checkout é `/home/amsterdan/developer/learn-rust`.

   Exemplo para a primeira aula:

   ```sh
   cargo new --bin --edition 2024 --vcs none /home/amsterdan/developer/learn-rust/aulas/m01/a01-primeiro-programa
   cd /home/amsterdan/developer/learn-rust/aulas/m01/a01-primeiro-programa
   ```

4. **Projeto novo em toda aula.** Usar o nome de pasta indicado na tabela, omitindo o prefixo do módulo conforme a regra 3. Criar projetos com `--vcs none` para não inicializar um repositório Git dentro de cada aula; o repositório do curso acompanha todos os projetos. Para binários, mostrar `cargo new --bin --edition 2024 --vcs none`; para bibliotecas, `cargo new --lib --edition 2024 --vcs none`, sempre com o caminho absoluto. Se precisar de dependências, apresentar as alterações exatas em `Cargo.toml`; o aluno as digita.
5. **Escopo explícito.** A coluna “Conteúdo e resultado” é o contrato da aula. Ensinar os conceitos listados e pedir o resultado indicado; não acrescentar um novo tópico avançado sem anunciá-lo e explicar por que é necessário. Uma menção de prévia não substitui a aula em que o tema é ensinado.
6. **Conceito novo versus integração.** **Novo:** introduz um conceito pela primeira vez. **Integração:** reutiliza conceitos ensinados antes, sem repetir a explicação completa. **Expansão:** aprofunda um conceito anterior e explicita qual detalhe novo será acrescentado. Combinações como **Novo + integração** apresentam um conceito e praticam outros já ensinados. O tutor pode relembrar pré-requisitos em poucas frases.
7. **Independência entre projetos.** Em uma integração, o tutor reapresenta o código-base e os arquivos necessários dentro do projeto novo. O aluno não precisa navegar para outra pasta, copiar arquivos ocultos ou manter um workspace compartilhado.
8. **Ritmo da aula.** Apresentar um objetivo por vez, mostrar o conteúdo de `Cargo.toml` e `src/` necessário, propor um desafio curto e informar como o aluno pode verificar o resultado. Não despejar um capítulo inteiro nem avançar automaticamente para a aula seguinte.
9. Usar Rust 2024 nos projetos novos. Identificar explicitamente quando uma aula exige crate externo, runtime, rede, nightly ou ferramenta extra; informar a versão/dependência usada e explicar o propósito antes dos comandos.

---

## Módulo 01 — Toolchain, Cargo e fundamentos da linguagem (9 aulas)

**Fontes:** Rust Book §§1 e 3.1–3.2; Rust by Example: Hello World, Primitives, Variable Bindings, Types; Rustlings: intro, variables, primitive_types.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M01-A01 `m01-a01-primeiro-programa` | **Novo:** instalar Rust com `rustup` se necessário; distinguir `rustc` e Cargo; compilar e executar um `Hello, world!` mínimo. |
| M01-A02 `m01-a02-pacote-cargo` | **Novo:** usar `cargo new`, reconhecer `Cargo.toml`, `src/main.rs` e `target/`; executar `cargo run`, `cargo check` e `cargo build` no mesmo pacote. |
| M01-A03 `m01-a03-saida-formatada` | **Novo:** usar `println!`, placeholders posicionais/nomeados com `Debug` (`{:?}`/`{nome:?}`); imprimir um pequeno recibo. Deixar a implementação de `Display` para M10. |
| M01-A04 `m01-a04-variaveis` | **Novo:** declarar com `let`, anotar tipos e usar `mut`; produzir um programa com estado imutável e uma atualização deliberadamente mutável. |
| M01-A05 `m01-a05-shadowing-e-escopo` | **Novo:** distinguir shadowing, mutabilidade e escopo de bloco; transformar uma entrada em etapas usando o mesmo nome sem confundir os mecanismos. |
| M01-A06 `m01-a06-constantes` | **Novo:** declarar `const`, anotar tipo e diferenciar constante de binding; calcular e exibir um valor configurado por constante. |
| M01-A07 `m01-a07-tipos-numericos` | **Novo:** inteiros, floats, sinais, tamanhos, literais, inferência e overflow; escolher tipos explícitos para um conversor simples. |
| M01-A08 `m01-a08-bool-e-char` | **Novo:** `bool`, `char` e Unicode; distinguir caractere, escalar Unicode e byte sem entrar ainda em UTF-8 de `String`. |
| M01-A09 `m01-a09-tuplas-e-arrays` | **Novo:** tuplas, arrays fixos, acesso e desestruturação simples; deixar slices para M04. |

## Módulo 02 — Funções, expressões e controle de fluxo (8 aulas)

**Fontes:** Rust Book §§3.3–3.5; Rust by Example: Expressions, Flow of Control, Functions; Rustlings: functions, if.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M02-A01 `m02-a01-assinaturas-de-funcoes` | **Novo:** declarar e chamar funções, parâmetros tipados e convenções de nomes; separar cálculos em duas funções pequenas. |
| M02-A02 `m02-a02-expressoes-e-retornos` | **Novo:** distinguir expressão de statement, retorno explícito e expressão final; escrever funções que retornam valores sem `return` desnecessário. |
| M02-A03 `m02-a03-operadores` | **Novo:** operadores aritméticos, comparação, lógica e precedência; calcular e validar uma expressão com tipos compatíveis. |
| M02-A04 `m02-a04-if-como-expressao` | **Novo:** `if`/`else`, condições booleanas e ramos do mesmo tipo; classificar entradas em categorias. |
| M02-A05 `m02-a05-loop-e-break` | **Novo:** `loop`, `break` com valor e labels; interromper uma busca quando a condição for satisfeita. |
| M02-A06 `m02-a06-while` | **Novo:** laço condicionado, atualização de contador e encerramento; simular uma contagem regressiva. |
| M02-A07 `m02-a07-for-e-intervalos` | **Novo:** `for`, intervalos abertos/fechados e iteração sobre array; somar valores sem índices manuais. |
| M02-A08 `m02-a08-fizzbuzz` | **Integração:** combinar funções, operadores, `if` e laços em FizzBuzz; nenhuma sintaxe nova. |

## Módulo 03 — Ownership e movimentos (6 aulas)

**Fontes:** Rust Book §4.1; Rust by Example: Scoping Rules; Rustlings: move_semantics.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M03-A01 `m03-a01-escopo-e-drop` | **Novo:** validade, escopo e descarte automático; explicar em que ponto cada valor deixa de poder ser usado. |
| M03-A02 `m03-a02-copy-e-move` | **Novo:** diferença entre cópia de tipos `Copy` e movimento de `String`; prever quais bindings continuam válidos após atribuição. |
| M03-A03 `m03-a03-clone-explicito` | **Novo:** usar `clone` quando uma cópia de dados é intencional e identificar o custo de duplicação; comparar com movimento. |
| M03-A04 `m03-a04-ownership-em-funcoes` | **Novo:** passar e devolver ownership por funções; escrever chamadas sem perder valores que ainda serão necessários. |
| M03-A05 `m03-a05-ownership-em-agregados` | **Novo:** acompanhar ownership em tuplas e arrays que contêm `String`; localizar movimentos do agregado e de componentes desestruturados sem depender de structs ainda não ensinadas. |
| M03-A06 `m03-a06-modelo-de-transferencia` | **Integração:** construir um fluxo de pedidos que transfere, devolve ou clona dados conforme a intenção; justificar cada decisão de ownership. |

## Módulo 04 — Referências, borrowing e slices (5 aulas)

**Fontes:** Rust Book §§4.2–4.3; Rust by Example: Scoping Rules; Rustlings: primitive_types e move_semantics.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M04-A01 `m04-a01-borrowing-imutavel` | **Novo:** `&T`, emprestar sem mover e validade da referência; escrever função que consulta uma `String` sem tomá-la. |
| M04-A02 `m04-a02-borrowing-mutavel` | **Novo:** `&mut T` e regra de exclusividade; atualizar dado por referência sem manter empréstimos conflitantes. |
| M04-A03 `m04-a03-regras-e-referencias-pendentes` | **Novo:** convivência de referências, escopo do empréstimo e rejeição de dangling references; explicar a mensagem do compilador em um caso inválido. |
| M04-A04 `m04-a04-slices-de-string-e-array` | **Novo:** `&str`, slices de arrays, intervalos e limites; obter trechos sem copiar a sequência. |
| M04-A05 `m04-a05-api-de-slices` | **Integração:** criar funções que aceitam `&str` e `&[T]` para várias entradas; concluir com um analisador simples de texto sem cópias. |

## Módulo 05 — Structs e métodos (8 aulas)

**Fontes:** Rust Book §5; Rust by Example: Custom Types; Rustlings: structs.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M05-A01 `m05-a01-structs-nomeadas` | **Novo:** definir, instanciar e acessar campos de uma struct; modelar uma entidade com dados relacionados. |
| M05-A02 `m05-a02-atualizar-structs` | **Novo:** mutabilidade da instância, field init shorthand e sintaxe de atualização; prever movimentos de campos reaproveitados. |
| M05-A03 `m05-a03-tuple-e-unit-structs` | **Novo:** tuple structs e unit-like structs; escolher entre formas nomeadas e posicionais para um tipo pequeno. |
| M05-A04 `m05-a04-debug-derive` | **Novo:** atributo `derive(Debug)` e formatação `{:?}`/`{:#?}`; inspecionar uma struct. Não ensinar ainda macros personalizadas. |
| M05-A05 `m05-a05-metodos` | **Novo:** `impl`, métodos com `&self` e chamada por ponto; calcular uma propriedade a partir dos campos. |
| M05-A06 `m05-a06-metodos-mutaveis` | **Novo:** métodos com `&mut self` e receiver; implementar uma alteração que mantenha o estado válido. |
| M05-A07 `m05-a07-funcoes-associadas` | **Novo:** funções associadas e `Self`; criar construtor sem reapresentar métodos de instância. |
| M05-A08 `m05-a08-retangulo` | **Integração:** definir struct, construtor e métodos para área/comparação de retângulos; não incluir enums nem traits próprios. |

## Módulo 06 — Enums, Option e match básico (6 aulas)

**Fontes:** Rust Book §6; Rust by Example: Custom Types, Flow of Control; Rustlings: enums, options.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M06-A01 `m06-a01-enums` | **Novo:** definir enums, variantes e valores; modelar estados mutuamente exclusivos sem strings mágicas. |
| M06-A02 `m06-a02-dados-em-variantes` | **Novo:** variantes com campos nomeados ou posicionais; associar dados à variante correta. |
| M06-A03 `m06-a03-option` | **Novo:** `Option<T>`, `Some` e `None`; representar ausência sem valor sentinela. |
| M06-A04 `m06-a04-match-exaustivo` | **Novo:** `match` com enums e `Option`, extração simples e exaustividade; tratar todos os casos de uma operação. |
| M06-A05 `m06-a05-if-let-e-let-else` | **Novo:** usar `if let` para um caso e `let...else` para saída antecipada; não cobrir ainda a taxonomia completa de padrões. |
| M06-A06 `m06-a06-enum-de-comandos` | **Integração:** representar comandos pré-definidos por enum e despachá-los com `match`; reforçar tipos de domínio e exaustividade, sem introduzir parsing de texto. |

## Módulo 07 — Coleções da biblioteca padrão (7 aulas)

**Fontes:** Rust Book §8; Rust by Example: Std library types; Rustlings: vecs, strings, hashmaps.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M07-A01 `m07-a01-vec-criar-acessar` | **Novo:** criar, adicionar e acessar `Vec<T>` por índice ou `get`; tratar ausência retornada por `get`. |
| M07-A02 `m07-a02-vec-mutacao-e-ownership` | **Novo:** remover/alterar elementos e percorrer por referência compartilhada ou mutável; preservar o vetor quando ele ainda será usado. Adiar a comparação sistemática `iter`/`iter_mut`/`into_iter` para M13. |
| M07-A03 `m07-a03-string-e-utf8` | **Novo:** `String` versus `str`, UTF-8, bytes, `len` e por que índice numérico não significa caractere. |
| M07-A04 `m07-a04-operacoes-com-string` | **Novo:** `push`, `push_str`, `format!`, concatenação e iteração por `chars`; construir uma linha com tipos padrão já formatáveis. Implementar `Display` para tipo próprio fica para M10. |
| M07-A05 `m07-a05-hashmap` | **Novo:** criar, ler, atualizar, substituir e remover valores em `HashMap<K,V>`. |
| M07-A06 `m07-a06-entry-e-contagem` | **Novo:** API `entry`, `or_insert` e `or_default`; contar palavras sem uma busca seguida de inserção duplicada. |
| M07-A07 `m07-a07-placar-de-jogos` | **Integração:** agregar resultados de partidas em um `HashMap` e apresentar um placar; aplicar coleções em um domínio distinto da contagem de palavras. |

## Módulo 08 — Erros e casts primitivos (6 aulas)

**Fontes:** Rust Book §9 e passagens de §§2 e 12; Rust by Example: Error handling e casts em Conversion; Rustlings: error_handling e options. Traits de conversão aparecem no M10, depois de traits básicos.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M08-A01 `m08-a01-panic-e-backtrace` | **Novo:** panic, backtrace e falhas irrecuperáveis; distinguir bug/invariante quebrada de erro esperado. Não usar `panic!` para validar entrada normal. |
| M08-A02 `m08-a02-result` | **Novo:** `Result<T,E>`, `Ok`, `Err` e `match`; devolver falha recuperável em vez de abortar. |
| M08-A03 `m08-a03-propagacao-com-interrogacao` | **Novo:** propagar `Result` com `?` quando o tipo de erro de retorno é compatível; eliminar tratamento repetitivo sem apagar a falha. Conversões entre erros ficam para depois de M10. |
| M08-A04 `m08-a04-erros-proprios` | **Novo:** enum de erro próprio como tipo de retorno; separar categorias de falha sem implementar ainda traits de formatação/erro nem usar trait objects. |
| M08-A05 `m08-a05-compor-option-e-result` | **Novo:** compor operações opcionais/falíveis com métodos básicos e converter ausência em erro; evitar `unwrap` indiscriminado. |
| M08-A06 `m08-a06-casts` | **Novo:** casts numéricos com `as`, truncamento e verificações de intervalo; não confundir cast com conversão validada. |

## Módulo 09 — Módulos, crates e Cargo (9 aulas)

**Fontes:** Rust Book §§7 e 14; Rust by Example: Modules, Crates, Cargo; Rustlings: modules.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M09-A01 `m09-a01-arvore-de-modulos` | **Novo:** `mod`, árvore, escopo e caminhos entre módulos dentro de um único arquivo. |
| M09-A02 `m09-a02-caminhos-e-visibilidade` | **Novo:** caminhos `crate`, `self`, `super`, público/privado e privacidade de campos; explicar por que um item não é acessível. |
| M09-A03 `m09-a03-use-e-reexport` | **Novo:** `use`, aliases, `pub use` e imports glob; organizar API pública evitando imports glob indiscriminados. |
| M09-A04 `m09-a04-modulos-em-arquivos` | **Novo:** mover módulos para arquivos e subdiretórios mantendo caminhos e visibilidade. |
| M09-A05 `m09-a05-pacote-binario-e-biblioteca` | **Novo:** pacote, crate, múltiplos binários e `src/lib.rs`; criar biblioteca e binário de demonstração no mesmo pacote. |
| M09-A06 `m09-a06-dependencias-e-lockfile` | **Novo:** adicionar crate externo no `Cargo.toml`, papel do `Cargo.lock`, faixas de versão e consulta à documentação. |
| M09-A07 `m09-a07-workspaces` | **Novo:** workspace Cargo com membros e dependência local; identificar quando workspace ajuda e quando um projeto independente basta. |
| M09-A08 `m09-a08-perfis-install-e-publicacao` | **Novo:** perfis debug/release, `cargo install`, comandos Cargo estendidos e metadados para publicar; preparar o pacote sem publicar nada externamente. |
| M09-A09 `m09-a09-jogo-de-adivinhacao` | **Integração:** recriar o projeto do Book usando entrada padrão, `rand`, `parse`, `Result`, enum `Ordering`, `match` e loop. `parse` é usado como API pronta; sua implementação via `FromStr` fica para M10-A12. Reforçar Cargo sem repetir a teoria de dependências. Exibir valores com `Debug` até `Display` ser ensinado em M10. |

## Módulo 10 — Generics e traits (13 aulas)

**Fontes:** Rust Book §§10.1–10.2; Rust by Example: Generics, Traits, Conversion; Rustlings: generics, traits, conversions.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M10-A01 `m10-a01-funcoes-genericas` | **Novo:** parâmetros de tipo em funções; substituir duas funções duplicadas por uma implementação genérica. |
| M10-A02 `m10-a02-structs-e-enums-genericos` | **Novo:** tipos e enums genéricos com um ou mais parâmetros; usar valores de tipos diferentes sem duplicar definições. |
| M10-A03 `m10-a03-impl-generico` | **Novo:** `impl<T>`, métodos disponíveis para todos os tipos e métodos restritos a um tipo específico. |
| M10-A04 `m10-a04-definir-e-implementar-trait` | **Novo:** definir trait, implementar em tipo próprio e aplicar regra de coerência/orphan rule em casos básicos. |
| M10-A05 `m10-a05-metodos-padrao` | **Novo:** método padrão de trait e sobrescrita; compartilhar comportamento sem herança de classes. |
| M10-A06 `m10-a06-bounds-e-where` | **Novo:** trait bounds, mais de um bound e cláusula `where`; restringir uma função genérica ao comportamento exigido. |
| M10-A07 `m10-a07-impl-trait-entrada` | **Novo:** `impl Trait` em parâmetro e sua relação com genéricos; escolher a forma conforme a assinatura. |
| M10-A08 `m10-a08-impl-trait-retorno` | **Novo:** `impl Trait` no retorno para ocultar tipo concreto; devolver uma única família de implementação por função. |
| M10-A09 `m10-a09-display-e-tostring` | **Novo:** implementar `Display` e usar `ToString`; formatar tipo próprio para pessoas e relacionar o método gerado a partir de `Display`. |
| M10-A10 `m10-a10-from-e-into` | **Novo:** implementar `From` e usar `Into` para conversão infalível; conectar os traits padrão às regras de coerência. |
| M10-A11 `m10-a11-tryfrom-e-tryinto` | **Novo:** implementar conversão validada com `TryFrom`/`TryInto` e preservar a informação de falha. |
| M10-A12 `m10-a12-fromstr-e-parse` | **Novo:** implementar `FromStr` e consumir `parse`; relacionar a saída a `Result` aprendido no M08. |
| M10-A13 `m10-a13-asref-asmut` | **Novo:** usar `AsRef`/`AsMut` para expor vistas emprestadas de tipos diferentes; distinguir essa conversão de referência de `From`/`Into`. |

## Módulo 11 — Lifetimes e recursos avançados de tipos e traits (10 aulas)

**Fontes:** Rust Book §10.3 e §§18–20; Rust by Example: Scoping Rules, Generics, Traits; Rustlings: lifetimes, traits.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M11-A01 `m11-a01-validade-e-lifetime` | **Novo:** lifetime como relação entre referências; mostrar que anotação não prolonga a vida do dado. |
| M11-A02 `m11-a02-lifetime-em-funcoes` | **Novo:** anotar relações entre parâmetros e retorno; corrigir assinatura que poderia devolver referência inválida. |
| M11-A03 `m11-a03-struct-com-referencia` | **Novo:** armazenar referência em struct com lifetime explícito e garantir que o dado viva o bastante. |
| M11-A04 `m11-a04-elision-e-static` | **Novo:** regras de elision e significado/contextos de `'static`; não tratar `'static` como solução universal. |
| M11-A05 `m11-a05-lifetime-bounds-e-higher-ranked` | **Novo:** bounds de lifetime, subtipagem e higher-ranked trait bounds em uma API de callback com ponteiro de função; closures ficam para M13. |
| M11-A06 `m11-a06-tipos-associados` | **Novo:** tipos associados e diferença para parâmetros genéricos de trait; implementar uma abstração pequena. |
| M11-A07 `m11-a07-parametros-padrao-e-operadores` | **Novo:** parâmetros de tipo padrão e sobrecarga de operador; demonstrar quando a sintaxe padrão ajuda. |
| M11-A08 `m11-a08-coerencia-e-traits-avancados` | **Expansão:** sintaxe totalmente qualificada, supertraits e newtype como solução para casos da regra do orphan; aplicar a regra básica de M10 sem reapresentá-la. |
| M11-A09 `m11-a09-alias-e-never` | **Novo:** type aliases e never type `!`; nomear tipos compostos e reconhecer funções que não retornam. |
| M11-A10 `m11-a10-dst` | **Novo:** tipos dimensionados dinamicamente, `str` e ponteiros que carregam metadados; reconhecer por que tipos sem tamanho conhecido não vivem diretamente em variáveis locais. `dyn Trait` será apresentado em M17. |

## Módulo 12 — Testes e programa de linha de comando (9 aulas)

**Fontes:** Rust Book §§11–12; Rust by Example: Testing; Rustlings: tests.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M12-A01 `m12-a01-assertions` | **Novo:** `#[test]`, `assert!`, `assert_eq!` e `assert_ne!`; validar comportamento esperado. |
| M12-A02 `m12-a02-testes-de-falha` | **Novo:** `should_panic`, mensagens e testes para entrada inválida; não usar panic como tratamento de erro em produção. |
| M12-A03 `m12-a03-harness-e-filtros` | **Novo:** `cargo test`, filtros, saída, execução paralela e `ignore`; selecionar e interpretar testes. |
| M12-A04 `m12-a04-testes-unitarios` | **Novo:** módulo `tests`, visibilidade e organização junto à implementação; testar funções privadas pelo módulo. |
| M12-A05 `m12-a05-testes-de-integracao` | **Novo:** diretório `tests/`, testes externos ao crate e `[dev-dependencies]`; testar API pública. |
| M12-A06 `m12-a06-argumentos-e-ambiente` | **Novo:** `std::env::args`, variáveis de ambiente e validação de argumentos; produzir mensagens de uso. |
| M12-A07 `m12-a07-arquivos-e-stderr` | **Novo:** abrir, ler e escrever arquivos; lidar com caminhos, UTF-8, erros de I/O, `eprintln!` e código de saída. |
| M12-A08 `m12-a08-paths-e-processos` | **Novo:** `Path`/`PathBuf`, operações de filesystem e iniciar um processo filho com `std::process::Command`; tratar saída e falha do processo. |
| M12-A09 `m12-a09-minigrep` | **Integração:** organizar biblioteca/binário para busca textual em arquivo, testar lógica e oferecer busca sensível a maiúsculas configurável por ambiente; não reensinar Cargo ou `Result`. |

## Módulo 13 — Closures e iteradores (8 aulas)

**Fontes:** Rust Book §13; Rust by Example: Functions; Rustlings: iterators.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M13-A01 `m13-a01-sintaxe-de-closures` | **Novo:** closures, parâmetros, inferência e anotação opcional; substituir uma função local simples. |
| M13-A02 `m13-a02-captura-de-ambiente` | **Novo:** captura imutável, mutável e por movimento; identificar qual acesso a closure exige. |
| M13-A03 `m13-a03-fn-fnmut-fnonce` | **Novo:** contratos `Fn`, `FnMut` e `FnOnce`; associar cada trait às capturas e chamadas possíveis. |
| M13-A04 `m13-a04-iterator-next` | **Novo:** trait `Iterator`, `next` e estado de consumo; implementar sequência simples. |
| M13-A05 `m13-a05-iter-iter-mut-into-iter` | **Novo:** diferença entre iterar por referência, mutável e por valor; relacionar a ownership sem repetir M07. |
| M13-A06 `m13-a06-map-filter` | **Novo:** adaptadores lazy `map`/`filter`; transformar e selecionar dados. |
| M13-A07 `m13-a07-collect-e-fold` | **Novo:** `collect`, tipo de destino, `zip`, `enumerate`, `fold` e composição curta; não incluir implementação de iterator customizado. |
| M13-A08 `m13-a08-iterator-proprio` | **Novo:** implementar `Iterator` para uma sequência própria e definir item/fim; não incluir benchmark ou alegações de desempenho nesta aula. |

## Módulo 14 — Smart pointers e mutabilidade interior (8 aulas)

**Fontes:** Rust Book §15; Rust by Example: Std library types; Rustlings: smart_pointers.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M14-A01 `m14-a01-box-e-tipo-recursivo` | **Novo:** `Box<T>`, alocação no heap e tipo recursivo com tamanho conhecido. |
| M14-A02 `m14-a02-deref-e-coercao` | **Novo:** `Deref`, operador `*` e deref coercion; implementar wrapper que se comporta como referência. |
| M14-A03 `m14-a03-drop` | **Novo:** trait `Drop`, limpeza customizada e `std::mem::drop`; observar momento de descarte. |
| M14-A04 `m14-a04-rc` | **Novo:** ownership compartilhado single-thread com `Rc<T>` e contagem forte. |
| M14-A05 `m14-a05-cell-e-refcell` | **Novo:** mutabilidade interior com `Cell<T>` e `RefCell<T>`; contrastar checagem estática e runtime. |
| M14-A06 `m14-a06-rc-refcell` | **Novo:** combinar compartilhamento e mutação interior em grafo local; restringir exemplo a uma thread. |
| M14-A07 `m14-a07-weak-e-ciclos` | **Novo:** `Weak<T>`, referência fraca e prevenção de ciclos que impedem descarte. |
| M14-A08 `m14-a08-cow-e-escolha-de-pointer` | **Novo:** `Cow` e critérios de escolha entre valor, `Box`, `Rc` e interior mutability; sem introduzir `Arc<Mutex<T>>`. |

## Módulo 15 — Threads e concorrência (8 aulas)

**Fontes:** Rust Book §16; Rust by Example: Std misc; Rustlings: threads.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M15-A01 `m15-a01-spawn-e-join` | **Novo:** `thread::spawn`, `JoinHandle` e `join`; aguardar resultado de trabalho concorrente. |
| M15-A02 `m15-a02-move-em-threads` | **Novo:** capturar dados com closure `move`; resolver ownership ao enviar dados para thread. |
| M15-A03 `m15-a03-panics-em-threads` | **Novo:** tratar resultado de `join` e distinguir panic da thread de falha do processo. |
| M15-A04 `m15-a04-canais-mpsc` | **Novo:** enviar e receber mensagens por canal; tratar encerramento do receiver. |
| M15-A05 `m15-a05-varios-produtores` | **Novo:** clonar transmissores e coordenar vários produtores com um consumidor. |
| M15-A06 `m15-a06-mutex` | **Novo:** `Mutex<T>`, guard e exclusão mútua em um caso de estado compartilhado. |
| M15-A07 `m15-a07-arc-mutex` | **Novo:** combinar `Arc<Mutex<T>>` entre threads; explicar contagem atômica e locking sem repetir `Rc`. |
| M15-A08 `m15-a08-send-sync-e-modelos` | **Novo + integração:** auto traits `Send`/`Sync`; comparar message passing e estado compartilhado usando threads/canais/mutex já estudados, sem construir ainda um pool. |

## Módulo 16 — Programação assíncrona (11 aulas)

**Fontes:** Rust Book §17; exemplos usam `trpl` conforme o Book e identificam o runtime/dependências subjacentes.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M16-A01 `m16-a01-concorrencia-e-io` | **Novo:** diferenciar concorrência/paralelismo e trabalho CPU-bound/I/O-bound; escrever uma simulação pequena e escolher um caso em que async ajuda. |
| M16-A02 `m16-a02-future-async-await` | **Novo:** future como valor pendente, `async fn`/bloco e `.await`; compor uma operação que retorna resultado posterior. |
| M16-A03 `m16-a03-runtime-e-primeira-future` | **Novo:** executar async com `trpl::block_on`, adicionar a dependência da aula e consultar runtime; não apresentar Tokio como API de `std`. |
| M16-A04 `m16-a04-funcao-async-de-rede` | **Novo:** decompor uma função de busca de página em chamadas async e tratar resultado ausente; informar que a execução de rede depende de acesso externo. |
| M16-A05 `m16-a05-spawn-task-e-handle` | **Novo:** `spawn_task`, `sleep`, handle e espera pela conclusão; comparar conceitualmente com thread sem misturar APIs. |
| M16-A06 `m16-a06-join-select` | **Novo:** usar `join!` para aguardar resultados combinados e `select` para reagir à primeira conclusão; compor um timeout a partir dessas operações. |
| M16-A07 `m16-a07-yield-e-trabalho-bloqueante` | **Novo:** mostrar starvation, ceder controle ao runtime e identificar trabalho síncrono que bloqueia tarefas async. |
| M16-A08 `m16-a08-canais-async` | **Novo:** enviar/receber mensagens por canal assíncrono; tratar espera e encerramento do receiver. |
| M16-A09 `m16-a09-streams` | **Novo:** streams como sequência temporal, uso de `Stream`/`StreamExt` e diferença para `Iterator` síncrono. |
| M16-A10 `m16-a10-future-poll` | **Novo:** entender `Future::Output`, `poll`, `Poll`, `Context` e `Waker` como visão estrutural; reconhecer por que normalmente se usa `.await` em vez de chamar `poll`. |
| M16-A11 `m16-a11-pin-unpin-stream` | **Novo:** relacionar `Pin`/`Unpin` à segurança de mover futures e examinar `Stream::poll_next`; conectar a interface de baixo nível ao uso de streams do M16-A09. |

## Módulo 17 — Orientação a objetos e padrões avançados (7 aulas)

**Fontes:** Rust Book §§18–19; Rust by Example: Custom Types, Flow of Control; Rustlings: enums.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M17-A01 `m17-a01-encapsulamento-rust` | **Novo:** encapsulamento com structs, métodos e privacidade; comparar composição em Rust com ideias OO sem ensinar sintaxe de structs de novo. |
| M17-A02 `m17-a02-trait-objects-e-dispatch` | **Novo:** `dyn Trait`, dispatch dinâmico e coleção de tipos diferentes por comportamento; usar `Box<dyn Error>` como aplicação; não repetir traits genéricos do M10. |
| M17-A03 `m17-a03-padrao-state` | **Novo:** implementar o padrão State e comparar transições por objetos com enum; foco nos trade-offs, não em reensinar traits. |
| M17-A04 `m17-a04-refutabilidade` | **Novo:** classificar padrões irrefutáveis e refutáveis; justificar por que cada classe é aceita em certos lugares. |
| M17-A05 `m17-a05-contextos-de-padroes` | **Novo:** aplicar essa classificação em `let`, parâmetros, `for`, `if let`, `while let` e `match`; reutilizar sintaxe já conhecida sem reensinar `if let`. |
| M17-A06 `m17-a06-desestruturacao-completa` | **Novo:** padrões de tuplas, structs, enums, referências, arrays/slices, `_` e `..`; extrair partes sem mover dados acidentalmente. |
| M17-A07 `m17-a07-guards-alternativas-e-at` | **Novo:** padrões alternativos, guards, intervalos e `@`; não repetir match básico ou `if let`. |

## Módulo 18 — Atributos, macros e documentação (10 aulas)

**Fontes:** Rust Book §§14 e 20.5; Rust by Example: Macros, Attributes, Meta; Rustlings: macros.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M18-A01 `m18-a01-atributos` | **Novo:** atributos externos/internos, escopo e metadados; reconhecer atributos de crate e distingui-los de metadados Cargo, `derive` e `test`; mostrar `#![feature(...)]` em snippet de referência marcado como nightly, sem exigir que o projeto stable compile esse trecho nem reensinar `Debug`. |
| M18-A02 `m18-a02-cfg` | **Novo:** `cfg`/`cfg_attr`, compilação condicional e seleção de plataforma; demonstrar dois caminhos sem exigir duas máquinas. |
| M18-A03 `m18-a03-lints` | **Novo:** atributos de lint `allow`/`warn`/`deny`/`forbid` e escopo; preferir corrigir aviso a silenciá-lo. |
| M18-A04 `m18-a04-macro-rules-basico` | **Novo:** `macro_rules!`, matcher/transcriber e fragmentos; criar macro pequena sem repetição. |
| M18-A05 `m18-a05-macros-repeticao-e-dsl` | **Novo:** repetição, separadores, interface variádica e DSL pequena; não reapresentar a sintaxe-base. |
| M18-A06 `m18-a06-higiene-de-macros` | **Novo:** expansão, escopo/hygiene e erros de macro; inspecionar uma expansão conceitualmente. |
| M18-A07 `m18-a07-proc-macro-derive` | **Novo:** crate procedural macro e derive customizado; declarar dependências e separar crate consumidor. |
| M18-A08 `m18-a08-macro-procedural-de-atributo` | **Novo:** criar macro procedural attribute-like em crate separado e consumi-la no binário do mesmo projeto independente. |
| M18-A09 `m18-a09-macro-procedural-funcao` | **Novo:** criar macro procedural function-like em crate separado e consumi-la no binário; comparar com `macro_rules!` sem repetir derive. |
| M18-A10 `m18-a10-rustdoc-e-playground` | **Novo:** `///`, exemplos/documentação gerada com `rustdoc`/`cargo doc` e playground; validar exemplo documentado com doc test. |

## Módulo 19 — Unsafe, FFI, compatibilidade e ferramentas (12 aulas)

**Fontes:** Rust Book §§14, 20 e apêndices; Rust by Example: Unsafe Operations, Compatibility, Meta; Rustlings: clippy.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M19-A01 `m19-a01-unsafe-e-invariantes` | **Novo:** propósito do `unsafe`, responsabilidades e limite mínimo; listar invariantes antes de abrir bloco inseguro. |
| M19-A02 `m19-a02-raw-pointers` | **Novo:** ponteiros crus const/mut, criação e dereferência insegura; comparar garantias com referências. |
| M19-A03 `m19-a03-funcoes-unsafe` | **Novo:** declarar/chamar função `unsafe`, documentar precondições e encapsular a operação em API segura quando possível. |
| M19-A04 `m19-a04-static-mut` | **Novo:** acessar estado global mutável, aplicar as regras da edição 2024 e avaliar alternativas seguras. |
| M19-A05 `m19-a05-unions` | **Novo:** definir union e acessar seus campos em contexto inseguro; registrar como o aluno garante qual campo está válido. |
| M19-A06 `m19-a06-unsafe-traits` | **Novo:** implementar unsafe trait e documentar invariantes exigidas; não repetir blocos `unsafe` nem funções inseguras. |
| M19-A07 `m19-a07-ffi-e-repr-c` | **Novo:** `extern`, ABI, `repr(C)`, atributo `#[link]`, vínculo com função C e fronteira de validade/ownership. |
| M19-A08 `m19-a08-rustfmt-clippy-rustup` | **Novo:** aplicar `rustfmt`, ler Clippy e usar `rustup`/`rustfix`; listar comandos para o aluno, sem executá-los pelo agente. |
| M19-A09 `m19-a09-edicoes-e-identificadores` | **Novo:** edições, compatibilidade e raw identifiers; interpretar aviso de edição e planejar atualização. |
| M19-A10 `m19-a10-funcoes-avancadas` | **Novo:** ponteiros de função e closures retornadas, incluindo boxing quando necessário; pressupõe closures básicas do M13 e trait objects do M17. |
| M19-A11 `m19-a11-benchmarks` | **Novo:** medir e comparar um loop com iteradores; distinguir `cargo bench` com harness estável de atributos de benchmark que exigem nightly. Manter trechos nightly como referência separada e não tirar conclusões de performance sem medir. |
| M19-A12 `m19-a12-apendices-e-ecossistema` | **Integração:** criar um pequeno CLI de consulta que organiza links/notas sobre palavras-chave, operadores, traits deriváveis, ferramentas, edições, traduções, releases e nightly; resumir como Rust é desenvolvido, sem ensinar novamente tópicos dos módulos. |

## Módulo 20 — Projeto final: servidor web multithread (7 aulas)

**Fontes:** Rust Book §21, com conceitos de ownership, erros, traits, canais, threads e testes; Rust by Example: Std misc e Testing.

| Aula / projeto | Conteúdo e resultado |
|---|---|
| M20-A01 `m20-a01-listener-tcp` | **Novo:** `TcpListener`, aceitar conexão local e ler fluxo de bytes; entregar servidor de uma conexão sem parser HTTP. |
| M20-A02 `m20-a02-requisicao-e-resposta-http` | **Novo:** ler requisição limitada, identificar rota e montar status/headers/corpo; entregar resposta HTTP simples. Reapresentar listener no projeto novo. |
| M20-A03 `m20-a03-servidor-sequencial` | **Integração:** aceitar várias conexões sequencialmente e observar o bloqueio; entregar loop com encerramento controlado. |
| M20-A04 `m20-a04-thread-por-conexao` | **Integração:** atender cada conexão em thread e explicar custo/limite; recriar listener e parsing no novo pacote. |
| M20-A05 `m20-a05-pool-de-threads` | **Novo:** projeto de pool inspirado no Book: fila por canal, workers e distribuição de jobs; não pressupor arquivos do M15. |
| M20-A06 `m20-a06-shutdown-e-testes` | **Novo:** encerramento ordenado, `Drop`, tratamento de erro e testes dos componentes do pool/servidor. |
| M20-A07 `m20-a07-servidor-final` | **Integração:** produzir servidor completo multithread com documentação e testes básicos em um projeto novo; demonstrar todos os módulos sem acrescentar protocolo além do escopo do Book. |

## Verificação de cobertura e não redundância

- Rust Book: capítulos 1–21, incluindo jogo de adivinhação, `minigrep`, Cargo, testes, closures/iteradores, smart pointers, threads, async, OO, padrões, recursos avançados e servidor web; os apêndices são tratados como consulta no M19-A12.
- Rust by Example: Hello World, tipos primitivos/compostos, bindings, conversões, expressões, fluxo, funções/métodos/closures, módulos/crates/Cargo, atributos, generics, escopo, traits, macros, erros, tipos da biblioteca padrão, threads/canais/arquivos, testes, unsafe, compatibilidade e meta (documentação/playground/benchmark).
- Rustlings: intro, variables, functions, if, primitive types, vecs, move semantics, structs, enums, strings, modules, hashmaps, options, error handling, generics, traits, lifetimes, tests, iterators, smart pointers, threads, macros, clippy e conversions. As aulas com **Integração** reforçam esses exercícios sem criar uma segunda explicação do mesmo conceito.
- Os pontos de repetição intencional são marcados como integração: projeto de adivinhação, FizzBuzz, análise de coleções, `minigrep`, padrões aplicados e servidor final. Fora disso, conceitos introdutórios aparecem em uma única aula e módulos posteriores avançam para aplicações ou detalhes novos.
