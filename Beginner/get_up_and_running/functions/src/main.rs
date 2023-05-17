fn main() {
    let valor = my_function(10);
    println!("{valor}");
}

fn my_function(x: i32) -> i32 {
    println!("my_function called with: {x}");
    x
}
