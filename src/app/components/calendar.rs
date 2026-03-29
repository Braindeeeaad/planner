//use web_sys::HtmlCollection;
use yew::prelude::*; 


#[function_component(Calendar)] 
pub fn calendar() -> Html {
    let days = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

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
                                (0..24).map(|_hour| html! {
                                    <div class="grid-cell"></div>
                                }).collect::<Html>()
                            }
                        </div>
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}