#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate biona_core;
extern crate serde_json;


pub mod schema;
pub mod models;
pub mod paths;

fn main() {

    //launch rocket with the predefined routes from biona core
    biona_core::rocket()
        .mount("/table", routes![paths::tables::table_page, paths::tables::table_data, paths::update::table_update_page])
        .mount("/", routes![paths::select::from_table])
        .launch();

}