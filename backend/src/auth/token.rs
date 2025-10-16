use chrono::Utc;
use jsonwebtoken::{
     decode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
     errors::Error as JwtError
};

use crate::models::models::{Claims, User};

pub async fn get_jwt(user: User) -> Result<String, JwtError>{
     let sec_key = std::env::var("SECRET_KEY").expect("secret_key must be in env");
     let expir = Utc::now()
          .checked_add_signed(chrono::Duration::hours(24))
          .expect("invalid timestamp")
          .timestamp();
     let claim = Claims{
          sub: user.username.to_string(),
          exp: expir as usize,
     };
     
     jsonwebtoken::encode(
          &Header::new(Algorithm::HS256),
          &claim,
          &EncodingKey::from_secret(sec_key.as_bytes()),
     )
}

pub async fn tokend_data_extractor(
     token: String,
     secret: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
     let validation = Validation::new(Algorithm::HS256);
     let token_message = 
     decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &validation);
     return token_message;
}

