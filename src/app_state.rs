use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime, Local, TimeZone, Utc, Datelike};
use uuid::Uuid;

use crate::calendar_controller::{CalendarController, UICalendarEvent, DateInfo};
use crate::task_controller::{TaskController, Task};
use crate::notes_controller::{NotesController, Note, Category};

#[derive(Debug, Default)]
pub struct AppState {
    pub calendar_controller: CalendarController,    
    pub task_controller: TaskController,
    pub notes_controller: NotesController,
    folder_name: String,
}

impl AppState {
    pub fn new(folder: Option<&str>) -> Self {
        AppState {
            calendar_controller: CalendarController::new(),
            task_controller: TaskController::new(),
            notes_controller: NotesController::new(),
            folder_name: folder.unwrap_or(if cfg!(target_os = "windows") {
                "C:/Users/Public"
            } else {
                "~"
            }).to_string(),
        }
    }

    pub fn import_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let calendar_path = format!("{}/calendar.ics", self.folder_name);
        let tasks_path = format!("{}/tasks.json", self.folder_name);
        let notes_path = format!("{}/notes.json", self.folder_name);
        let ctg_path = format!("{}/categories.json", self.folder_name);
        
        if std::path::Path::new(&calendar_path).exists() {
            if let Err(e) = self.calendar_controller.import_calendar(&calendar_path){
                eprintln!("Error loading: {}", e);
            }
        }
        if std::path::Path::new(&tasks_path).exists() {
            if let Err(e) = self.task_controller.import_tasks(&tasks_path) {
                eprintln!("Error loading: {}", e);
            }
        }
        if std::path::Path::new(&notes_path).exists() && std::path::Path::new(&ctg_path).exists() {
            if let Err(e) = self.notes_controller.import_notes(&format!("{}", self.folder_name)) {
                eprintln!("Error loading: {}", e);
            }
        }

