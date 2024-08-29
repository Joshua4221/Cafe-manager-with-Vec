use utils::get_string_input;

mod coffee_shop_operations;
mod customer_order_operations;
mod menus;
mod utils;

use coffee_shop_operations::inventory_models::CoffeeShopInventory;
use customer_order_operations::customer_order_model::CustomerOrderHistory;
use menus::{auth_menu::AuthMenu, run_admin_program, run_client_program};

fn run_program() -> Option<()> {
    let mut customer_orders = CustomerOrderHistory::new();
    let mut coffee_inventory = CoffeeShopInventory::new();

    loop {
        AuthMenu::show_auth_type();

        let auth_input = get_string_input()?;

        match AuthMenu::select_auth_type(&auth_input) {
            Some(AuthMenu::Admin) => run_admin_program(&mut coffee_inventory, &customer_orders),
            Some(AuthMenu::Client) => run_client_program(&mut customer_orders, &coffee_inventory),
            None => break,
        };
    }

    None
}

fn main() {
    run_program();
}
