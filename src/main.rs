slint::include_modules!();

mod app_state;
mod calendar_controller;
mod task_controller;
mod notes_controller;

use app_state::AppState;
use chrono::{Datelike, Local, NaiveDate};
use slint::{ModelRc, SharedString, VecModel};
use std::sync::{Arc, Mutex};

fn to_slint_events(events: &[calendar_controller::UICalendarEvent]) -> ModelRc<SlintCalendarEvent> {
    let v: Vec<SlintCalendarEvent> = events
        .iter()
        .map(|e| {
            let date = NaiveDate::from_ymd_opt(e.year as i32, e.month as u32, e.day as u32)
                .unwrap_or_default();
            
            let weekday = match date.weekday() {
                chrono::Weekday::Mon => "Mon",
                chrono::Weekday::Tue => "Tue",
                chrono::Weekday::Wed => "Wed",
                chrono::Weekday::Thu => "Thu",
                chrono::Weekday::Fri => "Fri",
                chrono::Weekday::Sat => "Sat",
                chrono::Weekday::Sun => "Sun",
            };

            SlintCalendarEvent {
                uid: e.uid.clone().into(),
                title: e.title.clone().into(),
                begin_day: e.day as i32,
                day_of_week: weekday.into(),
                begin_time: format!("{:02}:{:02}", e.begin_hour, e.begin_mins).into(),
                end_time: format!("{:02}:{:02}", e.end_hour, e.end_mins).into(),
            }
        })
        .collect();
    ModelRc::new(VecModel::from(v))
}

fn to_slint_days(days: &[calendar_controller::DateInfo]) -> ModelRc<DateInfo> {
    let v: Vec<DateInfo> = days
        .iter()
        .map(|d| DateInfo {
            row_idx: d.row_idx as i32,
            col_idx: d.col_idx as i32,
            day_of_month: d.day_of_month as i32,
            has_event: d.has_event,
        })
        .collect();
    ModelRc::new(VecModel::from(v))
}

