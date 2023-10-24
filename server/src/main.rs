use rocket::{get, routes};
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[rocket::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![index,hello])
    .launch().await?;
    Ok(())
}