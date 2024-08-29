pub mod auth_menu;
pub mod custome_menu;
pub mod product_menu;

use custome_menu::CustomerMenu;
use product_menu::CoffeeShopMenu;

use crate::{
    coffee_shop_operations::{self, inventory_models::CoffeeShopInventory},
    customer_order_operations::{self, customer_order_model::CustomerOrderHistory},
    utils::get_string_input,
};

pub fn run_admin_program(
    coffee_inventory: &mut CoffeeShopInventory,
    customer_orders: &CustomerOrderHistory,
) -> Option<()> {
    loop {
        CoffeeShopMenu::show();

        let shop_input = get_string_input()?;

        match CoffeeShopMenu::select_menu_option(&shop_input) {
            Some(CoffeeShopMenu::AddProduct) => {
                coffee_shop_operations::add_coffee_product(coffee_inventory)
            }
            Some(CoffeeShopMenu::ViewProducts) => {
                coffee_shop_operations::view_coffee_products(coffee_inventory)
            }
            Some(CoffeeShopMenu::RemoveProduct) => {
                coffee_shop_operations::remove_coffee_product(coffee_inventory)
            }
            Some(CoffeeShopMenu::UpdateProduct) => {
                coffee_shop_operations::update_coffee_product(coffee_inventory)
            }
            Some(CoffeeShopMenu::ViewOrders) => {
                customer_order_operations::view_all_orders(customer_orders)
            }
            Some(CoffeeShopMenu::ReturnToPreviousMenu) => break,
            None => break,
        }
    }

    None
}

pub fn run_client_program(
    customer_orders: &mut CustomerOrderHistory,
    coffee_inventory: &CoffeeShopInventory,
) -> Option<()> {
    loop {
        CustomerMenu::display_menu();

        let order_input = get_string_input()?;

        match CustomerMenu::select_menu_option(&order_input) {
            Some(CustomerMenu::ViewCoffeeProducts) => {
                coffee_shop_operations::view_coffee_products(coffee_inventory)
            }
            Some(CustomerMenu::PlaceOrder) => {
                customer_order_operations::place_order(customer_orders, coffee_inventory)
            }
            Some(CustomerMenu::ViewOrders) => {
                customer_order_operations::view_all_orders(customer_orders)
            }
            Some(CustomerMenu::CancelOrder) => {
                customer_order_operations::delete_order(customer_orders)
            }
            Some(CustomerMenu::ModifyOrder) => {
                customer_order_operations::update_order(customer_orders, coffee_inventory)
            }
            Some(CustomerMenu::ReturnToPreviousMenu) => break,
            None => break,
        }
    }

    None
}
