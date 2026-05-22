use serde::{Serialize, Deserialize};
use icalendar::{Calendar, CalendarDateTime, Class, Component, Event, EventLike, Property, Todo};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::collections::BTreeMap;
use uuid::Uuid;
use chrono::{Duration, NaiveDate, NaiveTime, Utc};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CalendarEvent {
    pub uid: String, //id u ics formatu
    pub created: String, //CREATED:20151219T021727Z 2015. dec. 19, 02:17:27 UTC
    pub title: String,
    pub begin_date: String,
    pub end_date: String,
}

pub struct CalendarController {
    my_calendar: Calendar,
    events: BTreeMap<String, CalendarEvent>, //begin date, calendarevent (proveriti kako se redja)
}

impl CalendarController {
    pub fn new() -> Self {
        CalendarController {
            my_calendar: Calendar::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn get_calendar(&mut self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_to_string(filepath)?;
        let parsed_calendar: Calendar = contents.parse().map_err(|e| format!("Parsing error: {}", e))?;
        self.my_calendar = parsed_calendar.clone();

        for event in parsed_calendar.events(){
            let mut tmp: CalendarEvent::default();
            tmp.id = self.get_next_id();

            for property in event.properties() {
                match property.name().as_str() {
                    "CREATED" => tmp.created = property.value().to_string(),
                    "UID" => tmp.uid = property.value().to_string(),
                    "SUMMARY" => tmp.title = property.value().to_string(),
                    "DTSTART" => tmp.begin_date = property.value().to_string(),
                    "DTEND"   => tmp.end_date = property.value().to_string(),
                    _ => {}
                }
            }

            self.events.insert(tmp.begin_date, tmp);
        }
        
        Ok(())
    }
    pub fn change_event(&mut self, uid: &str, new_title: Option<String>, new_begin: Option<String>, new_end: Option<String>) -> bool{
        let mut tmp;
        for component in &mut self.my_calendar.components {
            if let CalendarComponent::Event(event) = component {
                if event.get_uid() == uid {
                    tmp = event.get_created();
                    // etc
                    if(Some(new_title)) event.summary(new_title);
                }
            }
        }
        self.events[tmp].title = new_title; //for example
        true
    }

    pub fn add_event(&mut self, title: String, begin_date: String, end_date: String) -> {
        let uid = Uuid::new_v4().to_string();
        let utc_now = UTC::now();
        let utc_now_formatted = utc_now.format("$Y$m$dT$H$M$SZ").to_string();

        let event = Event::new()
            .summary(ime)
            .starts(beg_time)
            .uid(&uid)
            .created(utc_now_formatted.clone())
            .done();

        self.my_calendar.push(event);
        
        let my_event = CalendarEvent {
            uid: uid.clone(),
            created: utc_now_formatted.clone(),
            title: title,
            begin_date: begin_date,
            end_date: end_date,
        }
        
        self.events.insert(utc_now_formatted.clone(), my_event);
    }

    pub fn delete_event(&mut self, uid: &str){
        //verovatno bi bilo bolje napraviti novi kalendar umesto ovoga primarno zbog exporta
        let mut tmp;
        for component in &mut self.my_calendar.components {
            if let CalendarComponent::Event(event) = component {
                if event.get_uid() == uid {
                    tmp = event.get_created();
                    event.status(EventStatus::Cancelled);
                }
            }
        }
        self.events.remove(tmp); 
        true
    }

    pub fn export_calendar(&self, filepath: &str) -> std::io::Result<()>{
        let ics_string = self.my_calendar.to_string();
        let mut output_file = File::create(filepath)?;
        output_file.write_all(ics_string.as_bytes())?;
        Ok(())
        //ili calendar.generate()? proveriti da li rade isto
    }

}
