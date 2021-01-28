table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        type_id -> Int4,
        is_partner -> Bool,
        notes -> Nullable<Varchar>,
    }
}

table! {
    contacts (id) {
        id -> Int4,
        person_id -> Nullable<Int4>,
        company_id -> Nullable<Int4>,
        line1 -> Nullable<Varchar>,
        line2 -> Nullable<Varchar>,
        line3 -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
    }
}

table! {
    people (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        is_employee -> Bool,
        notes -> Nullable<Varchar>,
    }
}

table! {
    sys_company_type (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    tmp (id) {
        id -> Int4,
        tmp_date -> Nullable<Date>,
        name -> Nullable<Varchar>,
    }
}

joinable!(companies -> sys_company_type (type_id));
joinable!(contacts -> companies (company_id));
joinable!(contacts -> people (person_id));

allow_tables_to_appear_in_same_query!(
    companies,
    contacts,
    people,
    sys_company_type,
    tmp,
);
