fn main() {
    let idade:i8 = 17;
    let responsavel_autorizou = false;
    let eh_maior = idade >= 18;

    if idade >= 18 {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com a assinatura do responsável");
    } else {
        println!("Não pode entrar");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };
    println!("É {} de idade", condicao);

    match condicao {
        "maior" => { println!("Pode entrar") }
        _ => println!("Não pode entrar"),
    }

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