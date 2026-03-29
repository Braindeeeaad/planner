//use web_sys::HtmlCollection;
use yew::prelude::*; 


#[function_component(Calendar)] 
pub fn calendar() -> Html {
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    html! { 
        

        <div class="calendar-container"> 
            
            //Generates (Monday-Tuesday-etc) header
            <div class="header-grid">
                
                <div class="time-spacer"></div>
                {
                    days.iter().map(|day| html!{
                        <div class="day-header"> 
                            {day}
                        </div>
                    }).collect::<Html>()
                }
            </div>
            

            <div class="calendar-grid">
                
                //Makes the 12 AM - 11PM column
                <div class="time-labels-column">
                    {
                        (0..24).map(|hour| {
                            let period = if hour < 12 { "AM" } else { "PM" };
                            let display_hour = if hour % 12 == 0 { 12 } else { hour % 12 };
                            html! {
                                <div class="time-label">
                                    { format!("{}:00 {}", display_hour, period) }
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                
                //Makes the rest of the column grids
                {
                    days.iter().map(|_day| html! {
                        <div class="day-column">
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
