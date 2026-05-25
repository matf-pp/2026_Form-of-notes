//Calendar controller tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_generation() {
        let controller = CalendarController::new();
        // May 2026: 1st is a Friday (index 5)
        let (_, grid) = controller.get_calendar_data(5, 2026);
        
        // 1st of May should be at row 0, col 5
        let first = grid.iter().find(|d| d.day_of_month == 1).unwrap();
        assert_eq!(first.row_idx, 0);
        assert_eq!(first.col_idx, 5);
        
        // 31st of May should be at row 4, col 0
        let last = grid.iter().find(|d| d.day_of_month == 31).unwrap();
        assert_eq!(last.row_idx, 4);
        assert_eq!(last.col_idx, 0);
    }

    #[test]
    fn test_event_sorting() {
        let mut controller = CalendarController::new();
        // Add events out of order
        controller.add_event("Late".to_string(), "20260525T150000".to_string(), "20260525T160000".to_string());
        controller.add_event("Early".to_string(), "20260525T080000".to_string(), "20260525T090000".to_string());

        let (events, _) = controller.get_calendar_data(5, 2026);
        
        // Check if "Early" comes before "Late"
        assert_eq!(events[0].title, "Early");
        assert_eq!(events[1].title, "Late");
    }
    
    #[test]
    fn test_change_event() {
        let mut controller = CalendarController::new();
        
        // 1. Add an event and capture the UID
        let uid = controller.add_event(
            "Original Title".to_string(), 
            "20260525T100000".to_string(), 
            "20260525T110000".to_string()
        );

        // 2. Attempt to change it
        let success = controller.change_event(
            &uid, 
            Some("Updated Title".to_string()), 
            Some("20260525T120000".to_string()), 
            None
        );

        // 3. Verify success
        assert!(success, "change_event should return true for existing UID");

        // 4. Verify the change by re-fetching the month data
        let (events, _) = controller.get_calendar_data(5, 2026);
        let updated_event = events.iter().find(|e| e.uid == uid).unwrap();
        
        assert_eq!(updated_event.title, "Updated Title");
        assert_eq!(updated_event.begin_hour, 12);
    }

}



//*--------------------------------------Task Controller tests--------------------------------------------
/*

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_task_creation_and_id_assignment() {
        let mut tc = TaskController::new();
        tc.add_task(1, "Task A", None);
        tc.add_task(2, "Task B", Some("2026-06-01"));

        assert_eq!(tc.tasks.len(), 2);
        assert_eq!(tc.tasks[0].id, 1);
        assert_eq!(tc.tasks[1].id, 2);
        assert_eq!(tc.tasks[1].deadline, Some("2026-06-01".to_string()));
    }

    #[test]
    fn test_add_time_and_status_update() {
        let mut tc = TaskController::new();
        tc.add_task(1, "Coding", None);
        
        // Add 30 mins, then 40 mins (total 1h 10m)
        tc.add_time(1, 0, 30);
        tc.add_time(1, 0, 40);
        
        let task = &tc.tasks[0];
        assert_eq!(task.status, 1); // Should be In Progress
        assert_eq!(task.total_time_hours, 1);
        assert_eq!(task.total_time_mins, 10);
    }

    #[test]
    fn test_sorting_logic() {
        let mut tc = TaskController::new();
        tc.add_task(1, "Alpha", None);
        tc.add_task(5, "Beta", None);
        
        // Test sort by priority (descending: 5 then 1)
        let sorted = tc.get_tasks(2);
        assert_eq!(sorted[0].priority, 5);
        assert_eq!(sorted[1].priority, 1);
    }

    #[test]
    fn test_finish_task() {
        let mut tc = TaskController::new();
        tc.add_task(1, "Do this", None);
        tc.finish_task(1);
        
        assert_eq!(tc.tasks[0].status, 2);
    }

    #[test]
    fn test_save_and_import() {
        let mut tc = TaskController::new();
        tc.add_task(1, "Persist me", None);
        
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();
        
        tc.save(path).unwrap();
        
        let imported = TaskController::import_tasks(path).unwrap();
        assert_eq!(imported.tasks.len(), 1);
        assert_eq!(imported.tasks[0].title, "Persist me");
    }
}

*/