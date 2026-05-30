use serde::{Serialize, Deserialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use chrono::{Datelike, DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike};
use icalendar::{Calendar, CalendarComponent, Component, Event, Property, EventLike, DatePerhapsTime};


#[derive(Debug, Clone)]
pub struct DateInfo {
    pub row_idx: i32,
    pub col_idx: i32,
    pub day_of_month: i32,
    pub has_event: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UICalendarEvent{
    pub uid: String,
    pub title: String,
    pub day: u8,
    pub month: u8,
    pub year: u32,
    pub begin_hour: u8,
    pub begin_mins: u8,
    pub end_hour: u8,
    pub end_mins: u8,
}

#[derive(Debug, Default)]
pub struct CalendarController {
    my_calendar: Calendar,
}

impl CalendarController {
    pub fn new() -> Self {
        CalendarController {
            my_calendar: Calendar::new(),
        }
    }

    pub fn import_calendar(&mut self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_to_string(filepath)?;
        let mut parsed_calendar: Calendar = contents.parse().map_err(|e| format!("Parsing error: {}", e))?;
        
        for component in &mut parsed_calendar.components {
            if let CalendarComponent::Event(event) = component {
                if let Some(start) = event.get_start(){
                    let naive = self.to_naive(start);
                    event.starts(naive);
                }
                if let Some(end) = event.get_end(){
                    let naive = self.to_naive(end);
                    event.ends(naive);
                }
            }
        }

        self.my_calendar = parsed_calendar;

        Ok(())
    }

    fn to_naive(&self, dt: icalendar::DatePerhapsTime) -> chrono::NaiveDateTime {
        match dt {
            icalendar::DatePerhapsTime::DateTime(cal_dt) => match cal_dt {
                icalendar::CalendarDateTime::Floating(naive) => naive,
                
                icalendar::CalendarDateTime::Utc(utc) => {
                    let local: chrono::DateTime<chrono::Local> = utc.with_timezone(&chrono::Local);
                    local.naive_local() 
                },
                
                icalendar::CalendarDateTime::WithTimezone { date_time, .. } => {
                    let local = chrono::Local.from_local_datetime(&date_time).unwrap();
                    local.naive_local()
                },
            },
            icalendar::DatePerhapsTime::Date(date) => date.and_hms_opt(0, 0, 0).unwrap(),
        }
    }

    pub fn change_event(
    &mut self, 
    uid: &str, 
    new_title: Option<String>, 
    new_date: Option<String>,
    new_begin: Option<String>, 
    new_end: Option<String>
) -> bool {
    for component in &mut self.my_calendar.components {
        if let CalendarComponent::Event(event) = component {
            if event.get_uid() == Some(uid) {
                
                // 1. Update title summary if provided
                if let Some(new_title_str) = new_title {
                    event.summary(&new_title_str);
                }

                // 2. Safely extract values by evaluating the variants of DatePerhapsTime
                let old_start_str = event.properties().iter()
                    .find(|(k, _)| k.as_str() == "DTSTART")
                    .map(|(_, v)| v.value())
                    .unwrap_or_else(|| "20260101T000000");

                let old_end_str = event.properties().iter()
                    .find(|(k, _)| k.as_str() == "DTEND")
                    .map(|(_, v)| v.value())//potencijalno .clone() 
                    .unwrap_or_else(|| "20260101T235900");

                let clean_start = old_start_str.replace('Z', "");
                let clean_end = old_end_str.replace('Z', "");

                let old_date = clean_start.get(0..8).unwrap_or("20260101");
                let old_start_time = clean_start.get(9..15).unwrap_or("000000");

                let mut final_start = clean_start.clone();
                let mut final_end = clean_end.clone();

                // 3. Decision Tree Evaluation
                match new_date {
                    Some(ref nd) => {
                        if let Some(ref ns) = new_begin {
                            final_start = format!("{}T{}", nd, ns);
                        } else {
                            final_start = format!("{}T000000", nd);
                        }

                        if let Some(ref ne) = new_end {
                            final_end = format!("{}T{}", nd, ne);
                        } else {
                            final_end = format!("{}T000000", nd);
                        }
                    },
                    None => {
                        if let Some(ref ns) = new_begin {
                            final_start = format!("{}T{}", old_date, ns);
                        }

                        if let Some(ref ne) = new_end {
                            // FIX: Added .as_str() to properly compare string types
                            if ne.as_str() > old_start_time {
                                final_end = format!("{}T{}", old_date, ne);
                            } else if let Some(ref ns) = new_begin {
                                final_end = format!("{}T{}", old_date, ns);
                            }
                        }
                    }
                }

                // 4. Update the event properties using proper borrowings (&str)
                event.add_property("DTSTART", &final_start);
                event.add_property("DTEND", &final_end);

                return true;
            }
        }
    }
    false
}

    pub fn add_event(&mut self, title: String, begin_date: String, end_date: String) -> String{
        let uid = uuid::Uuid::new_v4().to_string();

        let mut event = Event::new();
        event.summary(&title);
        event.uid(&uid);

        event.append_property(Property::new("DTSTART", &begin_date));
        event.append_property(Property::new("DTEND", &end_date));

        self.my_calendar.push(event);
        uid
    }

    pub fn get_month(&self, month: u32, year: u32) -> Vec<UICalendarEvent>{
        let needle = format!("{:02}.{}", month, year);
        let mut res = Vec::new();

        for component in &self.my_calendar.components {
            if let CalendarComponent::Event(event) = component {
                if let Some(start_dt) = event.get_start(){
                    let start_str = Self::format_dt(&start_dt);
                    if start_str.contains(&needle) {
                        let end_str = event.get_end().map(|d| Self::format_dt(&d)).unwrap_or_default();
                        
                        if let Some(event_data) = Self::parse_to_ui_event(
                            event.get_uid().unwrap_or_default(),
                            event.get_summary().unwrap_or("No name"),
                            &start_str,
                            &end_str
                        ) {
                            res.push(event_data);
                        }
                    }
                }
            }
        }
        res
    }

    pub fn get_calendar_data(&self, month: u32, year: u32) -> (Vec<UICalendarEvent>, Vec<DateInfo>) {
        let mut events = self.get_month(month, year);
        
        events.sort_by(|a, b| {
            a.day.cmp(&b.day)
                .then(a.begin_hour.cmp(&b.begin_hour))
                .then(a.begin_mins.cmp(&b.begin_mins))
        });

        let mut grid = Vec::new();
        let first_day = NaiveDate::from_ymd_opt(year as i32, month, 1).unwrap();
        let days_in_month = (NaiveDate::from_ymd_opt(year as i32, if month == 12 { 1 } else { month + 1 }, 1).unwrap() 
            - chrono::Duration::days(1)).day();

        let start_weekday = first_day.weekday().num_days_from_sunday() as i32; // Sunday je 0

        for day in 1..=days_in_month as i32 {
            let absolute_idx = day + start_weekday - 1;
            grid.push(DateInfo {
                row_idx: absolute_idx / 7,
                col_idx: absolute_idx % 7,
                day_of_month: day,
                has_event: events.iter().any(|e| e.day == day as u8),
            });
        }

        (events, grid)
    }

    pub fn delete_event(&mut self, uid: &str){
        self.my_calendar.components.retain(|component| {
            if let CalendarComponent::Event(event) = component {
                return event.get_uid() != Some(uid);
            }
            true
        });
    }

    pub fn save(&self, filepath: &str) -> std::io::Result<()>{
        let mut new_cal = Calendar::new();

        for component in &self.my_calendar.components {
            if let CalendarComponent::Event(event) = component {
                let mut new_event = Event::new();

                if let Some(uid) = event.get_uid() { new_event.uid(&uid); }
                if let Some(summary) = event.get_summary() { new_event.summary(&summary); }

                let to_utc = |naive: chrono::NaiveDateTime| {
                    chrono::Local.from_local_datetime(&naive)
                        .unwrap()
                        .with_timezone(&chrono::Utc)
                };

                if let Some(start) = event.get_start() {
                    new_event.starts(to_utc(self.to_naive(start)));
                }
                if let Some(end) = event.get_end() {
                    new_event.ends(to_utc(self.to_naive(end)));
                }
                new_cal.push(new_event);
            }
        }
        
        let mut output_file = File::create(filepath)?;
        output_file.write_all(new_cal.to_string().as_bytes())?;
        Ok(())
    }

    fn format_dt(dt: &icalendar::DatePerhapsTime) -> String {
        match dt {
            icalendar::DatePerhapsTime::DateTime(cal_dt) => {
                match cal_dt {
                    icalendar::CalendarDateTime::Floating(naive) => {
                        naive.format("%d.%m.%Y %H:%M").to_string()
                    }
                    icalendar::CalendarDateTime::Utc(utc_dt) => {
                        let local: DateTime<Local> = utc_dt.with_timezone(&Local);
                        local.format("%d.%m.%Y %H:%M").to_string()
                    }
                    icalendar::CalendarDateTime::WithTimezone { date_time, .. } => {
                        date_time.format("%d.%m.%Y %H:%M").to_string()
                    }
                }
            }
            icalendar::DatePerhapsTime::Date(date) => {
                date.format("%d.%m.%Y").to_string()
            }
        }
    }

    fn parse_to_ui_event(uid: &str, title: &str, start_str: &str, end_str: &str) -> Option<UICalendarEvent> {
        // Expected format: 20260525T201200
        let start = NaiveDateTime::parse_from_str(start_str, "%d.%m.%Y %H:%M").ok()?;
        let end = NaiveDateTime::parse_from_str(end_str, "%d.%m.%Y %H:%M").ok()?;

        Some(UICalendarEvent {
            uid: uid.to_string(),
            title: title.to_string(),
            day: start.day() as u8,
            month: start.month() as u8,
            year: start.year() as u32,
            begin_hour: start.hour() as u8,
            begin_mins: start.minute() as u8,
            end_hour: end.hour() as u8,
            end_mins: end.minute() as u8,
        })
    }
}