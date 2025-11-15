use poem::{
    Error, handler,
    http::StatusCode,
    web::{Data, Json, Path},
};
use std::sync::{Arc, Mutex};
use store::store::Store;

use crate::{
    auth_middleware::UserId,
    request_input::CreateWebsiteInput,
    request_output::{CreateWebsiteOutput, GetWebsiteOutput},
};

#[handler]
pub fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId, /*
                              * To access this either write like
                              * let id = user_id.0
                              *
                              * OR
                              *
                              * UserId(user_id): UserId
                              * This will destructure it like we destructure objects in JS
                              */
) -> Result<Json<GetWebsiteOutput>, Error> {
    // let mut s = Store::new_instance().unwrap();
    let mut locked_s = s.lock().unwrap();

    /*
    We had to make locked_s mutable as get_website needs &mut self,
    so, we don't have to pass like get_website(&mut locked_s , id);
    this happens automatically.

    when it goes out of scope, it's automatically gets unlocked
    */

    let website = locked_s
        .get_website(id, user_id)
        .map_err(|_| Error::from_string("DB error", StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(GetWebsiteOutput {
        url: website.url,
        id: website.id,
        user_id: website.user_id,
    }))

    // format!("hello: {name} ") // return string
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
    UserId(user_id): UserId,
) -> Result<Json<CreateWebsiteOutput>, Error> {
    let url = data.url;

    // persist this in DB
    // sqlx => close to pg in JS
    // diesel => close to prisma
    let mut locked_s = s.lock().unwrap();
    let website = locked_s
        .create_website(user_id, url)
        .map_err(|_| Error::from_string("DB error", StatusCode::INTERNAL_SERVER_ERROR))?;

    let response = CreateWebsiteOutput { id: website.id };

    Ok(Json(response))
}
