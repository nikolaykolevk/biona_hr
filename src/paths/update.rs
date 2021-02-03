use crate::models::*;

// use biona macro to generate responder with name table_update_page for updating the rows of the table
generate_row_update!(
    crate::meta(),
    "companies", Companies,
    "tmp", Tmp
);
