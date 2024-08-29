pub enum CustomerMenu {
    ViewCoffeeProducts,
    PlaceOrder,
    ViewOrders,
    CancelOrder,
    ModifyOrder,
    ReturnToPreviousMenu,
}

impl CustomerMenu {
    pub fn select_menu_option(input: &str) -> Option<Self> {
        match input {
            "1" => Some(CustomerMenu::ViewCoffeeProducts),
            "2" => Some(CustomerMenu::PlaceOrder),
            "3" => Some(CustomerMenu::ViewOrders),
            "4" => Some(CustomerMenu::CancelOrder),
            "5" => Some(CustomerMenu::ModifyOrder),
            "6" => Some(CustomerMenu::ReturnToPreviousMenu),
            _ => None,
        }
    }

    pub fn display_menu() {
        println!("== Coffee Shop Customer Menu ==");
        println!("1. View Coffee Products");
        println!("2. Place an Order");
        println!("3. View Your Orders");
        println!("4. Cancel an Order");
        println!("5. Modify an Order");
        println!("6. Return to previous menu");
        println!("============================");
        println!("Please enter your selection:");
    }
}
