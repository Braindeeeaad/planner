use std::f64::NAN;

use yew::prelude::*; 
use yew::MouseEvent;
use log::info;
use wasm_bindgen::*;

use crate::app::pages::planner::calendar::calendar::{Calendar};
 

struct Point{
    x:Option<f64>, 
    y:Option<f64> 
}
pub struct GridDims{
    pub x: f64, 
    pub y: f64, 
    pub width: f64, 
    pub height: f64
}

/* 

    Need canvas component to help draw tasks onto calendar

*/
#[function_component(Canvas)]
pub fn canvas() -> Html{
    let mouse_down_point = use_state(|| Point{x:None,y:None});
    let mouse_up_point = use_state(|| Point{x:None,y:None});
    let current_day_idx = use_state(|| 0);
    let current_grid_dim = use_state(|| GridDims{x:0.0,y:0.0,width:0.0,height:0.0});

    

    //Abuse event bubbling, but use gaurd clauses to only care about clicks on grids
    let on_mouse_click = {

        let mouse_down_point = mouse_down_point.clone();
        Callback::from( move |event: MouseEvent|{
            if let Some(target) = event.target() {
                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>(){

                        if !element.class_name().contains("grid-cell"){
                            return
                        }

                        //Grid cell clicked
                        let grid_id_str = element.id();
                        let grid_idx:u32 = grid_id_str.parse().unwrap();
                        let width = element.offset_width() as f64; 
                        let height = element.offset_height() as f64; 

                        let rect = element.get_bounding_client_rect(); 
                        let rect_x = rect.x(); 
                        let rect_y = rect.y();

                        mouse_down_point.set(Point{x:Some(rect_x),y:Some(rect_y)});
                        current_day_idx.set(grid_idx);
                        current_grid_dim.set(GridDims { x: (rect_x), y: (rect_y), width: (width), height: (height) })
                        

                }
            }
        })
    };
    
    html!{

    }

} 
