use biona_core::biona_macros::TableTrait;
use serde::{Deserialize, Serialize};

#[derive(TableTrait, Default, Debug, Serialize, Deserialize)]
#[table_name(companies)]
#[tables_join(companies::table.inner_join(sys_company_type::table))]
pub struct Companies {

    #[updatable(false)]
    #[insertable(false)]
    #[primary_key(true)]
    #[hidden(true)]
    #[priority(4)]
    #[display_name("ID")]
    pub id : i32,

    #[priority(3)]
    #[display_name("Company Name")]
    pub name : String,

    #[field_schema(sys_company_type::name)]
    #[updatable(false)]
    #[insertable(false)]
    #[display_name("Type")]
    #[priority(1)]
    #[from_system_table(true)]
    pub type_name : String,

    #[selectable(false)]
    pub type_id : i32,

    #[priority(2)]
    #[display_name("Partner")]
    pub is_partner : bool,

    #[display_name("Notes")]
    #[sortable(false)]
    pub notes : Option<String>,

}