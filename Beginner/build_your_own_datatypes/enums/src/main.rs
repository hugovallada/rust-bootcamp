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
        String::from("Json value")
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

    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace {
        from: String::from("test"),
        to: String::from("test2"),
    };

    let json_string = cmd.serialize();
}
