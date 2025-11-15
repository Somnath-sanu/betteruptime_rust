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
    let s = Arc::new(Mutex::new(
        Store::new_instance().expect("Failed to connect to database"),
    ));
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

* each clone is an independent struct and opens its own DB connection.
*Arc::clone(&arc) produces the same underlying object (same address)

*so Clone does not guarantee sharing a single connection — it depends on how Clone is implemented.
*/

/*
*Why you needed a Mutex as well as Arc — mutability rules

* Store::create_website(&mut self, ...) and get_website(&mut self, ...) take &mut self. To call them you need exclusive mutable access to the Store object.
*
* Arc<T> gives shared ownership but only allows read-only shared references. You cannot get a &mut T from Arc<T>.
*
* Arc<Mutex<T>> allows multiple owners and provides a MutexGuard that gives you a &mut T (exclusive) so you can call &mut methods.
*
* So Arc + Mutex perfectly matches the need: many owners, but one mutable use at a time.
*/

/*
 * tokio::main(flavor = "multi_thread") — why multi_thread?
 *
 * tokio has two runtime flavors:
 *      current_thread (single-threaded runtime) — runs all tasks on the    current thread.
 *   multi_thread — runs a work-stealing thread pool so multiple tasks run in parallel on different OS threads.
 *
 *
*/
