use actix_web::dev::ServiceRequest;
use actix_web::{web, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AppState {
    pub jwt_secret: String,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let data = match req.app_data::<web::Data<AppState>>() {
        Some(data) => data,
        None => {
            let error = actix_web::error::ErrorInternalServerError("App state not found");
            return Err((error, req));
        }
    };

    let token = credentials.token();
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(data.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => token_data,
        Err(_) => {
            let error = actix_web::error::ErrorUnauthorized("Invalid token");
            return Err((error, req));
        }
    };

    let now = match std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as usize)
    {
        Ok(now) => now,
        Err(_) => {
            let error = actix_web::error::ErrorInternalServerError("Time error");
            return Err((error, req));
        }
    };

    if token_data.claims.exp < now {
        let error = actix_web::error::ErrorUnauthorized("Token expired");
        return Err((error, req));
    }

    req.extensions_mut().insert(token_data.claims);

    Ok(req)
}
