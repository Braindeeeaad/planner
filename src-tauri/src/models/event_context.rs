use sqlx::FromRow; 



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
    start_location_id: Optional<String>,
    end_location_id: Optional<String>, 
    recurring_rule: Optional<String> //-e.f "FREQ=WEEKLY;BYDAY=MO,WE,FR" 
}
