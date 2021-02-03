use crate::models::*;

//use biona macro to generate responder with name table_data for the tables
generate_table_page_and_table_data!(
    crate::meta(),
    "companies", Companies, "Companies Table",
    "tmp", Tmp, "TMP Table"
);
