fn main() {
    let mut s1 = String::from("Rust");
    let r1 = &s1; // define referencia com &
    print_string(r1); // poderiamos passar &s1 direto
    println!("s1 is :{s1}");

    let r2 = &mut s1; // refencias são imutaveis por padrão
    add_to_string(r2);
    println!("s1 is: {s1}");


    let s2 = generate_string();
}

fn generate_string() -> String {
    let s = String::from("Ferris");
    s // Devolver a referencia daria problema, pois o s seria excluido e a referencia não existiria mais
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome"); // Rust tem automatico dereference, então não precisamos passar (*p1).push_str
}

fn print_string(p1: &str) {
    println!("p1 is :{p1}");
}
