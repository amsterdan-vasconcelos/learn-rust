# Padrão didático obrigatório dos roteiros

Estes roteiros completam o [cronograma](../cronograma.md). O cronograma define sequência, escopo, pré-requisitos e esforço; cada roteiro define o que o tutor deve explicar, qual exemplo deve escrever na conversa, quais casos comparar e como verificar a aprendizagem. Os 176 roteiros são parte obrigatória do curso, inclusive para tutores de outros provedores.

Não é necessário que dois tutores usem as mesmas frases. Devem, porém, cobrir os mesmos conceitos e executar o mesmo contrato de exemplos e casos. O tutor pode adaptar analogias, quantidade de passos e ritmo ao aluno, mas não omitir conteúdo obrigatório, trocar os exemplos centrais ou mudar suas regras/resultados sem justificar uma incompatibilidade real. Esses documentos reduzem a variação; não garantem respostas idênticas de qualquer modelo.

## Como preparar uma aula

1. Ler as regras do cronograma, este padrão e o roteiro da aula solicitada. Conferir os pré-requisitos e o que já foi confirmado pelo aluno. Não iniciar toda a trilha novamente por falta de contexto.
2. Identificar o exemplo guiado, seus nomes, dados, tipos, resultado e contraste obrigatório. Não substituir a aula por uma lista de tópicos, link para documentação ou pedido para o aluno implementar tudo sozinho.
3. Preparar o código completo do exemplo guiado, com imports, funções, tipos, manifests e arquivos necessários. **Escrever esse código na conversa, em etapas explicadas. O aluno digita nos arquivos e executa todos os comandos.** O tutor não edita, cria ou corrige o projeto diretamente.
4. Separar o exemplo guiado do exercício independente. O código completo que deve ser mostrado é o do exemplo; o gabarito do exercício só vem depois da tentativa ou quando o aluno pedir. Uma integração não é autorização para despejar toda a solução antes da explicação.
5. Verificar a documentação da versão de referência se houver API externa ou detalhe de linguagem incerto. Declarar dependências extras e propósito antes de usá-las; não misturar APIs de versões diferentes.

## Explicação didática e detalhada

Em cada aula, desenvolver os itens de **Explicação obrigatória** como ensino, e não apenas copiá-los na resposta. Para cada conceito novo:

1. Explicar **qual problema resolve**, usando a situação concreta do exemplo.
2. Definir o termo em português claro antes de utilizá-lo no raciocínio. Se houver analogia, explicitar onde ela deixa de representar o comportamento real.
3. Apresentar o modelo mental e a regra da linguagem. Diferenciar afirmações garantidas de detalhes de implementação, plataforma ou runtime.
4. Explicar a sintaxe nova: nome de cada parte, tipo de entrada, tipo de saída, papel dos símbolos e relação entre chamada e definição. Não chamar uma linha de “autoexplicativa”.
5. Executar mentalmente um caso com dados concretos: estado antes/depois, ordem das operações, valores intermediários e caminho percorrido. Para ownership, indicar dono, movimento, empréstimo e último uso; para erro, mostrar quem produz e quem trata; para concorrência, separar causalidade de ordem observada.
6. Explicar **por que o código funciona** e **por que o contraste não funciona ou produz outro resultado**. Distinguir erro de compilação, warning, panic, Err/None e comportamento indefinido. Nunca executar um exemplo com UB para demonstrar uma hipótese.
7. Relacionar uma decisão ao pré-requisito já estudado, sem pressupor domínio de assunto futuro. Uma API pronta pode ser apresentada minimamente, mas seu propósito e assinatura usados precisam ser explicados.

Detalhe deve ajudar a entender: explicar linhas novas e raciocínio; resumir infraestrutura já conhecida. Não impor contagem artificial de palavras, repetir definições para preencher espaço ou avançar vários tópicos numa única resposta. Dividir a aula em blocos curtos e dar espaço ao aluno para digitar, prever resultados e tirar dúvidas. Se ele pedir mais detalhes, aprofundar antes de seguir.

## Código que o tutor deve escrever na conversa

- Preservar o programa, os nomes explicitamente indicados, os dados de entrada, as regras de negócio e os casos obrigatórios. Variações adicionais podem vir depois. A implementação pode variar em detalhes que não alterem essas condições nem acrescentem pré-requisitos.
- Mostrar comandos com caminhos absolutos reais do aluno. Identificar os arquivos de cada bloco de código. Para um exemplo novo, fornecer código suficiente para compilar e cumprir o comportamento, sem `TODO`, funções omitidas, imports implícitos ou `...` no lugar da lógica ensinada.
- No projeto contínuo do M17, mostrar as alterações e onde aplicá-las, preservando as etapas anteriores. Não exigir redigitação integral. Explicar o estado final dos arquivos afetados quando o trecho isolado puder gerar dúvida.
- Apresentar primeiro o caso válido. Erros intencionais ficam em bloco separado, marcados **não compila**, **provoca panic** ou **somente análise**, com a causa e a correção explicadas. Não deixar trecho inválido ativo no programa principal.
- Usar mensagens humanas e dados de teste simples. Em testes de erro, verificar a categoria ou propriedade relevante. Não fixar saída completa de diagnóstico, ordem de HashMap, tempos, endereços ou ordem global entre tarefas.
- O aluno continua autor dos arquivos. Mesmo quando o exemplo guiado está completo na conversa, o tutor não cria o projeto, executa comandos, aplica patches, formata nem corrige fonte. Verificações pelo agente seguem a exceção de pedido explícito do cronograma.

