static GLOBAL:f32 = 1.22;
fn main() {
    let variavel_int:i8 = 127;
    println!("Variável inteiro: {}, Tamanho = {} bytes ou {} bits", variavel_int, std::mem::size_of_val(&variavel_int), std::mem::size_of_val(&variavel_int)*8);

    let variavel_unsigned:u8 = 128;
    println!("variável unsigned = {}, Tamanho = {} bytes ou {} bits", variavel_unsigned, std::mem::size_of_val(&variavel_unsigned), std::mem::size_of_val(&variavel_unsigned)*8);

    let variavel = 300;
    println!("variavel = {}, Tamanho = {} bytes ou {} bits", variavel, std::mem::size_of_val(&variavel), std::mem::size_of_val(&variavel)*8);

    let decimal:f32 = 2.5;
    println!("decimal = {}, Tamanho = {} bytes ou {} bits", decimal, std::mem::size_of_val(&decimal), std::mem::size_of_val(&decimal)*8);

    let booleana:bool = false;
    println!("booleana = {}, Tamanho = {} bytes ou {} bits", booleana, std::mem::size_of_val(&booleana), std::mem::size_of_val(&booleana)*8);

    let letra:char = 'C';
    println!("letra = {}, Tamanho = {} bytes ou {} bits", letra, std::mem::size_of_val(&letra), std::mem::size_of_val(&letra)*8);

    const PI:f32 = 3.14;
    println!("PI = {}, Tamanho = {} bytes ou {} bits", PI, std::mem::size_of_val(&PI), std::mem::size_of_val(&PI)*8);

    println!("GLOBAL = {}, Tamanho = {} bytes ou {} bits", GLOBAL, std::mem::size_of_val(&GLOBAL), std::mem::size_of_val(&GLOBAL)*8);
}
