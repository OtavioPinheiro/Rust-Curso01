fn soma(a:i32, b:i32) -> i32 {
    a + b
}

fn main() {
    let a:i32 = 3;
    let b:i32 = 5;

    println!("O resultado da soma entre {} + {} = {}", a, b, soma(a,b));
}