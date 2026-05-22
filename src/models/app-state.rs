


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppState {
    pub calendar_controller: CalendarController,    
    pub task_controller: TaskController,
    window: Option<AppWindow>, //idk if needed, depends on implemention of front to back communication
    folder_name: String,
}

impl AppState {
    pub fn new(folder: Option<&str>) -> Self {
        AppState {
            my_calendar: Calendar::new(),
            events: BTreeMap::new(),
            workspace_name: "My Workspace",
            folder_name: Some(folder) ? str : "home", //find a default folder for windows and linux platforms
        }
    }

    pub fn get_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.calendar_controller.get_calendar(self.calendar_controller, self.folder_name + "/calendar.json");
        self.task_controller.get_tasks(self.task_controller, self.folder_name + "/tasks.json");
        
        Ok(())
    }

    pub fn export_calendar(&self, filepath: &str) -> std::io::Result<()>{
        let ics_string = self.my_calendar.to_string();
        let mut output_file = File::create(filepath)?;
        output_file.write_all(ics_string.as_bytes())?;
        Ok(())
        //ili calendar.generate()? proveriti da li rade isto
    }

    pub fn initialize_ui(&mut self) {
        let window = AppWindow::new().expect("Unable to create");
        self.window = Some(Window);
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        let window = self.window.as_ref().expect("Unable to access");
        window.run();
    }
}
