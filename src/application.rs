pub mod adapters {
    pub mod console {
        pub mod actions {
            pub mod find_poem_by_author_id;
            pub mod find_poem_by_id;
            pub mod find_poem_by_title;
        }
        pub mod console_app;
        pub mod types;
    }
}

pub mod interfaces {
    pub mod single_action_controller;
}