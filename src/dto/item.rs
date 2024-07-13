pub mod item {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Item {
        pub done: bool,
        pub description: String,
    }
}
