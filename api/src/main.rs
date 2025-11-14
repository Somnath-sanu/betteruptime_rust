use std::sync::{Arc, Mutex};

use poem::{EndpointExt, Route, Server, get, listener::TcpListener, post};
use store::store::Store;

use crate::routes::{
    user::{sign_in, sign_up},
    website::{create_website, get_website},
};

pub mod request_input; // include the file
pub mod request_output;
pub mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new_instance().unwrap()));
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in))
        .data(s);

    //creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}

// Json(data) means we are destructuring like
// let u = {name: "shanu"}; let {name} = u;
// since Json is a unit struct , if we use this data: Json<CreateWebsiteInput>
// we have to access like this const url = data.0.url;

/*
If I would have direcly applied the clone trait to a struct and pass it
accoss threads, we logged the address , everytime we are geeeting the different address , means differnet DB connections each time, problem remails the same
so we used Arc , but since we need mut refrecence , we need mutex as well
*/
