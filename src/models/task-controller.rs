use serde::{Serialize, Deserialize};
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::collections::BTreeMap;
use uuid::Uuid;
use chrono::{Duration, NaiveDate, NaiveTime, Utc};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Task {
    pub id: u32, //interni id
    pub title: String,
    pub status: u8, //0 DNS, 1 In Progress, 2 Done
    pub priority: u8,
    pub total_time: String, // 000:00:00 ddd:hh:mm
    pub deadline: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TaskController {
    pub next id: u32, //individual tabs of tasks
    pub tasks: Vec<Task>,
}

impl TaskController{
    pub fn new() -> Self {
        next_id: 1,
        tasks: Vec::new(),
    }
    pub fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_task(&mut self, priority: Option<u8>, title: &str, deadline: Option<&str>) -> bool {
        let mut tmp = Task{
            id: self.get_next_id(),
            status: 0,
            times: BTreeMap<String, String>::new();
            total_time: "000:00:00",
            begin_date: "",
            deadline: Some(deadline),
            priority: Some(priority),
        };
        tasks.push(tmp);
        true
    }

    pub fn change_task(&mut self, id: u32, priority: Option<u8>, title: Option<String>, deadline: Option<String>){
        for x in &mut self.tasks {
            if((*x).id == id){
                if(Some(priority)) (*x).priority = priority;
                if(Some(title)) (*x).title = title;
                if(Some(deadline)) (*x).priority = deadline;
                break;
            }
        }
    }
    pub fn finish_task(&mut self, id: u32){
        for x in &mut self.tasks {
            if((*x).id == id){
                (*x).status = 2;
                break;
            }
        }
    }
    pub fn add_time(&mut self, id: u32, time: &str){ //time: hh:mm
        for x in &mut self.tasks {
            if((*x).id == id){
                let mut m1 = (*x).total_time.chars().nth(8).to_digit(10).unwrap()
                    + (*x).total_time.chars().nth(7).to_digit(10).unwrap() * 10;
                let mut m2 = time.chars().nth(4).to_digit(10).unwrap()
                    + time.chars().nth(3).to_digit(10).unwrap() * 10;

                let mut h1 = (*x).total_time.chars().nth(5).to_digit(10).unwrap()
                    + (*x).total_time.chars().nth(4).to_digit(10).unwrap() * 10;
                let mut h2 =  time.chars().nth(1).to_digit(10).unwrap()
                    + time.chars().nth(0).to_digit(10).unwrap() * 10;

                let mut d1 = (*x).total_time.chars().nth(2).to_digit(10).unwrap()
                    + (*x).total_time.chars().nth(1).to_digit(10).unwrap()*10
                    + (*x).total_time.chars().nth(0).to_digit(10).unwrap()*100;

                m1 += m2;
                if(m1 >= 60){
                    m1 -= 60;
                    h1 += 1;
                }
                h1 += h2;
                if(h1 >= 24){
                    d1 += 1;
                    h1 -= 24;
                }
                (*x).total_time = format!("{:03}:{:02}:{:02}", d1, h1, m1);
            }
        }
    }

    pub fn get_tasks(sortby: u8) -> Vec<Task>{ //1 names asc, 2 priority desc, 3 deadline earliest first
        if(sortby == 1){
            self.tasks.sort_by_key(|t| t.title.clone());
        }else if(sortby == 2){
            //add for 2 and 3 to leave the finished ones at the end
            self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
        }else if(sortby == 3){
            self.tasks.sort_by_key(|t| t.deadline.clone());
        }
        self.tasks
    }
    
}
use serde::{Serialize, Deserialize};
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::collections::BTreeMap;
use uuid::Uuid;
use chrono::{Duration, NaiveDate, NaiveTime, Utc};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Task {
    pub id: u32, //interni id
    pub title: String,
    pub status: u8, //0 DNS, 1 In Progress, 2 Done
    pub priority: u8,
    pub total_time: String, // 000:00:00 ddd:hh:mm
    pub deadline: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TaskController {
    pub next id: u32, //individual tabs of tasks
    pub tasks: Vec<Task>,
}

impl TaskController{
    pub fn new() -> Self {
        next_id: 1,
        tasks: Vec::new(),
    }
    pub fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_task(&mut self, priority: Option<u8>, title: &str, deadline: Option<&str>) -> bool {
        let mut tmp = Task{
            id: self.get_next_id(),
            status: 0,
            times: BTreeMap<String, String>::new();
            total_time: "000:00:00",
            begin_date: "",
            deadline: Some(deadline),
            priority: Some(priority),
        };
        tasks.push(tmp);
        true
    }

    pub fn change_task(&mut self, id: u32, priority: Option<u8>, title: Option<String>, deadline: Option<String>){
        for x in &mut self.tasks {
            if((*x).id == id){
                if(Some(priority)) (*x).priority = priority;
                if(Some(title)) (*x).title = title;
                if(Some(deadline)) (*x).priority = deadline;
                break;
            }
        }
    }
    pub fn finish_task(&mut self, id: u32){
        for x in &mut self.tasks {
            if((*x).id == id){
                (*x).status = 2;
                break;
            }
        }
    }
    pub fn add_time(&mut self, id: u32, time: &str){ //time: hh:mm
        for x in &mut self.tasks {
            if((*x).id == id){
                let mut m1 = (*x).total_time.chars().nth(8).to_digit(10).unwrap()
                    + (*x).total_time.chars().nth(7).to_digit(10).unwrap() * 10;
                let mut m2 = time.chars().nth(4).to_digit(10).unwrap()
                    + time.chars().nth(3).to_digit(10).unwrap() * 10;

                let mut h1 = (*x).total_time.chars().nth(5).to_digit(10).unwrap()
                    + (*x).total_time.chars().nth(4).to_digit(10).unwrap() * 10;
                let mut h2 =  time.chars().nth(1).to_digit(10).unwrap()
                    + time.chars().nth(0).to_digit(10).unwrap() * 10;

                let mut d1 = (*x).total_time.chars().nth(2).to_digit(10).unwrap()
                    + (*x).total_time.chars().nth(1).to_digit(10).unwrap()*10
                    + (*x).total_time.chars().nth(0).to_digit(10).unwrap()*100;

                m1 += m2;
                if(m1 >= 60){
                    m1 -= 60;
                    h1 += 1;
                }
                h1 += h2;
                if(h1 >= 24){
                    d1 += 1;
                    h1 -= 24;
                }
                (*x).total_time = format!("{:03}:{:02}:{:02}", d1, h1, m1);
            }
        }
    }

    pub fn get_tasks(sortby: u8) -> Vec<Task>{ //1 names asc, 2 priority desc, 3 deadline earliest first
        if(sortby == 1){
            self.tasks.sort_by_key(|t| t.title.clone());
        }else if(sortby == 2){
            //add for 2 and 3 to leave the finished ones at the end
            self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
        }else if(sortby == 3){
            self.tasks.sort_by_key(|t| t.deadline.clone());
        }
        self.tasks
    }
    
}