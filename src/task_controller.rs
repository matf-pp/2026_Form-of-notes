use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Task {
    pub id: u32, //interni id
    pub title: String,
    pub status: u8, //0 DNS, 1 In Progress, 2 Done
    pub priority: u8,
    pub total_time_days: u32,
    pub total_time_hours: u8,
    pub total_time_mins: u8, // 000:00:00 ddd:hh:mm
    pub deadline: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TaskController {
    pub next_id: u32, //individual tabs of tasks
    pub tasks: Vec<Task>,
}

impl TaskController{
    pub fn new() -> Self {
        Self{
            next_id: 1,
            tasks: Vec::new(),
        }
    }
    pub fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_task(&mut self, priority: u8, title: &str, deadline: Option<&str>) -> bool {
        let tmp = Task{
            id: self.get_next_id(),
            title: title.to_string(),
            status: 0,
            total_time_days: 0,
            total_time_hours: 0,
            total_time_mins: 0,
            deadline: deadline.map(str::to_string),
            priority,
        };
        self.tasks.push(tmp);
        true
    }

    pub fn change_task(&mut self, id: u32, p: Option<u8>, t: Option<String>, d: Option<String>) -> bool{
        if let Some(task) = self.tasks.iter_mut().find(|x| x.id == id) {
            if let Some(p) = p {task.priority = p;}
            if let Some(t) = t {task.title = t;}
            if d.is_some() {task.deadline = d;}
            return true;
        }
        false
    }
    pub fn finish_task(&mut self, id: u32){
        if let Some(task) = self.tasks.iter_mut().find(|x| x.id == id) {
            task.status = 2;
        }
    }

    pub fn add_time(&mut self, id: u32, time_hours: u8, time_mins: u8){ //time: hh:mm
        if let Some(task) = self.tasks.iter_mut().find(|x| x.id == id) {
            task.total_time_mins += time_mins;
            if task.total_time_mins >= 60 {
                task.total_time_hours += 1;
                task.total_time_mins %= 60;
            }
            task.total_time_hours += time_hours;
            if task.total_time_hours >= 24 {
                task.total_time_days += 1;
                task.total_time_hours %= 24;
            }
            task.status = 1;
        }
    }

    pub fn get_tasks(&mut self, sortby: u8) -> Vec<Task>{ //1 names asc, 2 priority desc, 3 deadline earliest first
        if sortby == 1 {
            self.tasks.sort_by_key(|t| t.title.clone());
        }else if sortby == 2 {
            //add for 2 and 3 to leave the finished ones at the end
            self.tasks.sort_by(|a, b| {
                let status_a = a.status == 2;
                let status_b = b.status == 2;
                status_a.cmp(&status_b)
                    .then(a.priority.cmp(&b.priority))
            });
        }else if sortby == 3 {
            self.tasks.sort_by(|a, b| {
                let status_a = a.status == 2;
                let status_b = b.status == 2;
                status_a.cmp(&status_b)
                    .then_with(|| {
                        match (&a.deadline, &b.deadline) {
                            (Some(da), Some(db)) => da.cmp(db),
                            (Some(_), None) => std::cmp::Ordering::Less,    // Has deadline comes first
                            (None, Some(_)) => std::cmp::Ordering::Greater, // No deadline goes last
                            (None, None) => std::cmp::Ordering::Equal,
                        }
                    }) 
            });
        }
        self.tasks.clone()
    }

    pub fn import_tasks(&mut self, filepath: &str) -> Result<Self, Box<dyn std::error::Error>>{
        let json = std::fs::read_to_string(filepath)?;
        let controller = serde_json::from_str(&json)?;
        Ok(controller)
    }

    pub fn save(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        std::fs::write(filepath, json)?;
        Ok(())
    }
    pub fn delete_task(&mut self, id: u32) -> bool {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        self.tasks.len() < initial_len 
    }
}