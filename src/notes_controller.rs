use std::collections::{HashMap, HashSet}
use serde::{Serialize, Deserialize};
use uuid::Uuid;

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

#[derive(Debug)]
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
            last_update: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_sec(),
        };
        self.notes.insert(note.id, note.clone());
        note
    }

    pub fn edit_note_content(&mut self, id: Uuid, content: &str) -> Result<(), String> {
        if let Some(note) = self.notes.get_mut(&id) {
            note.content = content.to_string();
            note.last_update = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_sec();
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn edit_note_title(&mut self, id: Uuid, title: &str) -> Result<(), String> {
        if let Some(note) = self.notes.get_mut(&id) {
            note.title = title.to_string();
            note.last_update = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_sec();
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn get_note(&self, id: Uuid) -> Option<&Note> {
        self.notes.get(&id)
    }

    pub fn get_notes(&self) -> Vec<&Note> {
        let mut notes_list: Vec<&Note> = self.notes.values().collect();
        notes_list.sort_by(|a, b| b.last_update.cmp(&a.last_update));
        notes_list
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
        if !self.categories.contains_key(&categories_id) {
            return Err("Category does not exist".to_string());
        }

        if let Some(note) = self.notes.get_mut(&id) {
            note.categories.insert(category_id);
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn remove_category(&mut self, note_id: Uuid, category_id: Uuid) -> Result<(), String> {
        if !self.categories.contains_key(&categories_id) {
            return Err("Category does not exist".to_string());
        }

        if let Some(note) = self.notes.get_mut(&id) {
            note.categories.remove(&category_id);
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    pub fn filter_category(&self, category_id: Uuid) -> Vec<&Note> {
        self.notes.values().filter(|note| note.categories.contains(&category_id)).collect()
    }
}