use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::lib::jwt::{Jwt, TokenType};
use crate::lib::result::Result;
#[derive(Deserialize)]
pub struct Args {
    refresh_token: String,
}

pub async fn refresh_access_token(
    args: web::Json<Args>,
    jwt: web::Data<Jwt>,
) -> Result<impl Responder> {
    let claims = jwt.validate_jwt(&args.refresh_token, TokenType::refresh)?;
    let response = jwt.create_tokens(claims)?;

    Ok(HttpResponse::Ok().json(response))
}
