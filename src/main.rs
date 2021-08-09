#[macro_use] extern crate rocket;

#[get("/")]
fn index() ->&'static str {
    "Master Blaster"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
