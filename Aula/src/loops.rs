fn laco_while() {
    let numero:i8 = 7;
    let mut multiplicador:i8 = 1;
    
    while multiplicador <= 10 {
        println!("{} x {} = {}", numero, multiplicador, numero * multiplicador);
        multiplicador += 1;
    } 
}

fn laco_loop() {
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

fn laco_for() {
    let numero:i8 = 8;
    
    for multiplicador in 1..=10 {
        println!("{} x {} = {}", numero, multiplicador, numero * multiplicador);
    }
}

fn main() {
    println!("Números pares de 1 a 10");
    for numero in 1..=10 {
        if numero % 2 != 0 {
            continue
        }
        println!("{} é par", numero);
    }

    println!("\nTabuada do 6 com loop");
    laco_loop();

    println!("\nTabuada do 7 com while");
    laco_while();

    println!("\nTabuada do 8 com for");
    laco_for();
}