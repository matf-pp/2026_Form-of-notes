/*
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;

slint::include_modules!();
mod app_state;

use app_state::AppState;
use std::sync::{Arc, Mutex};
*/

mod calendar_controller;
//mod task_controller;

fn main() -> Result<(), Box<dyn std::error::Error>> {
/*    let window = AppWindow::new()?;
    let state = Arc::new(Mutex::new(AppState::new()));

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_get_tasks(move |sort_by| {
            let mut s = state.lock().unwrap();
            let tasks = tasks_to_slint(&s.get_tasks(sort_by as u8));
            win.unwrap().set_tasks(tasks.clone());
            tasks
        });
    }

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_add_task(move |title, priority, deadline| {
            let mut s = state.lock().unwrap();
            let deadline = if deadline.is_empty() { None } else { Some(deadline.as_str()) };
            s.modify_task(0, Some(priority as u8), Some(&title), deadline);
            save_and_refresh(&mut s, &win);
        });
    }

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_change_task(move |id, priority, title, deadline| {
            let mut s    = state.lock().unwrap();
            let priority = if priority < 0 { None } else { Some(priority as u8) };
            let title    = if title.is_empty()    { None } else { Some(title.as_str()) };
            let deadline = if deadline.is_empty() { None } else { Some(deadline.as_str()) };
            s.modify_task(id as u32, priority, title, deadline);
            save_and_refresh(&mut s, &win);
        });
    }

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_finish_task(move |id| {
            let mut s = state.lock().unwrap();
            s.finish_task(id as u32);
            save_and_refresh(&mut s, &win);
        });
    }

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_update_time(move |id, time_str| {
            let mut s = state.lock().unwrap();
            s.add_time(id as u32, &time_str);
            save_and_refresh(&mut s, &win);
        });
    }

    // ── Calendar ──────────────────────────────────────────────────────────

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_add_event(move |title, start, end, description| {
            let mut s   = state.lock().unwrap();
            let desc    = if description.is_empty() { None } else { Some(description.as_str()) };
            s.add_event(&title, &start, &end, desc);
            let events  = events_to_slint(&s.get_events());
            win.unwrap().set_events(events);
            if let Err(e) = s.save_data() { eprintln!("Save failed: {e}"); }
        });
    }

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_remove_event(move |uid| {
            let mut s  = state.lock().unwrap();
            s.remove_event(&uid);
            let events = events_to_slint(&s.get_events());
            win.unwrap().set_events(events);
            if let Err(e) = s.save_data() { eprintln!("Save failed: {e}"); }
        });
    }

    {
        let win   = window.as_weak();
        let state = Arc::clone(&state);
        window.on_get_events(move || {
            let s      = state.lock().unwrap();
            let events = events_to_slint(&s.get_events());
            win.unwrap().set_events(events.clone());
            events
        });
    }

    {
        let state = Arc::clone(&state);
        window.on_export_ics(move |filepath| {
            if let Err(e) = state.lock().unwrap().export_calendar_ics(&filepath) {
                eprintln!("Export failed: {e}");
            }
        });
    }


    window.run()?;
    state.lock().unwrap().save_data()?;
*/
    Ok(())

}
/*
fn save_and_refresh(s: &mut AppState, win: &slint::Weak<AppWindow>) {
    if let Err(e) = s.save_data() { eprintln!("Save failed: {e}"); }
    win.unwrap().set_tasks(tasks_to_slint(&s.get_tasks(0)));
}

fn tasks_to_slint(tasks: &[task_controller::Task]) -> slint::ModelRc<SlintTask> {
    let vec: Vec<SlintTask> = tasks.iter().map(|t| {
        // parse "ddd:hh:mm" back into parts for the UI
        let parts: Vec<&str> = t.total_time.splitn(3, ':').collect();
        let days  = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
        let hours = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let mins  = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);

        SlintTask {
            id:          t.id as i32,
            title:       t.title.clone().into(),
            status:      t.status as i32,
            priority:    t.priority as i32,
            total_days:  days,
            total_hours: hours,
            total_mins:  mins,
            deadline:    t.deadline.clone().unwrap_or_default().into(),
        }
    }).collect();

    slint::ModelRc::new(slint::VecModel::from(vec))
}

fn events_to_slint(events: &[CalendarEvent]) -> slint::ModelRc<SlintCalendarEvent> {
    let vec: Vec<SlintCalendarEvent> = events.iter().map(|e| {
        SlintCalendarEvent {
            uid:        e.uid.clone().into(),
            title:      e.title.clone().into(),
            begin_date: e.begin_date.clone().into(),
            end_date:   e.end_date.clone().into(),
        }
    }).collect();
    slint::ModelRc::new(slint::VecModel::from(vec))
}
    */