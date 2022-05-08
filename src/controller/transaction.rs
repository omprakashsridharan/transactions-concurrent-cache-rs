use actix_web::{error, web, Responder, Result as ActixResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::AppData;

#[derive(Deserialize, Serialize, Validate)]
struct AddTransactionDto {
    #[validate(range(min = 0.0, message = "Invalid amount"))]
    amount: f32,
}

#[derive(Deserialize, Serialize)]
struct TransactionStats {
    total_amount: f32,
}

async fn add_transaction(
    add_transaction_dto: web::Json<AddTransactionDto>,
    data: web::Data<AppData>,
) -> ActixResult<String> {
    let cache = data.cache.clone();
    match add_transaction_dto.0.validate() {
        Ok(_) => {
            let current_timestamp = Utc::now();
            let amount = add_transaction_dto.0.amount;
            cache.insert(current_timestamp, amount).await;
            return Ok(current_timestamp.to_string());
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}

async fn get_statistics(data: web::Data<AppData>) -> ActixResult<impl Responder> {
    let mut total_amount: f32 = 0.0;
    let cache = data.cache.clone();
    for (_, amount) in cache.iter() {
        total_amount += amount;
    }
    Ok(web::Json(TransactionStats { total_amount }))
}

pub fn transaction_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/transactions")
            .route(web::post().to(add_transaction))
            .route(web::get().to(get_statistics)),
    );
}
