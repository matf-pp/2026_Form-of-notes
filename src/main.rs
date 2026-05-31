slint::include_modules!();

mod app_state;
mod calendar_controller;
mod task_controller;
mod notes_controller;

use app_state::AppState;
use chrono::{Datelike, Local, NaiveDate};
use slint::{ModelRc, SharedString, VecModel};
use uuid::Uuid;
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

fn to_slint_notes(notes: &[notes_controller::Note]) -> ModelRc<SlintNote> {
    let v: Vec<SlintNote> = notes
        .iter()
        .map(|n| {
            
            let cat_ids: Vec<SharedString> = n.categories
                .clone()
                .into_iter()
                .map(|id| SharedString::from(id.to_string()))
                .collect();

            SlintNote {
                id: n.id.to_string().into(),
                title: n.title.clone().into(),
                content: n.content.clone().into(),
                category_ids: ModelRc::new(VecModel::from(cat_ids)),
                last_update: n.last_update.to_string().into(),
            }
        })
        .collect();
    ModelRc::new(VecModel::from(v))
}

fn to_slint_categories(categories: &[notes_controller::Category]) -> ModelRc<SlintCategory> {
    let v: Vec<SlintCategory> = categories
        .iter()
        .map(|c| SlintCategory {
            id: c.id.to_string().into(),
            name: c.name.to_string().into(),
            color: c.color.clone().unwrap_or_default().into(),
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

//-------NOTES-------
    let s_new_note = state.clone();
    let ui_newn_weak = ui.as_weak();
    ui.on_new_note(move || {
        let mut s = s_new_note.lock().unwrap();
        s.create_note("New note");

        let _ = s.save_data();
        if let Some(ui) = ui_newn_weak.upgrade(){
            ui.set_notes(to_slint_notes(&s.get_notes()));
        }
    });

    let s_new_ctg = state.clone();
    let ui_newc_weak = ui.as_weak();
    ui.on_add_category(move |ctg_title| {
        let mut s = s_new_ctg.lock().unwrap();
        s.create_category(
            ctg_title.as_str(),
            Some("ff0000")
        );

        let _ = s.save_data();
        if let Some(ui) = ui_newc_weak.upgrade(){
            ui.set_categories(to_slint_categories(&s.get_categories()));
        }
    });

    let s_chg_nc = state.clone();
    let ui_chgc_weak = ui.as_weak();
    ui.on_change_note_content(move |id, content| {
        let mut s = s_chg_nc.lock().unwrap();
        
        let _ = s.edit_note_content(Uuid::parse_str(id.as_str()).unwrap(), content.as_str());

        let _ = s.save_data();
        if let Some(ui) = ui_chgc_weak.upgrade(){
            ui.set_notes(to_slint_notes(&s.get_notes()));
        }
    });

    let s_chg_nt = state.clone();
    let ui_chgt_weak = ui.as_weak();
    ui.on_change_note_title(move |id, title| {
        let mut s = s_chg_nt.lock().unwrap();
        
        let _ = s.edit_note_title(Uuid::parse_str(id.as_str()).unwrap(), title.as_str());

        let _ = s.save_data();
        if let Some(ui) = ui_chgt_weak.upgrade(){
            ui.set_notes(to_slint_notes(&s.get_notes()));
        }
    });

    let s_slc_note = state.clone();
    let ui_slcn_weak = ui.as_weak();
    ui.on_select_note(move |id| {
        let s = s_slc_note.lock().unwrap();

        let mut new_title = "-1".to_string();
        let mut new_content = "-1".to_string();
        let mut new_id = "-1".to_string();

        let uid: Uuid = Uuid::parse_str(id.as_str()).unwrap();
        if let Some(note) = s.get_note(uid) {
            new_title = note.title.clone();
            new_content = note.content.clone();
            new_id = id.to_string();
        }
        let _ = s.save_data();
        if let Some(ui) = ui_slcn_weak.upgrade(){
            ui.set_selected_note_id(new_id.into());
            ui.set_editing_title(new_title.into());
            ui.set_editing_content(new_content.into());
            ui.set_notes(to_slint_notes(&s.get_notes()));
        }
    });

    let s_del_note = state.clone();
    let ui_deln_weak = ui.as_weak();
    ui.on_delete_note(move |id| {
        let mut s = s_del_note.lock().unwrap();
        
        let _ = s.delete_note(Uuid::parse_str(id.as_str()).unwrap());

        let _ = s.save_data();
        if let Some(ui) = ui_deln_weak.upgrade(){
            ui.set_selected_note_id(SharedString::from("-1"));
            ui.set_notes(to_slint_notes(&s.get_notes()));
        }
    });

    let s_slc_ctg = state.clone();
    let ui_slcc_weak = ui.as_weak();
    ui.on_select_category(move |id| {
        let s = s_slc_ctg.lock().unwrap();
        let _ = s.save_data();
        if let Some(ui) = ui_slcc_weak.upgrade(){
            if id == "all" {
                ui.set_notes(to_slint_notes(&s.get_notes()));
            }
            if id != "all" {
                let uid: Uuid = Uuid::parse_str(id.as_str()).unwrap();
                ui.set_notes(to_slint_notes(&s.filter_by_category(uid)));
            }
        }
    });

    let s_asg_ctg = state.clone();
    let ui_asgc_weak = ui.as_weak();
    ui.on_add_note_category(move |note_id, ctg_id| {
        let mut s = s_asg_ctg.lock().unwrap();
        
        let note_uid: Uuid = Uuid::parse_str(note_id.as_str()).unwrap();
        let ctg_uid: Uuid = Uuid::parse_str(ctg_id.as_str()).unwrap();

        let _ = s.assign_category(note_uid, ctg_uid);
        
        let _ = s.save_data();
        if let Some(ui) = ui_asgc_weak.upgrade(){
            ui.set_notes(to_slint_notes(&s.get_notes()));

            let current_selected = ui.get_selected_note_id();
            ui.set_selected_note_id(slint::SharedString::from(""));
            ui.set_selected_note_id(current_selected);
        }
    });

    let s_rmv_ctg = state.clone();
    let ui_rmvc_weak = ui.as_weak();
    ui.on_remove_note_category(move |note_id, ctg_id| {
        let mut s = s_rmv_ctg.lock().unwrap();
        
        let note_uid: Uuid = Uuid::parse_str(note_id.as_str()).unwrap();
        let ctg_uid: Uuid = Uuid::parse_str(ctg_id.as_str()).unwrap();

        let _ = s.remove_category(note_uid, ctg_uid);

        let _ = s.save_data();
        if let Some(ui) = ui_rmvc_weak.upgrade(){
            ui.set_notes(to_slint_notes(&s.get_notes()));

            let current_selected = ui.get_selected_note_id();
            ui.set_selected_note_id(slint::SharedString::from(""));
            ui.set_selected_note_id(current_selected);
        }
    });

    let s_has_ctg = state.clone();
    ui.on_has_category(move |note_id, ctg_id| {
        let s = s_has_ctg.lock().unwrap();
        
        let note_uid: Uuid = Uuid::parse_str(note_id.as_str()).unwrap();
        let ctg_uid: Uuid = Uuid::parse_str(ctg_id.as_str()).unwrap();

        s.has_category(note_uid, ctg_uid)
    });

//Some initialising things
    
    let current_date = Local::now().date_naive();
    ui.invoke_get_events(current_date.month() as i32, current_date.year() as i32);

    {
        let mut s = state.lock().unwrap();
        ui.set_tasks(to_slint_tasks(&s.get_tasks(0)));
        ui.set_notes(to_slint_notes(&s.get_notes()));
        ui.set_categories(to_slint_categories(&s.get_categories()));
    }

    ui.run()
}