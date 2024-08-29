use customer_order_model::CustomerOrderHistory;
use order_model::CustomerOrder;

use crate::{
    coffee_shop_operations::inventory_models::CoffeeShopInventory,
    utils::{get_quantity_input, get_string_input},
};

pub mod customer_order_model;
pub mod order_model;

pub fn place_order(order: &mut CustomerOrderHistory, coffee_inventory: &CoffeeShopInventory) {
    if coffee_inventory.view_products().is_empty() {
        println!("Sorry, no products are available.");
        return;
    }

    for product in coffee_inventory.view_products() {
        println!("{:#?}", product);
    }

    println!("Please enter the name of the product you want to order:");

    let name = match get_string_input() {
        Some(name) => name,
        None => return,
    };

    let binding = coffee_inventory.view_products();

    let product_option = binding.iter().find(|product| product.name == name);

    match product_option {
        Some(product) => {
            let quantity = match get_quantity_input() {
                Some(quantity) => quantity,
                None => return,
            };

            let total_price = quantity as f64 * product.price;

            order.place_order(CustomerOrder {
                product_name: product.name.to_string(),
                quantity,
                total_price,
            });
            println!("Order placed successfully!");
        }
        None => {
            println!("Sorry, the product '{}' is not available.", name);
        }
    }
}

pub fn view_all_orders(order: &CustomerOrderHistory) {
    if order.view_orders().is_empty() {
        println!("Sorry, no Orders are available.");
        println!("== ==");
        return;
    }

    for order in order.view_orders() {
        println!("{:#?}", order);
    }
}

pub fn delete_order(order: &mut CustomerOrderHistory) {
    if order.view_orders().is_empty() {
        println!("Sorry, no Orders are available.");
        println!("== ==");
        return;
    }

    for order in order.view_orders() {
        println!("{:#?}", order);
    }

    println!("Please enter the name of the order you want to Delete:");

    let order_name = match get_string_input() {
        Some(name) => name,
        None => return,
    };

    match order.remove_orders(order_name) {
        true => println!("Successfully deleted"),
        false => println!("order not found"),
    }
}

pub fn update_order(order: &mut CustomerOrderHistory, coffee_inventory: &CoffeeShopInventory) {
    if order.view_orders().is_empty() {
        println!("Sorry, Orders are not available.");
        println!("== ==");
        return;
    }

    for order in order.view_orders() {
        println!("{:#?}", order);
    }

    println!("Please enter the name of the order you want to Update:");

    let name = match get_string_input() {
        Some(name) => name,
        None => return,
    };

    let binding = coffee_inventory.view_products();

    let product_option = binding.iter().find(|product| product.name == name);

    match product_option {
        Some(product) => {
            let quantity = match get_quantity_input() {
                Some(quantity) => quantity,
                None => return,
            };

            let total_price = quantity as f64 * product.price;

            let modify_order = order.modify_orders(CustomerOrder {
                product_name: product.name.to_string(),
                quantity,
                total_price,
            });

            match modify_order {
                true => println!("successfully updated"),
                false => println!("order not found"),
            }
        }
        None => {
            println!("Sorry, the product '{}' is not available.", name);
        }
    }
}
