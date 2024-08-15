use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Menu {
    pub id: i32,
    pub name: String,
    pub cook_time_seconds: i32,
    pub price: i32,
}
