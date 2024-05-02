// @generated automatically by Diesel CLI.

diesel::table! {
    bookmark (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        persona_id -> Int4,
    }
}

diesel::table! {
    bookmark_tag_relation (bookmark_id, tag_id) {
        bookmark_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    history (id) {
        id -> Int4,
        title -> Text,
        favicon -> Text,
        url -> Text,
        date -> Timestamp,
        persona_id -> Int4,
    }
}

diesel::table! {
    layout (id) {
        id -> Int4,
        workspace_id -> Int4,
        position_x -> Int4,
        position_y -> Int4,
    }
}

diesel::table! {
    persona (id) {
        id -> Int4,
        name -> Text,
        profile_pic -> Text,
        color -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    preference (id) {
        id -> Int4,
        persona_id -> Int4,
    }
}

diesel::table! {
    tab (id) {
        id -> Int4,
        url -> Text,
        layout_id -> Int4,
    }
}

diesel::table! {
    tab_layout_relation (tab_id, layout_id) {
        tab_id -> Int4,
        layout_id -> Int4,
    }
}

diesel::table! {
    tag (id) {
        id -> Int4,
        tagname -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        username -> Text,
        full_name -> Text,
        email -> Text,
        password -> Text,
        profile_pic -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    workspace (id) {
        id -> Int4,
        persona_id -> Int4,
        icon -> Text,
        color -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(bookmark -> persona (persona_id));
diesel::joinable!(history -> persona (persona_id));
diesel::joinable!(layout -> workspace (workspace_id));
diesel::joinable!(persona -> user (user_id));
diesel::joinable!(preference -> persona (persona_id));
diesel::joinable!(tab -> layout (layout_id));
diesel::joinable!(workspace -> persona (persona_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookmark,
    bookmark_tag_relation,
    history,
    layout,
    persona,
    preference,
    tab,
    tab_layout_relation,
    tag,
    user,
    workspace,
);
