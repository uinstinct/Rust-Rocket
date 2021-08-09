#[macro_use]
extern crate rocket;

use rocket::response::content::Json;
use rocket::response::status;

#[get("/")]
fn index() -> &'static str {
    "Master Blaster"
}

#[get("/<id>")]
fn read(id: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: {}", id)))
}

// struct Message<'r> {
//     id: Option<i64>,
//     message: &'r str,
// }

#[get("/j")]
fn json() -> Json<&'static str> {
    Json(
        "{
        'status': 'success',
        'message': 'Hello API!'
      }",
    )
}

// requires a content-type application/json header
#[post("/", format = "json")]
fn posting() -> Json<&'static str> {
    Json("doing")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, read, json, posting])
}
