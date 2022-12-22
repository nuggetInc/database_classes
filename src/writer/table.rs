use convert_case::{Case, Casing};

use crate::database::{
    column::{Column, ColumnType},
    table::Table,
};

use super::{
    column::{write_full_comment, write_typed_variable},
    creater::write_creater,
    deleter::write_deleter,
    foreign_getters::write_foreign_getters,
    getters::write_getters,
    updater::write_updater,
};

pub(crate) fn write_table<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += &write_table_definition(&table.name);

    buffer += &write_constructor_comment(table.iter_columns());
    buffer += &write_constructor(table.iter_columns());

    buffer += &write_foreign_getters(table, table.iter_foreign_keys());

    buffer += &write_getters(table);

    buffer += &write_creater(table);

    if table.primary_key.is_some() {
        buffer += &write_updater(table);
        buffer += &write_deleter(table);
    }

    buffer += "}\n";

    buffer += &write_enums(table);

    buffer
}

fn write_table_definition<'a>(table_name: &'a String) -> String {
    let mut buffer = String::new();

    buffer += "/** Database class for the `";
    buffer += table_name;
    buffer += "` table */\n";

    buffer += "class ";
    buffer += &table_name.to_case(Case::Pascal);
    buffer += "\n";
    buffer += "{\n";

    buffer
}

fn write_constructor_comment<'a>(columns: impl Iterator<Item = &'a Column>) -> String {
    let mut buffer = String::new();

    buffer += "\t/**\n";
    for column in columns {
        buffer += "\t * @param ";
        buffer += &write_full_comment(column);
        buffer += "\n";
    }
    buffer += "\t */\n";

    buffer
}

fn write_constructor<'a>(columns: impl Iterator<Item = &'a Column>) -> String {
    let mut buffer = String::new();

    buffer += "\tprivate function __construct(\n";
    for column in columns {
        buffer += "\t\tpublic readonly ";
        buffer += &write_typed_variable(column);
        buffer += ",\n";
    }
    buffer += "\t) {\n\t}\n";

    buffer
}

fn write_enums<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    for column in table.iter_columns() {
        let values = match &column.type_ {
            ColumnType::Enum(values) => values,
            _ => continue,
        };

        let name = column.name.to_case(Case::Pascal);

        buffer += &write_enum(&name, values);
    }

    buffer
}

fn write_enum<'a>(name: &'a String, values: &'a Vec<String>) -> String {
    let mut buffer = String::new();

    buffer += "\nenum ";
    buffer += &name;
    buffer += "Enum: string\n";
    buffer += "{\n";

    for value in values {
        buffer += "\tcase ";
        buffer += &value.to_case(Case::UpperSnake);
        buffer += " = \"";
        buffer += value;
        buffer += "\";\n";
    }

    buffer += "}\n";

    buffer
}
