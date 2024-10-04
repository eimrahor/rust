mod customer;
mod product;
use rand::Rng;

fn main() {
    let mut cmanager = customer::CustomerManager {
        customers: Vec::new(), 
    };

    let new_customers = vec![
        customer::Customer {
            name: "John".to_string(),
            surname: "Doe".to_string(),
            balance: 10000,
        },
        customer::Customer {
            name: "Daniel".to_string(),
            surname: "Schwartz".to_string(),
            balance: 15000,
        },
        customer::Customer {
            name: "James".to_string(),
            surname: "Smith".to_string(),
            balance: 7000,
        },
        customer::Customer {
            name: "Elaine".to_string(),
            surname: "Holt".to_string(),
            balance: 12000,
        },
        customer::Customer {
            name: "Melinda".to_string(),
            surname: "Maitland".to_string(),
            balance: 18000,
        },
    ];

    let cLen = new_customers.len();
    cmanager.adding_customers(new_customers);

    let mut pmanager = product::ProductManager {
        products: Vec::new(), 
    };

    let new_products = vec![
        product::Product {
            name: "Sofa".to_string(),
            price: 1000,
            stock_quantity: 20,
        },
        product::Product {
            name: "Chair".to_string(),
            price: 35,
            stock_quantity: 700,
        },
        product::Product {
            name: "Desk".to_string(),
            price: 200,
            stock_quantity: 100,
        },
        product::Product {
            name: "Table".to_string(),
            price: 350,
            stock_quantity: 150,
        },
        product::Product {
            name: "Furniture".to_string(),
            price: 900,
            stock_quantity: 24,
        },
    ];

    let pLen = new_products.len();
    pmanager.adding_products(new_products);

    let mut iter: u8 = 0;
    let mut cnumber: usize = 0;
    let mut pnumber: usize = 0;
    let mut quantity: u16 = 0;
    loop {
        cnumber = rand::thread_rng().gen_range(0,cLen);
        pnumber = rand::thread_rng().gen_range(0,pLen);
        quantity = rand::thread_rng().gen_range(1,6);
        if !cmanager.customers[cnumber].buy_product(&mut pmanager.products[pnumber], quantity) {
            iter += 1
        }
        if iter == 2 { break; }
    }
}