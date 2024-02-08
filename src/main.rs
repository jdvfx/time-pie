#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

#[derive (Debug)]
struct Task<'a>{
    time_start:&'a str,
    time_end:&'a str,
    mins_start:u32,
    mins_end:u32,
    name:&'a str,
}

impl<'a> Task<'a>{
    fn new(name:&'a str,time_start:&'a str,time_end:&'a str ) -> Self{
        let mins_start:u32 = time_to_mins(&time_start);
        let mins_end:u32 = time_to_mins(&time_end);
        Task{
            time_start,
            time_end,
            mins_start,
            mins_end,
            name,
        }
    }
}


fn time_to_mins(s:&str) -> u32{
    let s_:Vec<_> = s.split(":").collect();
    let h=s_.get(0).unwrap().parse::<u32>().unwrap()*60;
    let m=s_.get(1).unwrap().parse::<u32>().unwrap();
    return h+m
}

fn mins_to_time(mins:u32) -> String{
    let hours:u32 = (mins as f64/60.0).floor() as u32;
    let mins:u32 = mins - (hours*60); 
    format!("{hours}:{mins}")
}


fn main() {

    let mut tasks:Vec<Task>= Vec::new();
    let task1 = Task::new("sleep", "23:00","24:00");
    let task2 = Task::new("learn rust", "21:00","22:00");
    tasks.push(task1);
    tasks.push(task2);

    let total_time_in_day = "24:00";
    let total_mins = time_to_mins(&total_time_in_day);
    println!("total minutes in day:{}",&total_mins);

    for task in tasks{
        println!("{:?}",task);


        let duration = task.mins_end.abs_diff(task.mins_start);
        println!("duration:{} mins",duration);
    }

}
