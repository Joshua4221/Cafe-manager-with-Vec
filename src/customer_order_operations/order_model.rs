#[derive(Debug, Clone)]
pub struct CustomerOrder {
    pub product_name: String,
    pub quantity: u32,
    pub total_price: f64,
}
