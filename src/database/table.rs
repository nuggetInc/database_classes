use std::collections::HashMap;

use super::{
    column::Column,
    keys::{ForeignKey, Key},
};

#[derive(Debug)]
pub(crate) struct Table {
    pub(crate) name: String,
    pub(crate) column_names: Vec<String>,
    pub(crate) columns: HashMap<String, Column>,
    pub(crate) key_names: Vec<String>,
    pub(crate) keys: HashMap<String, Key>,
    pub(crate) primary_key: Option<Key>,
    pub(crate) foreign_key_names: Vec<String>,
    pub(crate) foreign_keys: HashMap<String, ForeignKey>,
}

impl Table {
    pub(crate) fn new(
        name: String,
        column_names: Vec<String>,
        columns: HashMap<String, Column>,
        key_names: Vec<String>,
        keys: HashMap<String, Key>,
        primary_key: Option<Key>,
        foreign_key_names: Vec<String>,
        foreign_keys: HashMap<String, ForeignKey>,
    ) -> Self {
        Self {
            name,
            column_names,
            columns,
            key_names,
            keys,
            primary_key,
            foreign_key_names,
            foreign_keys,
        }
    }

    pub(crate) fn iter_columns(&self) -> impl Iterator<Item = &Column> {
        self.column_names
            .iter()
            .map(|column_name| &self.columns[column_name])
    }

    pub(crate) fn iter_keys(&self) -> impl Iterator<Item = &Key> {
        self.key_names.iter().map(|key_name| &self.keys[key_name])
    }

    pub(crate) fn iter_foreign_keys(&self) -> impl Iterator<Item = &ForeignKey> {
        self.foreign_key_names
            .iter()
            .map(|foreign_key_name| &self.foreign_keys[foreign_key_name])
    }

    pub(crate) fn iter_column_names(&self) -> impl Iterator<Item = &String> {
        self.column_names.iter()
    }

    pub(crate) fn iter_key_names(&self) -> impl Iterator<Item = &String> {
        self.key_names.iter()
    }

    pub(crate) fn iter_foreign_key_names(&self) -> impl Iterator<Item = &String> {
        self.foreign_key_names.iter()
    }
}
