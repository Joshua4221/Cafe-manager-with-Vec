pub enum CoffeeShopMenu {
    AddProduct,
    ViewProducts,
    RemoveProduct,
    UpdateProduct,
    ViewOrders,
    ReturnToPreviousMenu,
}

impl CoffeeShopMenu {
    pub fn select_menu_option(input: &str) -> Option<Self> {
        match input {
            "1" => Some(CoffeeShopMenu::AddProduct),
            "2" => Some(CoffeeShopMenu::ViewProducts),
            "3" => Some(CoffeeShopMenu::RemoveProduct),
            "4" => Some(CoffeeShopMenu::UpdateProduct),
            "5" => Some(CoffeeShopMenu::ViewOrders),
            "6" => Some(CoffeeShopMenu::ReturnToPreviousMenu),
            _ => None,
        }
    }

    pub fn show() {
        println!("== Coffee Shop Admin Menu ==");
        println!("1. Add a product");
        println!("2. View products");
        println!("3. Remove a product");
        println!("4. Update a product");
        println!("5. View orders");
        println!("6. Return to previous menu");
        println!("=============================");
        println!("Please enter your selection:");
    }
}
