#[macro_use]
extern crate rocket; 

pub mod teams;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![

        ]
    )
}