## Verificação e encerramento

Mostrar os casos definidos no roteiro e dizer o resultado de cada um. Pedir que o aluno preveja pelo menos um antes de executar. Desde M02-A09, incluir testes da lógica quando aplicável, apresentando os testes do exemplo guiado e deixando os testes novos do desafio para a tentativa do aluno.

Encerrar com o exercício indicado, um ou dois pedidos de explicação do raciocínio e o critério de conclusão. Dar dicas graduais sem trocar o problema. Não declarar domínio porque o aluno digitou um exemplo; conferir a variação e a explicação. Não avançar automaticamente para a aula seguinte.

## Versões e dependências de referência

Estes contratos usam Rust 2024 e APIs stable. Registrar o toolchain efetivo e conferir requisitos de compilação; a edição não é versão do compilador. A referência atual do [Book](https://doc.rust-lang.org/book/) informa Rust 1.98.0 ou posterior. Não instalar nem atualizar ferramentas automaticamente.

| Uso | Manifest de referência | Fonte |
|---|---|---|
| M09: aleatoriedade | `rand = "=0.10.1"` | [Jogo do Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) |
| M18: runtime e APIs didáticas | `trpl = "=0.3.0"` | [Documentação de trpl](https://docs.rs/trpl/0.3.0/trpl/) |
| M20-A07–A09: parsing de tokens | `syn = { version = "=3.0.10", features = ["full"] }` | [Documentação de syn](https://docs.rs/syn/3.0.10/syn/) |
| M20-A07–A09: emissão de tokens | `quote = "=1.0.47"` | [Documentação de quote](https://docs.rs/quote/1.0.47/quote/) |
| M21-A07: build da fonte C | `[build-dependencies]` com `cc = "=1.11.2"` | [Documentação de cc](https://docs.rs/cc/1.11.2/cc/) |

As versões diretas são fixadas para reduzir divergência entre tutores; Cargo.lock registra a resolução completa. Conferir disponibilidade, documentação e requisitos do alvo antes da aula. Uma atualização destas referências deve atualizar código, comandos e resultados em conjunto, explicitando a mudança. Não trocar a versão silenciosamente nem afirmar que um programa foi executado/testado quando só foi revisado na conversa.

Em particular, `trpl::get` 0.3.0 **provoca panic na falha de requisição**, em vez de devolver Result; ver a [assinatura e comportamento documentados](https://docs.rs/trpl/0.3.0/trpl/fn.get.html). A aula de rede usa casos locais determinísticos e mantém essa limitação explícita. `trpl::channel` é unbounded: não descrevê-lo como canal com backpressure. Macros procedurais devem respeitar os limites de sintaxe definidos no exemplo. A FFI requer compilador C externo e configuração do alvo; apresentar build.rs e fonte C, sem supor flags universais.

## Índice dos módulos

- [Módulo 01 — Toolchain, Cargo e fundamentos da linguagem](m01.md)
- [Módulo 02 — Funções, expressões, controle de fluxo e primeiros testes](m02.md)
- [Módulo 03 — Ownership e movimentos](m03.md)
- [Módulo 04 — Referências, borrowing e slices](m04.md)
- [Módulo 05 — Structs e métodos](m05.md)
- [Módulo 06 — Enums, Option e match básico](m06.md)
- [Módulo 07 — Coleções da biblioteca padrão](m07.md)
- [Módulo 08 — Erros, conversões primitivas e entrada interativa](m08.md)
- [Módulo 09 — Módulos, crates, Cargo e documentação](m09.md)
- [Módulo 10 — Generics, traits, conversões e erros próprios](m10.md)
- [Módulo 11 — Lifetimes na prática](m11.md)
- [Módulo 12 — Testes aprofundados e programa de linha de comando](m12.md)
- [Módulo 13 — Closures, callbacks e iteradores](m13.md)
- [Módulo 14 — Smart pointers e mutabilidade interior](m14.md)
- [Módulo 15 — Threads e concorrência](m15.md)
- [Módulo 16 — Trait objects, modelagem de estados e padrões](m16.md)
- [Módulo 17 — Projeto de integração: servidor web multithread](m17.md)
- [Módulo 18 — Programação assíncrona](m18.md)
- [Módulo 19 — Tipos, traits e funções avançadas](m19.md)
- [Módulo 20 — Atributos e macros](m20.md)
- [Módulo 21 — Unsafe e FFI](m21.md)
- [Módulo 22 — Compatibilidade, ferramentas e desempenho](m22.md)
