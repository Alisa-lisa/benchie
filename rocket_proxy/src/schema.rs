table! {
    past_destinations (id) {
        id -> Int4,
        passenger -> Int4,
        city -> Varchar,
        street -> Nullable<Varchar>,
        street_number -> Nullable<Varchar>,
        times_visited -> Nullable<Int4>,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    past_destinations,
    spatial_ref_sys,
);
