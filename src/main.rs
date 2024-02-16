#[macro_use]
extern crate rocket;

#[get("/jobs")]
fn jobs() -> &'static str {
    "all jobs"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![jobs])
}
