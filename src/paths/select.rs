use biona_core::models::*;

//get data from the system tables
#[get("/select/<table>?<index>")]
pub fn from_table(table : String, index : u32) -> String {

    return match table.as_str() {
       "sys_company_type" => serde_json::to_string(&crate::models::sys_company_type::SysCompanyType::new().select_all(100, index)).unwrap_or_else(|_| String::new()),
       _ => String::from("")
    }

}