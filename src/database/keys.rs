use super::{column::Column, table::Table};

#[derive(Debug)]
pub(crate) struct Key {
    pub(crate) name: String,
    pub(crate) unique: bool,
    pub(crate) table_name: String,
    pub(crate) column_names: Vec<String>,
}

impl Key {
    pub(crate) fn new(
        name: String,
        unique: bool,
        table_name: String,
        column_names: Vec<String>,
    ) -> Self {
        Self {
            name,
            unique,
            table_name,
            column_names,
        }
    }

    pub(crate) fn iter_columns<'a>(&'a self, table: &'a Table) -> impl Iterator<Item = &'a Column> {
        if table.name != self.table_name {
            panic!("Table doesn't match with key");
        }

        self.column_names
            .iter()
            .map(|column_name| &table.columns[column_name])
    }

    pub(crate) fn iter_column_names<'a>(&'a self) -> impl Iterator<Item = &'a String> {
        self.column_names.iter()
    }
}

#[derive(Debug)]
pub(crate) struct ForeignKey {
    pub(crate) name: String,
    pub(crate) table_name: String,
    pub(crate) column_names: Vec<String>,
    pub(crate) foreign_table_name: String,
    pub(crate) foreign_column_names: Vec<String>,
}

impl ForeignKey {
    pub(crate) fn new(
        name: String,
        table_name: String,
        column_names: Vec<String>,
        foreign_table_name: String,
        foreign_column_names: Vec<String>,
    ) -> Self {
        Self {
            name,
            table_name,
            column_names,
            foreign_table_name,
            foreign_column_names,
        }
    }

    pub(crate) fn iter_columns<'a>(&'a self, table: &'a Table) -> impl Iterator<Item = &'a Column> {
        if table.name != self.table_name {
            panic!("Table doesn't match with key");
        }

        self.column_names
            .iter()
            .map(|column_name| &table.columns[column_name])
    }

    pub(crate) fn iter_column_names<'a>(&'a self) -> impl Iterator<Item = &'a String> {
        self.column_names.iter()
    }

    pub(crate) fn iter_foreign_columns<'a>(
        &'a self,
        table: &'a Table,
    ) -> impl Iterator<Item = &'a Column> {
        if table.name != self.foreign_table_name {
            panic!("Table doesn't match with key");
        }

        self.foreign_column_names
            .iter()
            .map(|column_name| &table.columns[column_name])
    }

    pub(crate) fn iter_foreign_column_names<'a>(&'a self) -> impl Iterator<Item = &'a String> {
        self.foreign_column_names.iter()
    }
}
