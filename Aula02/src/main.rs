fn main() {
    let notas: [f32; 4] = [7.0, 8.0, 9.5, 10.0];
    let inteiro: usize = 0;

    // println!("Notas:\nNota 1: {}\nNota 2: {}\nNota 3: {}\nNota 4: {}", notas[0], notas[1], notas[2], notas[3]);

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }

    matriz();
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    cores();
    conteudo_opcional();
    vectors();
    conta_corrente();
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool
{
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

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

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, yellow: u8, magenta: u8, black: u8}
}

fn cores()
{
    let cor: Color = Color::RgbColor(12, 5, 32);

    println!("Cor = {}",
        match cor {
            Color:: Red => "vermelho",
            Color:: Green => "verde",
            Color:: Blue => "azul",
            Color::RgbColor(0, 0, 0) | Color::CymkColor { cyan: _, yellow: _, magenta: _, black: 255 } => "preta",
            Color::RgbColor(_, green, _) => {
                println!("{}", green);
                "RGB desconhecido"
            }
            Color::CymkColor { cyan: _, yellow: _, magenta: _, black: _ } => "CYMK desconhecido"
        }
    );
}

fn conteudo_opcional()
{
    let conteudo_arquivo = ler_arquivo(String::from(""));
    // Rust não consegue exibir Optional, usamos o match ou o debug
    // O & passa a referência para o match, logo não terá o Ownership
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };

    // {:?} feature de debug
    println!("{:?}", conteudo_arquivo);

    // Forma simplificada de comparação - Pattern matching
    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza que há valor {}", valor);
    }
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String>
{
    Some(String::from("Conteúdo do arquivo"))
}

fn vectors()
{
    // Declaração de um vetor
    // let mut notas: Vec<f32> = Vec::new();
    // notas.push(10.0);
    // notas.push(8.8);
    // notas.push(6.5);

    // Declaração + Atribuição
    // let mut notas: Vec<f32> = vec![10.0, 8.8, 6.5];

    // Segunda forma de declaração de vetores informando a capacidade
    let mut notas: Vec<f32> = Vec::with_capacity(4);
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);
    println!("Capacidade = {}", notas.capacity());

    println!("{:?}", notas);

    notas.push(6.8);
    println!("Capacidade = {}", notas.capacity());
    println!("{:?}", notas);

    // Acessando valores
    println!("Nota 1 = {}", notas[0]);

    // Resolvendo problemas para índices inexistentes
    println!("Nota 6 = {}", match notas.get(7) {
        // Para resolver o problema de incompatibilidade devemos informar que trata-se de uma referência ou podemos desreferencia-la
        // Some(&n) => n,
        Some(n) => *n,
        None => 0.0
    });

    // Percorrendo um vetor

    // if let Some(nota) = notas.pop() {
    //     println!("Último valor = {}", nota);
    //     println!("{:?}", notas);
    // }

    // while let Some(nota) = notas.pop() {
    //     println!("Nota removida = {}", nota);
    // }
    // println!("{:?}", notas);

    for nota in &notas {
        println!("Nota = {}", nota);
    }
    println!("{:?}", notas);
}

struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String
}

fn conta_corrente() {
    let titular = Titular{nome: String::from("Teste"), sobrenome: String::from("Aluno")};
    let mut conta: Conta = Conta {
        titular,
        saldo: 100.0
    };

    conta.sacar(50.0);

    println!("Dados da conta: Titular = {} {}, Saldo = {}", 
    conta.titular.nome, 
    conta.titular.sobrenome, 
    conta.saldo);
}
