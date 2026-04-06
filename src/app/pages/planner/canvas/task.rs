use yew::prelude::*; 
use std::time::SystemTime;

#[derive(PartialEq)]
struct Task{
    name: String, 
    start: SystemTime, 
    duration: i32
}



#[derive(Properties,PartialEq)]
pub struct TaskCardProp{
    task: Task
}

#[function_component(TaskCard)]
pub fn task_card(props:&TaskCardProp)->Html{
    html!{}
}