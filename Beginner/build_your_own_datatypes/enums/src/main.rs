struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

enum ProductCategory {
    Books,
    Clothing,
    Eletronics,
}

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from(
                "{\"cmd\": \"undo\"}"
            ),
            Command::Redo => String::from(
                "{ \"cmd\": \"redo\" }"
            ),
            Command::AddText(s) => {
                format!(
                    "{{\
                        \"cmd\": \"add_text\",\
                        \"text\": \"{s}\"\
                    }}"
                )
            },
            Command::MoveCursor(line, column) => {
                format!(
                    "{{\
                        \"cmd\": \"move_cursor\",\
                        \"line\": {line},\
                        \"column\": {column}\
                    }}"
                )
            },
            Command::Replace { from, to } => {
                format!(
                    "{{\
                        \"cmd\": \"replace\",\
                        \"from\": \"{from}\",\
                        \"to\": \"{to}\"\
                    }}"
                )
            }
        };
        return json_string;
    }
}

fn main() {
    // let category = ProductCategory::Eletronics;
    // let product = Product {
    //     name: String::from("TV"),
    //     category,
    //     price: 500.0,
    //     in_stock: true,
    // };

    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace {
        from: String::from("test"),
        to: String::from("test2"),
    };

    let json_string = cmd1.serialize();
    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());

    // let age = 35;

    // match age {
    //     // Tipo o switch
    //     1 => println!("Happy first year"),
    //     13..=19 => println!("You are a teenager!"),
    //     other => println!("You are {other} years old"),
    // };
}
