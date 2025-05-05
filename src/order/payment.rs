use stripe::{CheckoutSessionPaymentMethodOptions, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData, CreateCheckoutSessionLineItemsPriceDataProductData, CreateCheckoutSessionPaymentMethodOptions, CreateCheckoutSessionPaymentMethodTypes, Currency};
use uuid::Uuid;
use stripe::CheckoutSessionMode;
use stripe::PaymentMethodType;

pub struct PaymentService {
    client: Client,
}

impl PaymentService {
    pub fn new(secret_key: &str) -> Self {
        Self {
            client: Client::new(secret_key),
        }
    }

    pub async fn create_checkout_session(
        &self,
        items: Vec<(String, i64, i32)>, // (name, price in cents, quantity)
        success_url: String,
        cancel_url: String,
    ) -> Result<String, stripe::StripeError> {
        let line_items: Vec<CreateCheckoutSessionLineItems> = items
            .into_iter()
            .map(|(name, price, quantity)| CreateCheckoutSessionLineItems {
                price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                    currency: Currency::INR,
                    unit_amount: Some(price),
                    product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                        name,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                quantity: Some(quantity as u64),
                ..Default::default()
            })
            .collect();

        let session = CreateCheckoutSession {
            success_url: Some(success_url.as_str()),
            cancel_url: Some(cancel_url.as_str()),
            payment_method_types : Some(vec![
                CreateCheckoutSessionPaymentMethodTypes::Card,
            ]),
            mode: Some(CheckoutSessionMode::Payment),
            line_items: Some(line_items),
            ..Default::default()
        };

        let checkout = stripe::CheckoutSession::create(&self.client, session).await?;
        Ok(checkout.url.unwrap_or_else(|| "no url".to_string()))
    }
}
