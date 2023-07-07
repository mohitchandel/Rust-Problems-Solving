#[derive(Debug)]
struct Product {
    identifier: u128,
    name: String,
    price: f64,
    quantity: u64,
}

impl Product {
    fn new_product(identifier: u128, name: String, price: f64, quantity: u64) -> Product {
        Product {
            identifier,
            name,
            price,
            quantity,
        }
    }

    fn calculate_total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            products: Vec::new(),
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn remove_product(&mut self, identifier: u128) {
        self.products.retain(|p| p.identifier != identifier);
    }

    fn get_product(&self, identifier: u128) -> Option<&Product> {
        self.products.iter().find(|p| p.identifier == identifier)
    }

    fn update_product_quantity(&mut self, identifier: u128, new_quantity: u64) {
        if let Some(product) = self
            .products
            .iter_mut()
            .find(|p| p.identifier == identifier)
        {
            product.quantity = new_quantity;
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    let laptops = Product::new_product(101, String::from("Laptops"), 500.99, 10);
    let mobiles = Product::new_product(102, String::from("Mobiles"), 100.99, 12);
    let tablets = Product::new_product(103, String::from("Tablets"), 250.12, 18);

    inventory.add_product(laptops);
    inventory.add_product(mobiles);
    inventory.add_product(tablets);

    inventory.remove_product(103);

    if let Some(product) = inventory.get_product(101) {
        println!("Product: {:?}", product);
    } else {
        println!("Product not found");
    }

    inventory.update_product_quantity(101, 15);
}