        Ok(())
    }

    pub fn save_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.task_controller.save(&format!("{}/tasks.json", self.folder_name))?;
        self.calendar_controller.save(&format!("{}/calendar.ics", self.folder_name))?;
        self.notes_controller.save(&format!("{}", self.folder_name))?;
        Ok(())
    }

    pub fn modify_tasks(&mut self, id: u32, new_priority: Option<u8>, new_title: Option<&str>, new_deadline: Option<&str>) -> bool {
        if id == 0 {
            let title = new_title.unwrap_or("New Task");
            let priority = new_priority.unwrap_or(1);
            self.task_controller.add_task(priority, title, new_deadline)
        }else{
            if id >= self.task_controller.next_id { return false; }
            self.task_controller.change_task(id, new_priority, new_title.map(String::from), new_deadline.map(String::from))
        }
    }

    pub fn add_time(&mut self, id: u32, hours: u8, mins: u8){
        self.task_controller.add_time(id, hours, mins);
        let _ = self.save_data();
    }

    pub fn finish_task(&mut self, id: u32) -> bool{
        self.task_controller.finish_task(id);
        true
    }

    pub fn get_tasks(&mut self, sort_by: u8) -> Vec<Task> {
        self.task_controller.get_tasks(sort_by)
    }

    pub fn delete_task(&mut self, id: u32) -> bool {
        self.task_controller.delete_task(id);
        true
    }

    pub fn add_event( &mut self, title: &str, day: u8, month: u8, year: u32,
        begin_hour: u8, begin_mins: u8, end_hour: u8, end_mins: u8) -> String { // ovo je time 20250524T100000

        let start = format!("{:4}{:02}{:02}T{:02}{:02}00", year, month, day, begin_hour, begin_mins);
        let end = format!("{:4}{:02}{:02}T{:02}{:02}00", year, month, day, end_hour, end_mins);

        let uid = self.calendar_controller.add_event(title.to_string(),
            start, end);
        
        let _ = self.calendar_controller.save(&format!("{}/calendar.ics", self.folder_name));
        
        uid
    }

    pub fn change_event(&mut self, uid: &str, new_title: Option<String>, new_day: Option<u8>, new_month: Option<u8>, new_year: Option<u32>, 
        new_begin_hour: Option<u8>, new_begin_mins: Option<u8>, new_end_hour: Option<u8>, new_end_mins: Option<u8>) -> bool {
        
        let title = new_title.filter(|t| !t.is_empty());
        
        let new_date = if let (Some(y), Some(m), Some(d)) = 
            (new_year, new_month, new_day) {
                Some(format!("{:04}{:02}{:02}", y, m, d))
            } else { None };

        let new_beg = if let (Some(bh), Some(bm)) = 
            (new_begin_hour, new_begin_mins) {
                Some(format!("{:02}{:02}00", bh, bm))
            } else { None };
        
        let new_end = if let (Some(eh), Some(em)) = (new_end_hour, new_end_mins){
            Some(format!("{:02}{:02}00", eh, em))
        }else {None};

        let res = self.calendar_controller.change_event(uid, title, new_date, new_beg, new_end);
        
        if res {
            let _ = self.calendar_controller.save(&format!("{}/calendar.ics", self.folder_name));
        }
        res
    }

    pub fn delete_event(&mut self, uid: &str) -> bool {
        self.calendar_controller.delete_event(uid);
        true
    }

    pub fn get_events(&self, month: u32, year: u32) -> (Vec<UICalendarEvent>, Vec<DateInfo>) {
        if month > 12 || month == 0 {
            return (Vec::new(), Vec::new());
        }
        let current_date = Local::now().date_naive();

        let year_now: u32 = current_date.year() as u32;
        let month_now: u32 = current_date.month();
        
        if year_now > year {return (Vec::new(), Vec::new());}
        if year_now == year && month < month_now{
            return (Vec::new(), Vec::new());
        }

        self.calendar_controller.get_calendar_data(month, year)
    }

    pub fn make_date(&self, year: u32, month: u8, day: u8, hour: u8, mins: u8) -> String {
        let local_dt = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
            .and_then(|d| d.and_hms_opt(hour as u32, mins as u32, 0))
            .expect("Invalid date or time provided");

        let utc_dt = Local.from_local_datetime(&local_dt)
            .single() 
            .unwrap_or_else(|| local_dt.and_local_timezone(Local).unwrap());

        utc_dt.with_timezone(&chrono::Utc).format("%Y%m%dT%H%M%SZ").to_string()
    }

    pub fn create_note(&mut self, title: &str) {
        self.notes_controller.create_note(title, "");
    }

    pub fn edit_note_content(&mut self, id: Uuid, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.notes_controller.edit_note_content(id, content)?;
        Ok(())
    }

    pub fn edit_note_title(&mut self, id: Uuid, title: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.notes_controller.edit_note_title(id, title)?;
        Ok(())
    }

    pub fn get_note(&self, id: Uuid) -> Option<&Note> {
        self.notes_controller.get_note(id)
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.notes_controller.get_notes()
    }

    pub fn delete_note(&mut self, id: Uuid) -> Result<Note, String> {
        self.notes_controller.delete_note(id)
    }

    pub fn create_category(&mut self, name: &str, color: Option<&str>) {
        self.notes_controller.create_category(name, color);
    }

    pub fn assign_category(&mut self, note_id: Uuid, category_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.notes_controller.assign_category(note_id, category_id)?;
        Ok(())
    }

    pub fn remove_category(&mut self, note_id: Uuid, category_id: Uuid)-> Result<(), Box<dyn std::error::Error>> {
        self.notes_controller.remove_category(note_id, category_id)?;
        Ok(())
    }
    pub fn has_category(&self, note_id: Uuid, category_id: Uuid) -> bool {
        self.notes_controller.has_category(note_id, category_id)
    }

    pub fn filter_by_category(&self, category_id: Uuid) -> Vec<Note> {
        self.notes_controller.filter_by_category(category_id)
    }

    pub fn get_categories(&self) -> Vec<Category> {
        self.notes_controller.get_categories()
    }
}