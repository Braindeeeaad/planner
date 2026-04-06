//use web_sys::HtmlCollection;
use yew::prelude::*; 
use std::time::SystemTime;
use yew::{MouseEvent};
use log::info;


use wasm_bindgen::JsCast;

pub struct GridDims{
    pub x: u32, 
    pub y: u32, 
    pub width: u32, 
    pub length: u32
}

#[derive(Properties, PartialEq)]
pub struct CalendarProps{
    //pub tasks:Vec<Task>,
     
    pub onmousedown: Callback<(MouseEvent,usize)>,
    pub onmouseup: Callback<(MouseEvent, usize)>
}
#[function_component(Calendar)] 
pub fn calendar(props: &CalendarProps) -> Html {
    let days = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    //let tasks:Vec<Task> = Vec::new()

    html! { 
        
        <div class="calendar-container"> 
            // 1. Headers Row (Floating Glassmorphism style)
            <div class="header-grid">
                <div class="time-spacer">
                </div>
                {
                    days.iter().enumerate().map(|(index, day)| html!{
                        <div class="day-header"> 
                            <div class="day-name">{day}</div>
                            <div class={if index == 0 { "day-number active" } else { "day-number" }}>
                                { (29 + index) }
                            </div>
                        </div>
                    }).collect::<Html>()
                }
            </div>
            
            // 2. Main Grid
            <div class="calendar-grid">
                // Left column: 12 AM - 11 PM
                <div class="time-labels-column">
                    {
                        (0..24).map(|hour| {
                            let period = if hour < 12 { "AM" } else { "PM" };
                            let display_hour = if hour % 12 == 0 { 12 } else { hour % 12 };
                            html! {
                                <div class="time-label">
                                    <span>{ format!("{} {}", display_hour, period) }</span>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
        
                // 7 vertical day columns
                {
                    days.iter().enumerate().map(|(day_index, _day)| html! {
                        <div class="day-column">
                            
                            // Let's add the Glowing Current Time Line only on the first day (Sunday)
                            if day_index == 0 {
                                <div class="current-time-line" style="top: 85px;">
                                    <div class="current-time-dot"></div>
                                </div>
                            }

                            {
                                (0..24).map(|hour| html! {
                                    <div class="grid-cell" id={hour.to_string()} ></div>
                                }).collect::<Html>()
                            }
                        </div>
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}