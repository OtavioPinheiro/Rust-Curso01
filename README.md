# Curso de RUST :crab: - Entendendo o básico da linguagem

# Sumário
1. [O que é RUST?](#o-que-é-rust)
2. [Ecossistema do Rust](#ecossistema-do-rust-crab)
3. [Comandos básicos dos Cargo](#comandos-básicos-do-cargo)
4. [Conceitos básicos do Rust](#conceitos-básicos-do-rust)
5. [Declaração de variáveis](#declaração-de-variáveis)
6. [Tipos primitivos](#tipos-primitivos)
7. [Variáveis e constantes](#variáveis-e-constantes)
8. [Escopo](#escopo)
9. [Shadowing](#shadowing)
10. [Funções](#funções)
11. [Condicionais](#condicionais)
12. [Loops - Estruturas de repetição](#loops-estruturas-de-repetição)
13. [*Ownership*](#ownership)
14. [*Borrowing*](#borrowing)
15. [Tratamento de erros](#tratamento-de-erros)
16. [Arrays](#arrays)
17. [Enums](#enums)
18. [Option](#option)
19. [Generics](#generics)
20. [Vectors](#vectors)


# O que é RUST?
Rust é uma linguagem de programação com o foco em:
- **Performance (Desempenho)**: O Rust é incrivelmente rápido e eficiente em termos de memória, pois não possui "*runtime*", ou seja, não executa nada em tempo de execução, impedindo a criação e a execução de tarefas assíncronas; e não possui *garbage colletor* (coletor de lixo), logo o próprio desenvolvedor fica resposável pel gerenciamento de memória. Rust pode alimentar serviços de desempenho crítico, executar em dispositivos embarcados e integra-se facilmente com outras linguagens de programação.
- **Confiabilidade**: Rust garante a segurança de memória e segurança de threads, permitindo a eliminação de classes *bugadas* (com defeito) em tempo de execução.
- **Produtividade**: A linguagem Rust possui uma ótima documentação, um compilador amigável com mensagens de erro úteis e ferramentas de alto nível. Possui um gerenciador de pacotes integrado e uma ferramenta de compilação, suporte inteligente a vários editores com auto-completar e inspeções de tipo, uma auto-formatador, etc.

A linguagem Rust é mais usada para desenvolver aplicações de linha de comando (CLI), aplicações web e aplicações para serem executadas em sistemas embarcados.

[Sumário](#sumário)

**FONTES**: 
- [RUST](https://www.rust-lang.org/)
- [What is Runtime in Rust?](https://stackoverflow.com/questions/68188420/what-is-runtime-in-rust)
- [Comprehensive Rust](https://google.github.io/comprehensive-rust/pt-BR/index.html)

# Ecossistema do Rust :crab:
Em Rust, assim como em outra linguagens, podemos escrever o nosso código em um bloco notas qualquer e depois compilar o arquivo, fazemos isso usando o comando `rustc <nome-do-arquivo.rs>`. Porém usar este método para um projeto grande não é viável e nem prático, então para organizar a estrutura do nosso projeto, gerenciar as dependências o Rust conta com o ***Cargo*** (`cargo`).

O ***Cargo*** quando usado para começar um projeto do zero irá criar toda a estrutura padrão do Rust automaticamente que consiste em:
- Criar a pasta `src`
- Criar dentro da pasta `src` o arquivo `main.rs`
- Criar o arquivo `Cargo.toml` na raiz do projeto. Este arquivo contém informações como o nome do projeto, versão e edição do cargo que está sendo utilizada e as dependências do projeto.

```
+--project
|   +--src
|       +--main.rs
+--Cargo.toml
```

[Sumário](#sumário)

## Comandos básicos do Cargo
| Comando | Funcionalidade |
|---------|----------------|
| `cargo --help` | Exibe uma lista dos comandos disponíveis e a funcionalidade. |
| `cargo new <nome-do-projeto>` | Cria um projeto (pacote `cargo`) Rust do zero, criando também a estrutura padrão de projeto. |
| `cargo init` | Cria um novo pacote `cargo` dentro de um diretório existente. |
| `cargo build` | Compila o pacote (projeto) atual. |
| `cargo run` | Executa um binário. Também pode ser usado para buildar e executar o programa Rust. |
| `cargo test` | Executa os testes. |
| `cargo update` | Atualiza a lista de dependências dentro do arquivo `Cargo.lock`. |
| `cargo install <nome-do-binário>` | Instala um binário Rust na pasta padrão (`$HOME/.cargo/bin`). |
| `cargo uninstall <nome-do-binário>` | Desinstala um binário Rust. |

[Sumário](#sumário)

# Conceitos básicos do Rust
## Declaração de variáveis
No Rust antes de declarar uma variável usamos a palavra reservada `let`.

Exemplo:
```rust
fn main() {
    let variavel:u8 = 128;
    println!("variável = {}", variavel);
}
```
[Sumário](#sumário)

## Tipos primitivos
No Rust, quando declaramos uma variável, podemos informar qual o tipo da variável. Fazemos isso após o nome da variável, seguido por dois pontos `:`. Note que, informar o tipo da variável em Rust é opicional! Caso não informe, o Rust irá fazer isso automáticamente, assumindo um padrão. Como, por exemplo, se declararmos uma variável com o valor 300 e não informarmos o tipo, por padrão o Rust irá assumir que o tipo é `i32`.

**Exemplo:**
```rust
fn main() {
    let variavel = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let booleana:bool = false;
    println!("Tamanho booleana = {}", std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}
```
[Sumário](#sumário)

**Mais sobres os tipos primitivos em Rust em:**
- [Biblioteca padrão do RUST](https://doc.rust-lang.org/std/index.html)
- [Aprenda por exemplos](https://doc.rust-lang.org/rust-by-example/primitives.html)
  
## Variáveis e constantes
No Rust uma variável é imutável por padrão, para modificar esse comportamento temos que informar ao Rust de que se trata de uma variável mutável por meio da palavra `mut` antes do nome da variável e depois de `let`.

**Exemplo:**
```rust
fn main() {
    let mut variavel = false;
    variavel = false;
}
```

A ***diferença entre variável imutável e constante no Rust*** é que, no caso das variáveis, mesmo que sejam imutáveis, possuem um endereço de memória, alocado pelo compilador, onde está o valor da variável e, em tempo de execução, o valor é recuperado da pilha de memória para ser usado pelo programa. No caso das constantes não há esta alocação de memória, pois em tempo de compilação, o compilador substituirá todas as constantes pelos seus respectivos valores informados na momento da declaração da constante.

**Exemplo de declaração de uma constante:**
```rust
fn main() {
    const PI:f32 = 3.14;
    println!("PI = {}", PI);
}
```

Muito similar a uma constante, temos as **variáveis globais** que são declaradas usando a palavra `static`. E, assim como as constantes, <ins>precisam ter os tipos e os valores informados no momento da declaração</ins> e não aceitam valores variáveis, ou seja, <ins>não é permitido associar a uma variável global ou a uma constante um valor que necessite executar uma função ou acessar um API para existir</ins>, o valor precisa ser constante.

**Exemplo:**
```rust
static GLOBAL:f32 = 1.22;
fn main() {
    println!("GLOBAL = {}", GLOBAL);
}
```

É possível deixar a **variável global como mutável**, porém o Rust não permitirá a execução do código pois dirá que o código é inseguro. Para contornar isso é possível usar a função ou bloco `unsafe`. Neste caso o desenvolvedor entende e assumi os riscos de se usar uma variável global mutável.

**Exemplo:**
```rust
static mut GLOBAL:f32 = 1.22;
fn main() {
    unsafe {
        println!("GLOBAL insegura = {}", GLOBAL);
    }
}
```
[Sumário](#sumário)

**FONTES:**
- [Sobre constantes](https://doc.rust-lang.org/std/keyword.const.html)
- [Constants](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html?highlight=const#constants)
- [Sobre variáveis estáticas](https://doc.rust-lang.org/std/keyword.static.html)
- [Static](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html?highlight=static#static)
- [Sobre unsafe](https://doc.rust-lang.org/std/keyword.unsafe.html)
- [Unsafe operations](https://doc.rust-lang.org/rust-by-example/unsafe.html)

**Para saber um pouco mais sobre operações inseguras no Rust:**
- [Rust Unsafe — uma introdução à parte da linguagem que nos dá super poderes - Franklyn Sancho](https://franklyn-sanc.medium.com/rust-unsafe-uma-introdu%C3%A7%C3%A3o-%C3%A0-parte-da-linguagem-que-nos-d%C3%A1-super-poderes-24f54bf8cca2)

## Escopo
Em Rust, assim como em outras linguagens de programação como C, C++, C#, Java, etc, um escopo é definido por um bloco `{}` e **as variáveis definidas dentro do escopo só estarão acessíveis àquele escopo**. É possível criar escopo vazios e anônimos.

**Exemplo:**
```rust
fn main() {
    let a = 3;

    {
        let b = 5;
    }
}
```
[Sumário](#sumário)

### Shadowing
Diferente de outras linguagens de programação mais famosas, o Rust permite a redeclaração de variáveis, ou seja, o código abaixo em Rust é válido e gera um *warning*, mas é compilado sem problemas.

Redeclarando variável:
```rust
let variavel:i8 = 10;
let variavel:i8 = 12;
println!("Variável = {}", variavel);
```

O conceito de *shadowing* é parecido com a redeclaração mas se aplicando a um escopo menor no código e também pode envolver mudança do tipo da variável, ou seja, com a funcionalidade *shadowing* é possível declarar uma variável com o mesmo nome em um escopo menor e, ainda, é possível mudar o tipo da variáve.

**Exemplo:**
```rust
fn sombra() {
    let a = 123;
    println!("[fora] a = {}", a);

    {
        let b = 22;
        println!("[dentro] b = {}", b);

        let a = "texto";
        println!("[dentro] a = {}", a);
    }

    println!("[fora] a = {}", a);
}
```
[Sumário](#sumário)

**FONTES:**
- [Scope and Shadowing](https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html)
- [Shadowing and Temporary Mutability in Rust - Thorsten Hans](https://www.thorsten-hans.com/shadowing-temporary-mutability-rust/)

## Funções
Para declarar uma função em Rust usa-se a palavra reservada `fn`, seguido do nome da função, dos parâmetros detro dos parêntesis `()` e, se houver, o retorno da função, definido pelo uso de uma seta `->`. No Rust, por padrão, se não for especificado o retorno da função, então a função não retorna nada. 

Vale lembrar que, caso a função necessite de parâmetros e/ou retorne algo, é necessário informar o tipo dos parâmetros esperados pela função e o tipo do retorno.

**Exemplo:**
```rust
fn soma(a:i32, b:i32) -> i32 {
    println!("O resultado da soma de {} + {} = {}", a, b, a + b);
    a + b
}
```

**Observação:** Em Rust tudo é uma expressão, então caso desejarmos utilizar o resultado da função definida acima em outra parte do código, temos que retornar o valor da soma e para isso podemos não colocar o `;` na última instrução da função, desta o Rust irá interpretar isso como sendo o retorno ou podemos utilizar o `return`, que é o padrão para a maioria das linguagens de programação.

Exemplo de utilização da função soma como expressão:
```rust
fn main() {
    println!("Valor da soma é: {}", soma(a, b));
}
```
[Sumário](#sumário)

## Condicionais
Em Rust, assim como na maioria das outras linguagens, a declaração `if` é a mais conhecida quando o assunto é condicionais. Mas, vale ressaltar que, em Rust, também temos o `match` que é similar com o `switch-case` de linguagens como Java, C, C++, C#, etc.

Uma peculiaridade do Rust em relação ao `if` é que, como tudo é uma expressão em Rust, podemos atribuir o resultado do `if` a uma variável.

**Exemplo 01:**
```rust
fn main() {
    let responsavel_autorizou = false;
    let idade:i8 = 17;

    if idade >= 18 {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com a assinatura do responsável");
    } else {
        println!("Não pode entrar");
    }
}
```

**Exemplo 02:**
```rust
fn main() {
    let idade:i8 = 20;
    let eh_maior = idade >= 18;

    let condicao = if eh_maior { "maior" } else { "menor" };
    println!("É {} de idade", condicao);
}
```

Quando estiver usando o `match` é importante sempre usar a expressão coringa `_` para designar todos os outros cenários não cobertos. Caso algum cenário possível não seja coberto, o código não será compilado e o compilador do Rust irá informar para usar o `_` no `match` para cobrir todos os casos possíveis. Vale lembrar, caso a instrução do `match` possua apenas uma linha, ao final deve ser usado a vírgula `,`, mesmo na última instrução. Caso seja instruções de múltiplas linhas, a vírgula é opicional, mas é necessário utilizar as chaves `{}`

**Exemplo:**
```rust
let idade:i8 = 17;
let eh_maior = idade >= 18;
let condicao = if eh_maior { "maior" } else { "menor" };

match condicao {
    "maior" => { println!("Pode entrar na balada") }
    _ => println!("Não pode entrar na balada"),
}
```

**Exemplo de match com if**
```rust
fn main() {
    let valor = 10;
    for i in 0..=valor {
        println!("{}: {}", i, match i {
                1..=4 => "É pouco",
                _ if i % 2 == 0 => "É par",
                _ => "É impar"
            }
        );
    }
}
```

[Sumário](#sumário)

## Loops (Estruturas de repetição)
Estruturas de repetição ou laços (*loops*) são usadas na linguagem de programação para fazer com que o *software* execute uma determinada ação ou tarefa repetidas vezes. Os laços mais famosos e conhecidos são `for` e `while`. Em Rust, também temos o `loop` que é uma estrutura de repetição que é executada indefinidamente até encontrar uma condição de parada e a palavra reservada `break`. No Rust, o `for` possui algumas peculiaridades, se assemelhando com o `for each` de algumas linguagens. Uma dessas semelhanças é que o `for` no Rust percorre intervalos (ou itaradores). Um exemplo de intervalos seria `1..11` que significa um intervalo de 1 a 10, pois o 11 não está incluído no intervalo, ou, da mesma forma, podemos escrever este mesmo intervalo da seguinte maneira `1..=10`.

Código de exemplo dos laços de repetição citados.

**Exemplo 01:**
```rust
fn main() {
    let numero:i8 = 7;
    let mut multiplicador:i8 = 1;
    
    while multiplicador <= 10 {
        println!("{} x {} = {}", numero, multiplicador, numero * multiplicador);
        multiplicador += 1;
    } 
}
```

**Exemplo 02:**
```rust
fn main() {
    let numero:i8 = 6;
    let mut multiplicador:i8 = 1;
    
    loop {
        println!("{} x {} = {}", numero, multiplicador, numero * multiplicador);
        multiplicador += 1;
        
        if multiplicador == 11 {
            break;
        }
    } 
}
```

**Exemplo 03:**
```rust
fn main() {
    let numero:i8 = 8;
    
    for multiplicador in 1..=10 {
        println!("{} x {} = {}", numero, multiplicador, numero * multiplicador);
    } 
}
```

É importante lembrar que em Rust, assim como em Python por exemplo, temos o `continue` que serve para pular para a próxima iteração. Então, caso queiramos exibir apenas os números pares de 1 a 10, podemos utilizar o `continue` para pular para próxima iteração toda vez que a condição de um número par for satisfeita.

**Exemplo 04:**
```rust
fn main() {
    for numero in 1..=10 {
        if numero % 2 != 0 {
            continue
        }
        println!("{} é par", numero);
    } 
}
```
[Sumário](#sumário)

# Ownership
Para o melhor entendimento dessa seção é recomendável o conhecimento básico sobre alocação de memória (*Stack* e *Heap*). 
Para saber mais sobre *Stack* e *Heap*:
- [Heap vs Stack - Gustavo Pantuza](https://blog.pantuza.com/artigos/heap-vs-stack)
- [Gerenciamento de memória - Stack vs Heap | Dias de Dev - Dias de Dev por Vinicius Dias](https://www.youtube.com/watch?v=7kJwVQGJCbw)

*Ownership* em Rust é um tipo de gerenciamento de memória. Temos o gerenciamento de memória manual, onde o desenvolvedor é responsável por alocar e desalocar espaço da memória, ou seja, todo o processo de gerenciamento de memória fica por conta do desenvolver. Temos o *garbage collector*, ou (na tradução livre) coletor de lixo, que é um processo executado por de baixo dos panos (em *background*), responsável por desalocar (liberar) a memória da *heap*. Temos, ainda, uma terceira forma de gerenciamento de memória que é usada pelo Rust que é a *ownership*, que, basicamente, significa que cada dado, valor ou conjuto de dados ou valores armazenados na *heap* só poderão ter um único dono.

Contextualizando, quando passamos, por exemplo, uma *string* por parâmetro para uma função em Rust, sendo que esta *string* pertencia a um escopo, temos o *move semantics*, ou seja, a variável que continha a *string*, sendo ela um ponteiro, passa a ter um valor inválido dentro do escopo que foi declarada, porque ela trocou de "dono" no momento em foi passado como parâmetro para a função.

**Exemplo:**
```rust
fn ownership() {
    let uma_string = String::from("Teste"); //uso o método String::from() para que a string seja armazenada na heap
                                    // porque trata-se de uma string que pode variar o seu tamanho
    rouba(uma_string);

    println!("{}", uma_string);
}

fn rouba(string: String) {
    println!("{}", string);
}
```

No código acima temos a variável `uma_string` que é do tipo `String` e é alocada na *heap*. Quando passamos essa variável como parâmetro para a função `rouba`, a função torna-se o novo dono do endereço de memória que representa o valor da variável `uma_string`, ou seja, local na memória onde está armazenado o valor desta string. Isso significa que agora quem pode manipular e ter acesso ao valor dessa variável é a função `rouba`. Quando a função termina de ser executada, ela sai de escopo e as variáveis, os parâmetros e todos os valores dentro deste escopo tem seu endereço de memória liberado, logo, como a função era a dona do endereço de memória referente ao valor da variável `uma_string`, quando ela termina de ser executada, a variável `uma_string` passa a ter o valor inválido. Esse é o comportamento esperado para o gerenciamento de memória por *ownership*.

Para saber mais sobre *ownership*:
- [Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

[Sumário](#sumário)

## Borrowing
Para resolver o problema anterior, ou seja, passar uma variável, que é um ponteiro (possui seu valor armazenado na *heap*), para uma função e continuar a usar a variável depois que a função for encerrada, podemos fazer com que a função retorne uma variável do mesmo tipo e esse retorno será usado no lugar da antiga variável. Ou podemos passar uma referência para a função ao invés de passar a variável em si, isso é chamado de *borrowing* em Rust. Em outras palavras, iremos emprestar o valor para a função executar a tarefa, desta forma o *ownership* não é perdido para a função, logo a variável não terá um valor inválido depois que a função for encerrada.

**Exemplo retornando um valor da função**
```rust
fn main() {
    let uma_string = String::from("teste"); //declarando uma string com tamanho indefinido

    let outra_string = rouba(uma_string);

    println!("{}", outra_string);
}

fn rouba(string: String) {
    println!("{}", string);

    string;
}
```

**Exemplo de *borrowing*:**
```rust
fn main() {
    let uma_string = String::from("teste");

    rouba(&uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &String) {
    println!("{}", string);
}
```

Caso queiramos modificar a referência passada para a função, precisamos tornar a variável e a referência mutáveis adicionando a palavra `mut`.

**Exemplo de *borrowing* com uma referência mutável:**
```rust
fn main() {
    let mut uma_string = String::from("teste");

    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" testando");
    println!("{}", string);
}
```

**Para saber mais sobre *borrowing*:**
- [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

[Sumário](#sumário)

# Tratamento de erros
No Rust, basicamente, erros são irrecuperáveis, não há uma estrutura como `try`-`catch`. Porém podemos lançar o erro/exceção.

Para lançar um erro ou uma exceção em Rust usamos a macro `panic!` e passamos uma mensagem, parecido com `throw new Exception("Mensagem de erro personalizada")` em Java ou em C#. É possível também definir a variável de ambiente `RUST_BACKTRACE=1` para habilitar o *backtrace* e ter uma visão mais detalhada sobre onde o erro ocorreu, ou ainda definir `RUST_BACKTRACE=full` para mostrar o *backtrace* completo.

**Exemplo:**
```rust
fn erros() {
    panic!("Erro!");
}
```

Outra maneira é usar o `Result` para recuperar o erro. Neste `Result` é retornado uma `String` em caso de sucesso e um número inteiro em caso de erro.

**Exemplo:**
```rust
fn erros() {
    match resultado() {
        Ok(s) => println!("Sucesso: {}", s),
        Err(numero) => println!("Código de erro: {}", numero)
    };
}

fn resultado() -> Result<String, u8> {
    Ok(String::from("Tudo certo"))
}
```

Em resumo, o Rust não possui `Exceptions`, ao invés disso possui estas duas formas de tratamento de erros, sendo a primeira, com o uso do `panic!`, a forma irrecuperável e a segunda, a forma recuperável com o `Result`. A forma de erro irrecuperável é quando, por exemplo, tenta-se acessar um índice inexistente de um *array* (ou vetor), acessar o índice 4 de um *array* de tamanho 4, como a contagem começa do 0 (zero), o último elemento terá o índice 3. Esta forma de erro (irrecuperável) são sempre sintomas de um *bug* (defeito) e quando isso ocorre queremos parar a execução do programa imediatamente, logo usamos o `panic!` que interrompe a execução. Já a segunda forma de erro, a recuperável, é quando, por exemplo, tenta-se acessar um arquivo e este arquivo não existe, retornando o erro `file not found`, quando isto ocorre queremos reportar o erro ao usuário e tentar a operação novamente.

Por via de regra quando o compilador possui mais informações sobre o erro é aconselhavél trabalhar com a forma irrecuperável (`panic!`), caso contrário utilizar o `Result<String, u8>`, assim o desenvolvedor ficará responsável por dar mais informações sobre o erro ocorrido.

**Para saber mais:**
- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)
- [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors)
- [To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)

[Sumário](#sumário)

# Arrays
Em Rust *arrays* são de tamanho definido, ou seja, não é possível declarar *arrays* dinâmicos, os *arrays* precisam ter o tamanho definido durante o tempo de compilação. Em outras palavras, isso `let algum_array = []` ou isso `let algum_array: [&str, tamanho] = []` não são permitidos. Caso seja necessário criar uma estrutura de dados parecido com *arrays* e que seja dinâmica (que não seja necessário informar o tamanho) é recomendável o uso de vetores.

A forma de se trabalhar com *arrays* em Rust segue o padrão de muitas liguagens de programação conhecidas, como o acesso aos elementos do *array* informando o índice dentro dos colchetes ([]). A maneira de se declarar um *array* é simples, basta informar os elementos dentro dos colchetes ou informar o tipo e o tamanho do *array* ou ambos. Então, temos:

```rust
fn main() {
    let primeiro_array = [1, 2, 3, 4, 5];
    let segundo_array: [u8; 4];
    let terceiro_array: [&str; 3] = ["test", "dev", "hml"];
}
```

vale ressaltar que ao informar o tipo e o tamanho do *array*, essas informações devem estar envolvidas pelos colchetese, separadas pelo ponto e vírgula `;` e precedidos por dois pontos `:`, assim como ilustra o exemplo anterior.

Como os *arrays* possuem iteradores, então é possível utilizar o `for` para percorrer cada um dos elementos, da mesma forma como é feito com o `for each` em outras linguagens.

**Exemplo:**
```rust
fn main() {
    let notas: [f32; 4] = [2.3, 4.5, 10.0, 9.5];

    for nota in notas {
        println!("As notas são:");
        println!("\t{}", nota);
    }
}
```

Para acessar o tamanho de *array* contamos com a função `len()` e, se o *array* possui apenas valores repetidos, é possível declarar o *array* de uma forma mais enxuta, informando o valor que se repete e o número de vezes que esse valor se repete.

```rust
fn main() {
    let notas_boas: [f32; 4] = [10.0; 4];

    for indice in 0..notas.len() {
        println!("A nota {} é {}", indice + 1, notas[indice]);
    }
}
```

Também é possível ter *arrays* de *arrays*, ou seja, matrizes.

**Exemplo:**
```rust
fn matriz() {
    let matriz: [[f32; 3]; 2] = [
        [3.3, 1.2, 0.1],
        [10.0, 9.0, 8.5]
    ];

    for linha in 0..matriz.len() {
        for coluna in 0..matriz[linha].len() {
            println!("O item da linha {} da coluna {} é {}", linha + 1, coluna + 1, matriz[linha][coluna]);
        }
    }
}
```

Caso queiramos declarar uma variável para acessar o índice de um *array*, essa variável precisará ser do tipo `usize`. O tipo `usize` significa que a variável ocupa o tamanho necessário para um endereço de memória, independente se o sistema for de 32 bits ou de 64 bits. Ou seja, essa variável em um sistema de 32 bits terá o tamanho de 4 bytes e se for em um sistema de 64 bits terá o tamanho de 8 bytes.

**Para mais informações sobre *arrays* em Rust :crab: :**
- [Primitive Type array](https://doc.rust-lang.org/std/primitive.array.html)

[Sumário](#sumário)

# Enums
**Enums**, ou "enumerações", servem para definir conjuntos de valores discretos e relacionados entre si. Imagine um menu de opções em um restaurante: cada prato do menu é um valor distinto dentro do `enum` "Prato". **Enums** garantem que apenas valores válidos sejam utilizados, evitando erros de digitação ou inconsistências no código. Para utilizar um `enum`, basta declarar uma variável do tipo `enum` e atribuir um de seus valores:

**Exemplos:**

```Rust
enum Prato {
    Feijoada,
    Strogonoff,
    Pizza,
}

let pedido: Prato = Prato::Feijoada;
```

```Rust
enum Cor {
    Vermelho,
    Verde,
    Azul,
}
```

[Sumário](#sumário)

# Option
O tipo `Option` em **Rust** é um `enum` que tem duas variantes: `Some` e `None`. Ele é usado para expressar a possibilidade de ausência de um valor. `Option` garante que você não terá problemas com `null pointers`, prevenindo _crashes_ e erros inesperados.

**Exemplos:**

```Rust
let resultado: Option<i32> = Some(42); // Contém o valor 42
let resultado_vazio: Option<i32> = None; // Não contém nenhum valor
```

```Rust
let nome: Option<String> = Some("João".to_string()); // Valor presente
let idade: Option<i32> = None; // Valor ausente
```

Para acessar o valor de um `Option`, utilize o método `unwrap` (se você tem certeza que o valor existe) ou `unwrap_or_else` (para definir um valor padrão caso o valor esteja ausente):

```Rust
let nome_real = nome.unwrap(); // Obter valor presente
let idade_padrao = idade.unwrap_or_else(|| 0); // Definir valor padrão
```

[Sumário](#sumário)

# Generics
Os Generics permitem escrever código em Rust que é genérico em relação ao tipo de dado que está sendo usado. Eles permitem que você escreva funções e estruturas de dados que funcionam com qualquer tipo de dado. Imagine uma função que manipula listas de elementos, mas você não quer escrever a mesma função para cada tipo de elemento (números, strings, etc.). Generics permitem que você defina uma única função que funcione com qualquer tipo de dado. Para definir um `generics` use a sintaxe `<T>`, onde `<T>` representa um tipo genérico, permitindo que uma função funcione com qualquer tipo de dado. Para usar a função, basta especificar o tipo ao chamar a função.

```Rust
fn meu_print<T>(lista: &Vec<T>) {
    for item in lista {
        println!("{}", item);
    }
}
```

```Rust
let lista_numeros = vec![1, 2, 3];
meu_print(&lista_numeros);

let lista_strings = vec!["Olá", "Mundo"];
meu_print(&lista_strings);

```

[Sumário](#sumário)

# Vectors
Vetores são uma coleção de valores do mesmo tipo, que podem crescer ou diminuir dinamicamente. Imagine uma lista de compras que pode ser modificada à medida que você vai adicionando itens. Vectors permitem essa flexibilidade e eficiência no armazenamento de dados.

Em **Rust**, eles são representados pelo tipo `Vec<T>`. Você pode adicionar elementos a um vetor, acessar elementos individuais pelo índice, etc.

**Exemplos:**

```Rust
let mut numeros: Vec<i32> = Vec::new(); // Cria um vetor vazio de números inteiros
numeros.push(1); // Adiciona o número 1 ao vetor
numeros.push(2); // Adiciona o número 2 ao vetor

println!("O primeiro número é: {}", numeros[0]); // Imprime o primeiro número do vetor
```

```Rust
let mut compras: Vec<String> = vec![];
compras.push("Leite");
compras.push("Pão");
compras.push("Frutas");

println!("{:?}", compras); // Exibe a lista de compras
```

[Sumário](#sumário)
