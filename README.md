# Curso de RUST - Entendendo o básico da linguagem

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
}
```

Mais sobres os tipos primitivos em Rust em:
- [Biblioteca padrão do RUST](https://doc.rust-lang.org/std/index.html)
- [Aprenda por exemplos](https://doc.rust-lang.org/rust-by-example/primitives.html)
  
## Variáveis e constantes
