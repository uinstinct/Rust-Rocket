#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use rocket::response::Debug;
use rocket::tokio::fs::File;

mod paste_id;
use paste_id::PasteId;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

/* read how to do async tasks in BACKGROUND
async fn a1() {
    println!("do job 1");
}
async fn a2() {
    println!("do job 1");
}*/

#[get("/<id>")]
async fn retreive(id: &str) -> Option<File> {
    let file_name = format!("upload/{id}", id = id);
    File::open(&file_name).await.ok()
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> Result<String, Debug<std::io::Error>> {
    let id = PasteId::new(3);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:9000", id = id);

    paste.open(128.kibibytes()).into_file(filename).await?;
    Ok(url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload, retreive])
}
