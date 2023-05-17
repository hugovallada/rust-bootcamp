fn main() {
    let s1 = String::from("Rust"); // Não vamos poder usar o valor de s1, pq o owner agora é s2
    let s2 = s1.clone(); // Quando usamos o clone, copiamos o valor ao invés de mover, ai podemos usar o s1 novamente

    print_string(s1.clone()); // passar a variavel como função, o ownership vai mudar para o p1, podemos fazer um clone
    println!("s1 is {s1}");

    let x = 10; // Tipos primitivos que são salvos na stack, são clonados por default ao invés de mover
    let i = x;

    println!("{x}");

    let s3 = generate_string();
    println!("{s3}");

    let s4 = add_to_string(s2);
    println!("{s4}");

    print_integer(x); // Vai ser automaticamente clonado para primitivos que ficam na stack, são cl
    println!("x is {x}");
}

fn print_integer(i: i32) {
    println!("i is {i}")
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
fn generate_string() -> String {
    String::from("Ferris")
}

fn print_string(p1: String) {
    println!("{}", p1);
}
