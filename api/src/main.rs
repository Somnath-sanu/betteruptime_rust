use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};
use store::store::Store;

use crate::{
    request_input::{CreateUserInput, CreateWebsiteInput},
    request_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput},
};

pub mod request_input; // include the file
pub mod request_output;

#[handler]
fn get_website(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::default().unwrap();

    let website = s.get_website(id).unwrap();

    Json(GetWebsiteOutput { url: website.url })

    // format!("hello: {name} ") // return string
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s = Store::default().unwrap();

    let user_id = s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput { id: user_id };

    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SigninOutput> {
    let mut s = Store::default().unwrap();

    let user_id = s.sign_in(data.username.clone(), data.password).unwrap();

    let response = SigninOutput { jwt: data.username };

    Json(response)
}

#[handler]

// Json(data) means we are destructuring like
// let u = {name: "shanu"}; let {name} = u;
// since Json is a unit struct , if we use this data: Json<CreateWebsiteInput>
// we have to access like this const url = data.0.url;
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;

    println!("URL is {}", url);

    // persist this in DB
    // sqlx => close to pg in JS
    // diesel => close to prisma
    let mut s = Store::default().unwrap();
    let website = s.create_website(String::from("1"), url).unwrap();

    let response = CreateWebsiteOutput { id: website.id };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/singup", post(sign_up))
        .at("/user/signin", post(sign_in));

    //creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
