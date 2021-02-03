use biona_core::biona_macros::TableTrait;
use biona_core::models::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(TableTrait, Default, Debug, Deserialize, Serialize)]
#[table_name(tmp)]
pub struct Tmp {

    #[updatable(false)]
    #[insertable(false)]
    #[primary_key(true)]
    pub id : i32,

    #[display_name("Date")]
    pub tmp_date : Option <NaiveDate>,

    #[display_name("Name")]
    pub name : Option <String>
}