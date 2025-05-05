# 🛒 E-Commerce Backend API (Rust + Actix Web)

> 📌 [Project URL on Roadmap.sh](https://roadmap.sh/projects/ecommerce-api)

This project is a simple backend for an e-commerce application built using **Rust**, **Actix Web**, and **PostgreSQL**. It supports user authentication, product browsing, shopping cart management, and checkout functionality.

## 🚀 Features

- ✅ User Sign-Up & Login with JWT
- ✅ Product Listing & Search
- ✅ Add to Cart, View Cart, Remove Item from Cart
- ✅ Checkout functionality
- ✅ PostgreSQL integration using SQLx

## 🧱 Tech Stack

- **Backend**: Rust, Actix Web
- **Database**: PostgreSQL
- **ORM**: SQLx
- **Authentication**: JWT

## 📦 API Endpoints

### 🔐 Authentication
| Method | Endpoint    | Description        |
|--------|-------------|--------------------|
| POST   | `/signUp`   | Register new user  |
| POST   | `/logIn`    | Login user         |

### 🛍️ Products
| Method | Endpoint          | Description             |
|--------|-------------------|-------------------------|
| GET    | `/product/all`    | Get all products        |
| GET    | `/product/{id}`   | Get product by ID       |
| POST   | `/search`         | Search for a product    |

### 🛒 Cart
| Method | Endpoint               | Description                    |
|--------|------------------------|--------------------------------|
| GET    | `/create_cart`         | Create a cart for a user       |
| POST   | `/addToCart`           | Add product to user's cart     |
| GET    | `/myCart`              | View all items in the cart     |
| GET    | `/flushCart`           | Remove all items from cart     |
| GET    | `/removeItem-crat`     | Remove an item or reduce qty   |

### 💳 Checkout
| Method | Endpoint      | Description                |
|--------|---------------|----------------------------|
| GET    | `/checkout`   | Proceed to checkout        |


## 🛠️ Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL installed and running
- [sqlx-cli](https://crates.io/crates/sqlx-cli) for database setup

### Run the server

```bash
# Clone the repository
git clone https://github.com/your-username/your-repo-name.git
cd your-repo-name

# Set your environment variables in `.env`
DATABASE_URL=postgres://username:password@localhost/database
JWT_SECRET=your_secret_key
STRIPE_SECRET=stripe_secret_key

# Run the project
cargo run
