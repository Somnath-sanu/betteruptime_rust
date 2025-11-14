use poem::{
    handler,
    web::{Data, Json, Path},
};
use std::sync::{Arc, Mutex};
use store::store::Store;

use crate::{
    request_input::CreateWebsiteInput,
    request_output::{CreateWebsiteOutput, GetWebsiteOutput},
};

#[handler]
pub fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    // let mut s = Store::new_instance().unwrap();
    let mut locked_s = s.lock().unwrap();

    /*
    We had to make locked_s mutable as get_website needs &mut self,
    so, we don't have to pass like get_website(&mut locked_s , id);
    this happens automatically.

    when it goes out of scope, it's automatically gets unlocked
    */

    let website = locked_s.get_website(id).unwrap();

    Json(GetWebsiteOutput { url: website.url })

    // format!("hello: {name} ") // return string
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let url = data.url;

    println!("URL is {}", url);

    // persist this in DB
    // sqlx => close to pg in JS
    // diesel => close to prisma
    let mut locked_s = s.lock().unwrap();
    let website = locked_s.create_website(String::from("1"), url).unwrap();

    let response = CreateWebsiteOutput { id: website.id };

    Json(response)
}
