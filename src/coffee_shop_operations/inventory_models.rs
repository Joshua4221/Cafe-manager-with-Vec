use super::product_model::CoffeeProduct;

pub struct CoffeeShopInventory {
    inventory: Vec<CoffeeProduct>,
}

impl CoffeeShopInventory {
    pub fn new() -> Self {
        Self {
            inventory: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: CoffeeProduct) {
        self.inventory.push(product);
    }

    pub fn view_products(&self) -> Vec<&CoffeeProduct> {
        self.inventory.iter().collect()
    }

    pub fn remove_product(&mut self, product_name: String) -> bool {
        if self
            .inventory
            .iter()
            .any(|product| product.name == product_name)
        {
            self.inventory
                .retain(|product| product.name != product_name);

            true
        } else {
            false
        }
    }

    pub fn update_product(&mut self, name: String, price: f64) -> bool {
        match self
            .inventory
            .iter_mut()
            .find(|product| product.name == name)
        {
            Some(product) => {
                product.price = price;

                true
            }
            None => false,
        }
    }
}
