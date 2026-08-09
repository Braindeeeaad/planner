use sqlx::FromRow; 
use crate::db::connection::establish_connection;

#[derive(FromRow)] 
pub struct locations{
    id: String, 
    name: String, 
    address: String, 
    latitude: f32, 
    longitude: f32 
}


#[derive(FromRow)] 
pub struct event_context{
    id: String, 
    title: String, 
    category: String ,
    start_location_id: Option<String>,
    end_location_id: Option<String>, 
    recurring_rule: Option<String> //-e.f "FREQ=WEEKLY;BYDAY=MO,WE,FR" 
}


pub fn create_location(){}


pub fn get_locations(){}


pub fn create_event_context(){}


pub fn get_event_contexts(){}