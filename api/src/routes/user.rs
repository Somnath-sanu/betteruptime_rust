use poem::{
    handler,
    web::{Data, Json},
};
use std::sync::{Arc, Mutex};
use store::store::Store;

use crate::{
    request_input::CreateUserInput,
    request_output::{CreateUserOutput, SigninOutput},
};

#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    let mut locked_s = s.lock().unwrap();

    let user_id = locked_s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput { id: user_id };

    Json(response)
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<SigninOutput> {
    let mut locked_s = s.lock().unwrap();

    let _user_id = locked_s
        .sign_in(data.username.clone(), data.password)
        .unwrap();

    let response = SigninOutput { jwt: data.username };

    Json(response)
}
