fn main() {
    // creation
    let a: i16 = 5;

    // mutability
    let mut b = 5;
    b = 10;

    // shadowing - Cria 2 variaveis de mesmo nome, e a mais recente vai esconder a mais antiga
    let c = 10;
    let c = 20;

    println!("c is: {c}");

    // scope
    let d = 30; 

    { // inner scope
        let d = 40;
        println!("inner d is: {d}");
    }
    
    println!("d is: {d}")
}
