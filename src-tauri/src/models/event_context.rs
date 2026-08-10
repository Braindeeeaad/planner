//use sqlx::FromRow;
use crate::db::connection::{Saved,New}; 
use sqlx::sqlite::{SqlitePool};
use sqlx::{FromRow, Row, sqlite::SqliteRow};
use std::marker::PhantomData;
use uuid::Uuid;


pub struct Location<State=New>{
    id: String, 
    name: String, 
    address: String, 
    latitude: f64, 
    longitude: f64,

    _state: std::marker::PhantomData<State>
}

impl<'r, State> FromRow<'r, SqliteRow> for Location<State> {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            address: row.try_get("address")?,
            latitude: row.try_get("latitude")?,
            longitude: row.try_get("longitude")?,
            _state: PhantomData,
        })
    }
}
impl Location<New> {
    pub fn new(
        name: String,
        address: String,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            address,
            latitude,
            longitude,
            _state: PhantomData,
        }
    }
}

pub async fn upload_location(
    location:Location,
    pool:&SqlitePool
)->anyhow::Result<Location<Saved>>{

        let query = "INSERT INTO locations
                           (id,name,address,latitude,longitude)
                           VALUES $1,$2,$3,$4,$5";
        
        sqlx::query(query)
            .bind(&location.id)
            .bind(&location.name)
            .bind(&location.address)
            .bind(&location.latitude)
            .bind(&location.longitude)
            .execute(pool)
            .await?;

        Ok(Location{
                id:location.id, 
                name:location.name, 
                address:location.address, 
                latitude:location.latitude, 
                longitude:location.longitude,
                _state:std::marker::PhantomData
            }
        )
        

    }


pub async fn get_locations(pool:&SqlitePool)->anyhow::Result<Vec<Location<Saved>>>{
    let locations = sqlx::query_as::<_,Location<Saved>>( 
        r#"SELECT id,name,address,latitude,longitude FROM locations"#
    ).fetch_all(pool) 
    .await?;
    Ok(locations) 
}



pub struct EventContext<State = New>{
    id: String, 
    title: String, 
    category: String ,
    start_location_id: Option<String>,
    end_location_id: Option<String>, 
    recurring_rule: Option<String>, //-e.f "FREQ=WEEKLY;BYDAY=MO,WE,FR"
    _state: std::marker::PhantomData<State>, 
}

impl<'r, State> FromRow<'r, SqliteRow> for EventContext<State> {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            category: row.try_get("category")?,
            start_location_id: row.try_get("start_location_id")?,
            end_location_id: row.try_get("end_location_id")?,
            recurring_rule: row.try_get("recurring_rule")?,
            _state: PhantomData,
        })
    }
}
impl EventContext<New> {
    pub fn new(
        title: String,
        category: String,
        start_location_id: Option<String>,
        end_location_id: Option<String>,
        recurring_rule: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            category,
            start_location_id,
            end_location_id,
            recurring_rule,
            _state: PhantomData,
        }
    }
}


pub async fn upload_event_context(
    context:EventContext<New>, 
    pool:&SqlitePool
)->anyhow::Result<EventContext<Saved>>{
    
    let query = "INSERT INTO event_contexts 
                       (id,title,category,start_location_id,end_location_id,recurring_rule) 
                       VALUES ($1,$2,$3,$4,$5,$6)"; 
    sqlx::query(query)
        .bind(&context.id)
        .bind(&context.title)
        .bind(&context.category)
        .bind(&context.start_location_id)
        .bind(&context.end_location_id)
        .bind(&context.recurring_rule)
        .execute(pool)
        .await?;

    
    Ok(
        EventContext{
            id:context.id, 
            title:context.title, 
            category:context.category, 
            start_location_id:context.start_location_id, 
            end_location_id:context.end_location_id, 
            recurring_rule:context.recurring_rule, 
            _state:std::marker::PhantomData
        }
    )
}


pub async fn get_event_contexts(pool: &SqlitePool)-> anyhow::Result<Vec<EventContext<Saved>>>{
    let event_contexts = sqlx::query_as::<_,EventContext<Saved>>(
        r#"SELECT id,title,category,start_location_id,end_location_id,recurring_rule FROM event_contexts"#
    ).fetch_all(pool) 
    .await?;
    Ok(event_contexts)
}