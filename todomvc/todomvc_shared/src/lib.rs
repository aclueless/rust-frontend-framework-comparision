use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use gloo_storage::{LocalStorage, Storage, errors::StorageError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todos {
    pub entries: Vec<TodoEntry>,
    pub filter: Filter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoEntry {
    pub id: uuid::Uuid,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn fits(&self, entry: &TodoEntry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }
}

impl TodoEntry {
    pub fn toggle_completion(&mut self) {
        self.completed = !self.completed;
    }
}

impl Todos {
    pub fn load_todos(key: &str) -> Self {
        match LocalStorage::get(key) {
            Ok(todos) => todos,
            Err(_) => Self {
                entries: Vec::new(),
                filter: Filter::All,
            }
        }
    }

    pub fn save_todos(&self, key: &str) -> Result<(), StorageError> {
        LocalStorage::set(key, self)
    }

    pub fn new_entry(&mut self, description: String) {
        self.entries.push(TodoEntry {
            id: uuid::Uuid::new_v4(),
            description,
            completed: false,
        });
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn completed_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.completed)
            .count()
    }

    pub fn is_all_completed(&self) -> bool {
        let has_active_todo = self
            .entries
            .iter()
            .any(|e| !e.completed);
        !has_active_todo
    }

    pub fn clear_completed(&mut self) {
        self
            .entries
            .retain(|e| !e.completed);
    }

    pub fn get_entry_by_id_mut(&mut self, id: &uuid::Uuid) -> Option<&mut TodoEntry> {
        self.entries.iter_mut().find(|e| e.id == *id)
    }

    pub fn get_filtered_entries(&self) -> impl Iterator<Item = &TodoEntry> {
        let filter = self.filter;
        self.entries.iter().filter(move |e| filter.fits(e))
    }

    pub fn get_filtered_entries_mut(&mut self) -> impl Iterator<Item = &mut TodoEntry> {
        let filter = self.filter;
        self.entries.iter_mut().filter(move |e| filter.fits(e))
    }

    pub fn get_filtered_entry_mut(&mut self, index: usize) -> Option<&mut TodoEntry> {
        self.get_filtered_entries_mut().nth(index)
    }

    pub fn set_completed_for_all(&mut self, completed: bool) {
        for entry in &mut self.get_filtered_entries_mut() {
            entry.completed = completed;
        }
    }

    pub fn remove_by_id(&mut self, id: &uuid::Uuid) {
        self.entries.retain(|e| e.id != *id);
    }

    pub fn remove_by_index(&mut self, index: usize) {
        let actual_index = match self.entries
            .iter()
            .enumerate()
            .filter(|e| self.filter.fits(e.1))
            .nth(index) {
                None => return,
                Some(ev) => ev.0,
        };
        self.entries.remove(actual_index);
    }
}
