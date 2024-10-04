pub struct ProductManager {
    pub products: Vec<Product>,
}

impl ProductManager {
    pub fn adding_products(&mut self, products: Vec<Product>) {
        for product in products {
            self.products.push(product);
        }
    }

    pub fn list_products(&self) {
        for product in &self.products {
            println!("{:?}", product);
        }
    }
}

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: u16,
    pub stock_quantity: u16,
}