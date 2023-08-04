# Curso de RUST :crab: - Entendendo o básico da linguagem

# Sumário
1. [O que é RUST?](#o-que-é-rust)
2. [Conceitos básicos do Rust](#conceitos-básicos-do-rust)
3. [Declaração de variáveis](#declaração-de-variáveis)
4. [Tipos primitivos](#tipos-primitivos)
5. [Variáveis e constantes](#variáveis-e-constantes)
6. [Escopo](#escopo)
7. [Shadowing](#shadowing)
8. [Funções](#funções)
9. [Condicionais](#condicionais)

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