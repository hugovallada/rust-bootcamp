struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    fn calculate_sales_tax_rate(&self) -> f32 {
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought");
        123
    }

    // Associate function - Chamadas de static methods em outras linguagens
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn new(name: String, price: f32) -> Product {
        Product { name: name, price: price, in_stock: true }
    }
}

fn main() {
    let mut book = Product {
        name: String::from("Book"),
        price: 100.0,
        in_stock: true,
    };
    book.in_stock = false;
    let sales_tax = book.calculate_sales_tax_rate();
    println!("Sales: {}", sales_tax);
    book.set_price(1.0);
    book.buy();

    let default_tax = Product::get_default_sales_tax();
    println!("Default tax: {default_tax}");

    let car = Product::new(String::from("Carro"), 20000.00);
}
