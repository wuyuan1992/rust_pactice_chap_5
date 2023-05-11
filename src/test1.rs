struct Product {
    name: String,
    price: f32,
    quantity: i32,
}

impl Product {
    // Define your `total_value` method here
    fn new(name: String) -> Product {
        Product {
            name,
            price: 22500.0,
            quantity: 18,
        }
    }

    fn total_value(&self) -> f32 {
        self.price * self.quantity as f32
    }
}

fn main() {
    let product = Product::new(String::from("Wuyuan"));

    // Call the `total_value` method on `product` and print the result here
    let total = product.total_value();
    println!("Total value of {}: {}", product.name, total);
}
