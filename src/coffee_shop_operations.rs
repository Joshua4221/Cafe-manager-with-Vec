use crate::utils;

pub mod inventory_models;
pub mod product_model;

use inventory_models::CoffeeShopInventory;

pub fn add_coffee_product(coffee_inventory: &mut CoffeeShopInventory) {
    println!("Input a product name:");

    let name = match utils::get_string_input() {
        Some(name) => name,
        None => return,
    };

    let price = match utils::get_amount_input() {
        Some(amount) => amount,
        None => return,
    };

    coffee_inventory.add_product(product_model::CoffeeProduct { name, price });
    println!("Product added to inventory.");
}

pub fn view_coffee_products(coffee_inventory: &CoffeeShopInventory) {
    if coffee_inventory.view_products().is_empty() {
        println!("Sorry, no Product are available.");
        println!("== ==");
        return;
    }

    for product in coffee_inventory.view_products() {
        println!("{:#?}", product);
    }
}

pub fn remove_coffee_product(coffee_inventory: &mut CoffeeShopInventory) {
    if coffee_inventory.view_products().is_empty() {
        println!("Sorry, no Product are available.");
        println!("== ==");
        return;
    }

    for product in coffee_inventory.view_products() {
        println!("{:#?}", product);
    }

    println!("Select the product you want to remove:");

    let name = match utils::get_string_input() {
        Some(name) => name,
        None => return,
    };

    match coffee_inventory.remove_product(name) {
        true => println!("Product successfully removed."),
        false => println!("Product not found in inventory."),
    }
}

pub fn update_coffee_product(coffee_inventory: &mut CoffeeShopInventory) {
    if coffee_inventory.view_products().is_empty() {
        println!("Sorry, no Product are available.");
        println!("== ==");
        return;
    }

    for product in coffee_inventory.view_products() {
        println!("{:#?}", product);
    }

    println!("Select the product name you want to edit:");

    let name_to_edit = match utils::get_string_input() {
        Some(name) => name,
        None => return,
    };

    let new_price = match utils::get_amount_input() {
        Some(amount) => amount,
        None => return,
    };

    match coffee_inventory.update_product(name_to_edit, new_price) {
        true => println!("Product updated successfully."),
        false => println!("Product not found."),
    }
}
