pub mod item {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Item {
        pub id: i64,
        pub done: bool,
        pub description: String,
    }
}
