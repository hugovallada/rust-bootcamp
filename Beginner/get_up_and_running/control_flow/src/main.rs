fn main() {
    let mut a = 5;

    if a > 5 {
        println!("Bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    let b = if a > 5 { 1 } else { -1 };

    // roda pra sempre até um break - Pode retornar um valor... não é mto comum
    let x: i32 = 'outer: loop {
        println!("Loop forever");
        loop {
            break 'outer 3; // se não nomear, ele vai sair só do mais interno
        }
    };

    while a < 5 {
        println!("a is {a}");
        a += 1;
    }

    // for -> itera sobre collections
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("{element}");
    }
}
