fn main() {
    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    let v2 = vec![1, 2, 3];

    let s = &v[0]; // can panic in case of invalid index
                   //let s2 = v.remove(0); // return an element from the vector

    let s2 = v.get(0); // does not panic, because return an option

    if let Some(s1) = v.get(0) {
        println!("{s1}");
    }

    // Iterate over the elements
    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}");
    }

    let mut v3: Vec<String> = vec![];

    for s in v {
        // Movendo ao invés de pegar emprestado, então v não vai ser mais usável. Chamar assim, é a mesma coisa que fazer v.into_iter()
        v3.push(s)
    }
}
