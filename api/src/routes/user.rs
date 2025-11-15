use jsonwebtoken::{EncodingKey, Header, encode};
use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use store::store::Store;

use crate::{
    request_input::CreateUserInput,
    request_output::{CreateUserOutput, SigninOutput},
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<CreateUserOutput>, Error> {
    let mut locked_s = s.lock().unwrap();

    let user_id = locked_s.sign_up(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let response = CreateUserOutput { id: user_id };

            Ok(Json(response))
        }
        Err(_) => Err(Error::from_status(StatusCode::CONFLICT)),
    }
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Result<Json<SigninOutput>, Error> {
    let mut locked_s = s.lock().unwrap();

    let user_id = locked_s.sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let my_claims = Claims {
                sub: user_id,
                exp: 1111111111111, // Infinite for now
            };

            let token = encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret("my_jwt_secret".as_ref()),
            )
            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SigninOutput { jwt: token };

            Ok(Json(response))
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}
