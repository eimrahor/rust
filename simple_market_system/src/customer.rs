use crate::product;

pub struct CustomerManager {
    pub customers: Vec<Customer>,
}

impl CustomerManager {
    pub fn adding_customers(&mut self, customers: Vec<Customer>) {
        for customer in customers {
            self.customers.push(customer);
        }
    }

    pub fn list_customers(&self) {
        for customer in &self.customers {
            println!("{:?}", customer);
        }
    }
}

#[derive(Debug)]
pub struct Customer {
    pub name: String,
    pub surname: String,
    pub balance: u16,
}

impl Customer {
    pub fn buy_product(&mut self, product: &mut product::Product, quantity: u16) -> bool {
        println!("Buying a product...");
        let total_cost = product.price * quantity;
        if self.balance >= total_cost {

            if self.balance < total_cost {
                return false;
            }
            self.balance -= total_cost;

            if product.stock_quantity < quantity {
                self.balance += total_cost;
                return false;
            }
            product.stock_quantity -= quantity;

            println!(
                "{} units {} purchased by {}, stock: {}, balance: {}",
                quantity, product.name, self.name, product.stock_quantity, self.balance
            );
            println!("--------------------------------------------------------------------");
            true
        } else {
            println!("Insufficient funds {}. Remaining balance: {}.", self.name, self.balance);
            println!("--------------------------------------------------------------------");
            false
        }
    }
}