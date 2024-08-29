use super::order_model::CustomerOrder;

pub struct CustomerOrderHistory {
    orders: Vec<CustomerOrder>,
}

impl CustomerOrderHistory {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn place_order(&mut self, order: CustomerOrder) {
        self.orders.push(order);
    }

    pub fn view_orders(&self) -> Vec<&CustomerOrder> {
        self.orders.iter().collect()
    }

    pub fn remove_orders(&mut self, product_name: String) -> bool {
        if self
            .orders
            .iter()
            .any(|order| order.product_name == product_name)
        {
            self.orders
                .retain(|order| order.product_name != product_name);

            true
        } else {
            false
        }
    }

    pub fn modify_orders(&mut self, order: CustomerOrder) -> bool {
        match self
            .orders
            .iter_mut()
            .find(|product| product.product_name == order.product_name)
        {
            Some(details) => {
                details.quantity = order.quantity;
                details.total_price = order.total_price;

                true
            }
            None => false,
        }
    }
}
