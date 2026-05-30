use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub categories: HashSet<Uuid>,
    pub last_update: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Default)]
pub struct NotesController {
    notes: HashMap<Uuid, Note>,
    categories: HashMap<Uuid, Category>,
}

impl NotesController {
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    pub fn create_note(&mut self, title: &str, content: &str) -> Note {
        let note = Note {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content: content.to_string(),
            categories: HashSet::new(),
            last_update: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
        };
        self.notes.insert(note.id, note.clone());
        note
    }

    pub fn edit_note_content(&mut self, id: Uuid, content: &str) -> Result<(), String> {
        if let Some(note) = self.notes.get_mut(&id) {
            note.content = content.to_string();
            note.last_update = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn edit_note_title(&mut self, id: Uuid, title: &str) -> Result<(), String> {
        if let Some(note) = self.notes.get_mut(&id) {
            note.title = title.to_string();
            note.last_update = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn get_note(&self, id: Uuid) -> Option<&Note> {
        self.notes.get(&id)
    }

    pub fn get_notes(&self) -> Vec<Note> {
        let mut notes_list: Vec<Note> = self.notes.values().cloned().collect();
        notes_list.sort_by(|a, b| b.last_update.cmp(&a.last_update));
        notes_list
    }

    pub fn delete_note(&mut self, id: Uuid) -> Result<Note, String> {
        if let Some(removed_note) = self.notes.remove(&id) {
            Ok(removed_note)
        } else {
            Err("Note not found".to_string())
        }
    }
    
    pub fn create_category(&mut self, name: &str, color: Option<&str>) -> Category {
        let category = Category {
            id: Uuid::new_v4(),
            name: name.to_string(),
            color: color.map(|s| s.to_string()),
        };
        self.categories.insert(category.id, category.clone());
        category
    }

    pub fn assign_category(&mut self, note_id: Uuid, category_id: Uuid) -> Result<(), String> {
        if !self.categories.contains_key(&category_id) {
            return Err("Category does not exist".to_string());
        }

        if let Some(note) = self.notes.get_mut(&note_id) {
            note.categories.insert(category_id);
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn remove_category(&mut self, note_id: Uuid, category_id: Uuid) -> Result<(), String> {
        if !self.categories.contains_key(&category_id) {
            return Err("Category does not exist".to_string());
        }

        if let Some(note) = self.notes.get_mut(&note_id) {
            note.categories.remove(&category_id);
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn filter_by_category(&self, category_id: Uuid) -> Vec<&Note> {
        self.notes.values().filter(|note| note.categories.contains(&category_id)).collect()
    }

    pub fn get_categories(&self) -> Vec<Category> {
        let mut ctg_list: Vec<Category> = self.categories.values().cloned().collect();
        ctg_list.sort_by(|a, b| a.name.cmp(&b.name));
        ctg_list
    }

    pub fn import_notes(&mut self, filepath: &str) -> Result<Self, Box<dyn std::error::Error>>{
        let mut ctg_path = PathBuf::from(filepath);
        ctg_path.push("categories");
        ctg_path.set_extension("json");

        let mut notes_path = PathBuf::from(filepath);  
        notes_path.push("categories");
        notes_path.set_extension("json");

        let notes_json = std::fs::read_to_string(notes_path)?;
        let ctg_json = std::fs::read_to_string(ctg_path)?;

        let imp_notes: HashMap<Uuid, Note> = serde_json::from_str(&notes_json)?;
        let imp_categories: HashMap<Uuid, Category> = serde_json::from_str(&ctg_json)?;

        let controller = NotesController {
            notes: imp_notes.clone(),
            categories: imp_categories.clone(),
        };
        
        Ok(controller)
    }

    pub fn save(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let ctg_json = serde_json::to_string_pretty(&self.categories)?;
        let notes_json = serde_json::to_string_pretty(&self.notes)?;

        let mut ctg_path = PathBuf::from(filepath);
        ctg_path.push("categories");
        ctg_path.set_extension("json");

        let mut notes_path = PathBuf::from(filepath);  
        notes_path.push("categories");
        notes_path.set_extension("json");

        std::fs::write(ctg_path, ctg_json)?;
        std::fs::write(notes_path, notes_json)?;
        Ok(())
    }
}