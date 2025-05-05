use crate::AppState;
use crate::auth::models::AuthenticatedUser;
use crate::order::models::CheckOut;
use crate::order::payment::PaymentService;
use crate::routes::models::ApiResponse;
use actix_web::{HttpResponse, Responder, post, web};
use serde_json::json;
use uuid::Uuid;

pub async fn create_checkout(
    data: web::Data<AppState>,
    order: web::Json<CheckOut>,
    user: AuthenticatedUser,
) -> impl Responder {
    // Step 1: Fetch cart items
    let cart_items = match sqlx::query!(
        r#"
        SELECT ci.quantity, p.name, p.price
        FROM cart_items ci
        JOIN products p ON p.id = ci.product_id
        WHERE ci.cart_id = $1
        "#,
        order.cart_id
    )
        .fetch_all(&data.db_pool)
        .await
    {
        Ok(items) => items,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "Error".to_string(),
                msg: "Failed to fetch cart items".to_string(),
                data: json!({}),
            });
        }
    };

    // Step 2: Prepare Stripe items and calculate total amount
    let mut stripe_items = vec![];
    let mut total_amount: i64 = 0;

    for item in cart_items {
        // Ensure name, price, and quantity are not null
        let name = item.name; 
        
        let price =  item.price;

        let quantity = item.quantity;

        // Stripe works with amounts in cents (i.e., 100 = ₹1.00)
        let item_total = (price * quantity as f64 * 100.0).round() as i64;
        total_amount += item_total;

        stripe_items.push((
            name.clone(),  // clone String from Option<String>
            (price * 100.0).round() as i64, // price in cents
            quantity,
        ));
    }


    if stripe_items.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse {
            status: "Error".to_string(),
            msg: "Cart is empty or contains invalid items".to_string(),
            data: json!({}),
        });
    }

    // Step 3: Create Stripe checkout session
    let payment_service = PaymentService::new(&data.stripe_secret);
    let session_result = payment_service
        .create_checkout_session(
            stripe_items,
            "http://localhost:8080/success".to_string(),
            "http://localhost:8080/cancel".to_string(),
        )
        .await;

    match session_result {
        Ok(session_url) => {
            // Step 4: Insert into orders table
            let user_id = match Uuid::parse_str(&user.claims.sub) {
                Ok(uid) => uid,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Invalid user ID in claims: {}", e));
                }
            };

            if let Err(e) = sqlx::query!(
                r#"
                INSERT INTO orders (user_id, payment_id, status, total_amount, currency)
                VALUES ($1, $2, 'pending', $3, 'usd')
                "#,
                user_id,
                session_url.clone(), // Store the session URL or session ID if available
                total_amount
            )
                .execute(&data.db_pool)
                .await
            {
                return HttpResponse::InternalServerError().body(format!("DB Insert error: {}", e));
            }

            // Step 5: Respond with session URL
            HttpResponse::Ok().json(ApiResponse {
                status: "Success".to_string(),
                msg: "Redirect the URL to pay".to_string(),
                data: json!({ "checkout_url": session_url }),
            })
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Stripe error: {}", e)),
    }
}
