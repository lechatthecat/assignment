use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: i64,
    pub restaurant_table_id: i32,
    pub menu_id: i32,
    pub expected_cook_finish_time: NaiveDateTime,
    pub is_served_by_staff: bool,
    pub price: i32,
    pub checked_by_user_id: i32,
}
