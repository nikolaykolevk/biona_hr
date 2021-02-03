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

pub fn meta() -> biona_core::models::Meta {
    biona_core::models::Meta {
        app_name: "Human Resources".to_string(),
        tables: vec![
            biona_core::models::TableMeta {
                name: "Companies".to_string(),
                link: "companies".to_string()
            }, biona_core::models::TableMeta {
                name: "Tmp".to_string(),
                link: "tmp".to_string()
            }
        ]
    }
}

fn main() {

    // models::Tmp::new().update_from_json("{\"id\":3,\"tmp_date\":\"2011-01-29\",\"name\":\"nikolay\"}");


    //launch rocket with the predefined routes from biona core
    paths::rocket()
        .mount("/table", routes![paths::tables::table_page, paths::tables::table_data, paths::tables::table_component])
        .mount("/table", routes![paths::update::table_update_page, paths::update::table_update_component])
        .mount("/", routes![paths::select::from_table])
        .launch();

}