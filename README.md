# Curso de RUST :crab: - Entendendo o básico da linguagem

# Sumário
1. [O que é RUST?](#o-que-é-rust)
2. [Conceitos básicos do Rust](#conceitos-básicos-do-rust)
3. [Declaração de variáveis](#declaração-de-variáveis)
4. [Tipos primitivos](#tipos-primitivos)
5. [Variáveis e constantes](#variáveis-e-constantes)

# O que é RUST?
Rust é uma linguagem de programação com o foco em:
- **Performance (Desempenho)**: O Rust é incrivelmente rápido e eficiente em termos de memória, pois não possui "*runtime*", ou seja, não executa nada em tempo de execução, impedindo a criação e a execução de tarefas assíncronas; e não possui *garbage colletor* (coletor de lixo), logo o próprio desenvolvedor fica resposável pel gerenciamento de memória. Rust pode alimentar serviços de desempenho crítico, executar em dispositivos embarcados e integra-se facilmente com outras linguagens de programação.
- **Confiabilidade**: Rust garante a segurança de memória e segurança de threads, permitindo a eliminação de classes *bugadas* (com defeito) em tempo de execução.
- **Produtividade**: A linguagem Rust possui uma ótima documentação, um compilador amigável com mensagens de erro úteis e ferramentas de alto nível. Possui um gerenciador de pacotes integrado e uma ferramenta de compilação, suporte inteligente a vários editores com auto-completar e inspeções de tipo, uma auto-formatador, etc.

A linguagem Rust é mais usada para desenvolver aplicações de linha de comando (CLI), aplicações web e aplicações para serem executadas em sistemas embarcados.

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

## Tipos primitivos
No Rust, quando declaramos uma variável, podemos informar qual o tipo da variável. Fazemos isso após o nome da variável, seguido por dois pontos `:`. Note que, informar o tipo da variável em Rust é opicional! Caso não informe, o Rust irá fazer isso automáticamente, assumindo um padrão. Como, por exemplo, se declararmos uma variável com o valor 300 e não informarmos o tipo, por padrão o Rust irá assumir que o tipo é `i32`.

Exemplo:
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

Mais sobres os tipos primitivos em Rust em:
- [Biblioteca padrão do RUST](https://doc.rust-lang.org/std/index.html)
- [Aprenda por exemplos](https://doc.rust-lang.org/rust-by-example/primitives.html)
  
## Variáveis e constantes
No Rust uma variável é imutável por padrão, para modificar esse comportamento temos que informar ao Rust de que se trata de uma variável mutável por meio da palavra `mut` antes do nome da variável e depois de `let`.

Exemplo:
```rust
fn main() {
    let mut variavel = false;
    variavel = false;
}
```

A ***diferença entre variável imutável e constante no Rust*** é que, no caso das variáveis, mesmo que sejam imutáveis, possuem um endereço de memória, alocado pelo compilador, onde está o valor da variável e, em tempo de execução, o valor é recuperado da pilha de memória para ser usado pelo programa. No caso das constantes não há esta alocação de memória, pois em tempo de compilação, o compilador substituirá todas as constantes pelos seus respectivos valores informados na momento da declaração da constante.

Exemplo de declaração de uma constante:
```rust
fn main() {
    const PI:f32 = 3.14;
    println!("PI = {}", PI);
}
```

Muito similar a uma constante, temos as **variáveis globais** que são declaradas usando a palavra `static`. E, assim como as constantes, <ins>precisam ter os tipos e os valores informados no momento da declaração</ins> e não aceitam valores variáveis, ou seja, <ins>não é permitido associar a uma variável global ou a uma constante um valor que necessite executar uma função ou acessar um API para existir</ins>, o valor precisa ser constante.

Exemplo:
```rust
static GLOBAL:f32 = 1.22;
fn main() {
    println!("GLOBAL = {}", GLOBAL);
}
```

É possível deixar a **variável global como mutável**, porém o Rust não permitirá a execução do código pois dirá que o código é inseguro. Para contornar isso é possível usar a função ou bloco `unsafe`. Neste caso o desenvolvedor entende e assumi os riscos de se usar uma variável global mutável.

Exemplo:
```rust
static mut GLOBAL:f32 = 1.22;
fn main() {
    unsafe {
        println!("GLOBAL insegura = {}", GLOBAL);
    }
}
```

**FONTES:**
- [Sobre constantes](https://doc.rust-lang.org/std/keyword.const.html)
- [Constants](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html?highlight=const#constants)
- [Sobre variáveis estáticas](https://doc.rust-lang.org/std/keyword.static.html)
- [Static](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html?highlight=static#static)
- [Sobre unsafe](https://doc.rust-lang.org/std/keyword.unsafe.html)
- [Unsafe operations](https://doc.rust-lang.org/rust-by-example/unsafe.html)

**Para saber um pouco mais sobre operações inseguras no Rust:**
- [Rust Unsafe — uma introdução à parte da linguagem que nos dá super poderes - Franklyn Sancho](https://franklyn-sanc.medium.com/rust-unsafe-uma-introdu%C3%A7%C3%A3o-%C3%A0-parte-da-linguagem-que-nos-d%C3%A1-super-poderes-24f54bf8cca2)