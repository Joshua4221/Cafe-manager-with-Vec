# Coffee Shop Inventory Management System

This project is a command-line based Coffee Shop Inventory Management System, implemented in Rust using a Vec to manage inventory and client orders. The system supports CRUD operations for products, allowing administrators to add, view, update, and remove products from the inventory, and handle client orders.

## Features

### Admin Features:
- **Add a Product:** Administrators can add new products with a name and price to the inventory.
- **View Products:** List all available products with their details.
- **Remove a Product:** Delete a product from the inventory by specifying its name.
- **Update a Product:** Modify the price of an existing product.
- **View Orders:** Review client orders and respond accordingly.
- **Navigation:** Easily return to the main menu or authentication menu.

### Client Features:
- **View Coffee Products:** Browse the available products in the inventory.
- **Place an Order:** Select and order available products.
- **View Your Orders:** Review your current and past orders.
- **Cancel an Order:** Cancel an existing order.
- **Modify an Order:** Update details of an existing order.
- **Return to Previous Menu:** Navigate back to the main menu.

## Project Structure

```bash
COFFEE_SHOP
│
├── src
│   ├── coffee_shop_operations
│   │   ├── coffee_shop_operations.rs       # Core operations for managing coffee shop inventory
│   │   ├── inventory_models.rs             # Models related to inventory items
│   │   └── product_model.rs                # Product data structure and related functions
│   │
│   ├── customer_order_operations
│   │   ├── customer_order_operations.rs    # Handles customer order logic
│   │   └── customer_order_model.rs         # Order data structure and related functions
│   │
│   ├── menus
│   │   └── menus.rs                        # Menu interactions for both admin and clients
│   │
│   ├── utils.rs                            # Utility functions for the project
│   └── main.rs                             # Main entry point of the application
│
├── Cargo.toml                              # Rust project configuration file
└── .gitignore                              # Files and directories to ignore in version control
```


### Technology Stack
- **Language:** Rust.
- **Data Structure:** HashMap for storing and managing products and orders.


## Getting Started

### Clone the repository and run the project using Rust:
```bash
git clone <repository-url>
cd COFFEE_SHOP
cargo run
```
