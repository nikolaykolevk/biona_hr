use biona_core::biona_macros::TableTrait;

#[derive(TableTrait, Default, Debug)]
#[table_name(sys_company_type)]
pub struct SysCompanyType {

    #[updatable(false)]
    #[insertable(false)]
    #[primary_key(true)]
    pub id : i32,

    pub name : String,
}