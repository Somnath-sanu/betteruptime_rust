use std::env;

use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, Validation, decode};
use poem::{Error, FromRequest, Request, RequestBody, Result, http::StatusCode};

use crate::routes::user::Claims;

pub struct UserId(pub String); // unit struct since it only holds one thing
/*
 Otherwise we can do something like this
 struct UserId {
   id: String
 }
*/

// Implemented a trait on a struct which uses generic
impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let token = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            /*
            ok_or_else : 
              If Option is Some(v) → return Ok(v)
              If Option is None → call the closure || ... → produce an error

              Transforms the Option<T> into a Result<T, E>, mapping [Some(v)] to [Ok(v)] and None to [Err(err())].
             */
            .ok_or_else(|| Error::from_string("missing token", StatusCode::UNAUTHORIZED))?;

        // now since token is a jwt
        // we need to verify the token and extract the user_id out of it

        dotenv().ok();

        let secret = env::var("JWT_SECRET")
            .map_err(|_| Error::from_string("Missing jwt secret", StatusCode::NOT_FOUND))?;
           // "?" is like throw the error in JS

        let key = DecodingKey::from_secret(secret.as_ref());

        let token_data = decode::<Claims>(&token, &key, &Validation::default())
            .map_err(|_| Error::from_string("Missing token", StatusCode::UNAUTHORIZED))?;

        /*
        Used map error because its returing jsonwebtoken Error but we are expecting Poem Error
         */

        Ok(UserId(token_data.claims.sub))
    }
}

/*
Future is similar to promise

you either write async func that returns Result(no need no do returns Promise like in JS async func returns Promise)
  async fn from_request(
      req: &'a Request,
      body: &mut RequestBody,
  ) -> Result<Self>;

   OR

  if u dont want to put async

  fn from_request(
      req: &'a Request,
      body: &mut RequestBody,
  ) -> impl Future<Output = Result<Self>> + Send;

   * func that implements future
*/
