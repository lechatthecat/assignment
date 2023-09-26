use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RestaurantTable {
   pub id: i32,
   pub table_number: i32,
   pub note: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestaurantTableOrder {
   pub id: i32,
   pub table_number: i32,
   pub table_note: Option<String>,
   pub menu_name: Option<String>,
   pub price: Option<i32>,
   pub cook_time_seconds: Option<i32>,
   pub order_id: Option<i64>,
   pub expected_cook_finish_time: Option<NaiveDateTime>,
   pub ordered_time: Option<NaiveDateTime>,
   pub is_served_by_staff: Option<bool>,
   pub served_by_user_id: Option<i32>,
   pub serve_staff_name: Option<String>,
   pub checked_by_user_id: Option<i32>,
   pub check_staff_name: Option<String>,
}
