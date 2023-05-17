use std::fmt::format;

fn mainv1() {
    let s1 = "Hello, world!";
    let s2 = String::from("Hello, world!");
    let s3 = "Hello, world!".to_string();
    let s4 = "Hello, world!".to_owned();
    let s5 = &s4[..];

    println!("{s5}")
}

fn mainv2() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    s.replace_range(.., "baz");
    println!("{s}")
}

fn mainv3() {
   let s1 = String::from("Hello, ");
   let s2 = String::from("world!");
   let s3 = s1 + &s2;

   println!("{s3}");
}

fn mainv4() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{s}")
}

fn mainv5() {
    let s1 = ["first", "second"].concat();
    let s2 = format!("{}{}", "first", "second");
    let s3 = concat!("first", "second");
    let s4 = String::from("teste");
    let s5 = s4+"okoko";
}

fn main() {
    let s1 = "ola ola ola ola ola";
    let s2 = &s1[0..4];
    println!("{s2}");

    iterate_over_string(s1)
}

fn iterate_over_string(value: &str) {
    for b in value.chars(){
        println!("{b}");
    }
}