fn to_slint_tasks(tasks: &[task_controller::Task]) -> ModelRc<SlintTask> {
    let v: Vec<SlintTask> = tasks
        .iter()
        .map(|t| SlintTask {
            id: t.id as i32,
            title: t.title.clone().into(),
            status: t.status as i32,
            priority: t.priority as i32,
            total_hours: t.total_time_hours as i32,
            total_mins: t.total_time_mins as i32,
            total_days: t.total_time_days as i32,
            deadline: t.deadline.clone().unwrap_or_default().into(),
        })
        .collect();
    ModelRc::new(VecModel::from(v))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let state = Arc::new(Mutex::new(AppState::new(None)));

    {
        let mut s = state.lock().unwrap();
        let _ = s.import_data();
    }

//--------------------------------------------TASKS--------------------------------------------
    let s_task_get = state.clone();
    ui.on_get_tasks(move |sort_by| {
        let mut s = s_task_get.lock().unwrap();
        to_slint_tasks(&s.get_tasks(sort_by as u8))
    });

    let s_task_mod = state.clone();
    let ui_mod_weak = ui.as_weak();
    ui.on_modify_task(move |id, priority, title, deadline| {
        let mut s = s_task_mod.lock().unwrap();
        s.modify_tasks(
            id as u32,
            Some(priority as u8),
            Some(title.as_str()),
            if deadline.is_empty() { None } else { Some(deadline.as_str()) },
        );
        let _ = s.save_data();
        if let Some(ui) = ui_mod_weak.upgrade(){
            ui.set_tasks(to_slint_tasks(&s.get_tasks(0)));
        }
    });

    let s_task_time = state.clone();
    let ui_time_weak = ui.as_weak();
    ui.on_update_time(move |id, hours, mins| {
        let mut s = s_task_time.lock().unwrap();
        s.add_time(id as u32, hours as u8, mins as u8);
        let _ = s.save_data();

        if let Some(ui) = ui_time_weak.upgrade(){
            ui.set_tasks(to_slint_tasks(&s.get_tasks(0)));
        }
    });

    let s_task_fin = state.clone();
    let ui_fin_weak = ui.as_weak();
    ui.on_finish_task(move |id| {
        let mut s = s_task_fin.lock().unwrap();
        s.finish_task(id as u32);
        let _ = s.save_data();
        
        if let Some(ui) = ui_fin_weak.upgrade(){
            ui.set_tasks(to_slint_tasks(&s.get_tasks(0)));
        }
    });

    let s_task_del = state.clone();
    let ui_del_weak = ui.as_weak();
    ui.on_delete_task(move |id| {
        let mut s = s_task_del.lock().unwrap();
        s.delete_task(id as u32);
        let _ = s.save_data();

        if let Some(ui) = ui_del_weak.upgrade(){
            ui.set_tasks(to_slint_tasks(&s.get_tasks(0)));
        }
    });

    ui.on_is_valid_date(|date_str| {
        NaiveDate::parse_from_str(date_str.as_str(), "%Y.%m.%d").is_ok()
    });

//---------------------------------------------------------Calendar-------------------------------

    let s_cal_get = state.clone();
    let ui_weak = ui.as_weak();
    ui.on_get_events(move |month, year| {
        if let Some(ui) = ui_weak.upgrade() {
            let s = s_cal_get.lock().unwrap();
            let (events, days) = s.get_events(month as u32, year as u32);
            
            ui.set_events(to_slint_events(&events));
            ui.set_calendar_days(to_slint_days(&days));
            ui.set_calendar_month(month);
            ui.set_calendar_year(year);

            let month_name = match month {
                1 => "January", 2 => "February", 3 => "March", 4 => "April",
                5 => "May", 6 => "June", 7 => "July", 8 => "August",
                9 => "September", 10 => "October", 11 => "November", 12 => "December",
                _ => "Unknown"
            };
            ui.set_calendar_month_name(month_name.into());
        }
    });

    let s_cal_add = state.clone();
    let ui_add_weak = ui.as_weak();
    ui.on_add_event(move |title, day, month, year, bh, bm, eh, em| {
        let mut s = s_cal_add.lock().unwrap();
        s.add_event(
            title.as_str(), day as u8, month as u8, year as u32,
            bh as u8, bm as u8, eh as u8, em as u8,
        );

        let _ = s.save_data();

        if let Some(ui) = ui_add_weak.upgrade() {
            let cur_month = ui.get_calendar_month();
            let cur_year = ui.get_calendar_year();
            let (events, days) = s.get_events(cur_month as u32, cur_year as u32);
            ui.set_events(to_slint_events(&events));
            ui.set_calendar_days(to_slint_days(&days));
        }
    });

    let s_cal_del = state.clone();
    let ui_del_weak = ui.as_weak();
    ui.on_delete_event(move |uid| {
        let mut s = s_cal_del.lock().unwrap();
        s.delete_event(uid.as_str());
        let _ = s.save_data();
        
        if let Some(ui) = ui_del_weak.upgrade() {
            let cur_month = ui.get_calendar_month();
            let cur_year = ui.get_calendar_year();
            let (events, days) = s.get_events(cur_month as u32, cur_year as u32);
            ui.set_events(to_slint_events(&events));
            ui.set_calendar_days(to_slint_days(&days));
        }
    });

    let s_cal_chg = state.clone();
    let ui_chg_weak = ui.as_weak();
    ui.on_change_event(move |uid, title, day, month, year, bh, bm, eh, em| {
        let mut s = s_cal_chg.lock().unwrap();
        
        //need to check this, it's behaving weirdly Potentially uid is making an issue idk, kinda tired
        let opt_title = if title.is_empty() { None } else { Some(title.to_string()) };
        let opt_day = if day == 0 { None } else { Some(day as u8) };
        let opt_month = if month == 0 { None } else { Some(month as u8) };
        let opt_year = if year == 0 { None } else { Some(year as u32) };
        let opt_bh = if bh == -1 { None } else { Some(bh as u8) };
        let opt_bm = if bm == -1 { None } else { Some(bm as u8) };
        let opt_eh = if eh == -1 { None } else { Some(eh as u8) };
        let opt_em = if em == -1 { None } else { Some(em as u8) };

        s.change_event(
            uid.as_str(),
            opt_title,
            opt_day,
            opt_month,
            opt_year,
            opt_bh,
            opt_bm,
            opt_eh,
            opt_em,
        );
        let _ = s.save_data();

        if let Some(ui) = ui_chg_weak.upgrade() {
            let cur_month = ui.get_calendar_month();
            let cur_year = ui.get_calendar_year();
            let (events, days) = s.get_events(cur_month as u32, cur_year as u32);
            ui.set_events(to_slint_events(&events));
            ui.set_calendar_days(to_slint_days(&days));
        }
    });


//Some initialising things
    
    let current_date = Local::now().date_naive();
    ui.invoke_get_events(current_date.month() as i32, current_date.year() as i32);

    {
        let mut s = state.lock().unwrap();
        ui.set_tasks(to_slint_tasks(&s.get_tasks(0)));
    }

    ui.run()
}