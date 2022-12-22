use convert_case::{Case, Casing};

use crate::database::{column::Column, keys::Key, table::Table};

use super::column::{write_full_comment, write_typed_variable};

pub(crate) fn write_deleter<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    let primary_key = table.primary_key.as_ref().unwrap();
    buffer += &write_comment(table, primary_key);

    buffer += "\tpublic static function delete(";

    buffer += &write_parameters(table, &primary_key);

    buffer += "): void\n";
    buffer += "\t{\n";

    buffer += &write_body(table, &primary_key);

    buffer += "\t}\n";

    buffer
}

fn write_comment<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    buffer += "\n\t/** Deletes a `";
    buffer += &table.name.to_case(Case::Pascal);
    buffer += "` by the primary key.\n";

    for column in key.iter_columns(table) {
        buffer += "\t * @param ";
        buffer += &write_full_comment(column);
        buffer += "\n";
    }

    buffer += "\t */\n";

    buffer
}

fn write_parameters<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    let mut columns = key.iter_columns(table);

    if let Some(column) = columns.next() {
        buffer += &write_typed_variable(column);
    }

    for column in columns {
        buffer += ", ";
        buffer += &write_typed_variable(column);
    }

    buffer
}

fn write_body<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    buffer += &write_query_params(key.iter_columns(table).collect());

    buffer += &write_query_prepare(table, key);
    buffer += "\t\t$sth->execute($params);\n";

    buffer
}

fn write_query_params<'a>(columns: Vec<&'a Column>) -> String {
    let mut buffer = String::new();

    if columns.len() == 1 {
        let column = columns[0];

        buffer += "\t\t$params = array(\":";
        buffer += &column.name;
        buffer += "\" => $";
        buffer += &column.name.to_case(Case::Camel);
        buffer += ");\n";
    } else if columns.len() > 1 {
        buffer += "\t\t$params = array(\n";
        for column in columns {
            buffer += "\t\t\t\":";
            buffer += &column.name;
            buffer += "\" => $";
            buffer += &column.name.to_case(Case::Camel);
            buffer += ",\n";
        }
        buffer += "\t\t);\n";
    }

    buffer
}

fn write_query_prepare<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    buffer += "\t\t$sth = getPDO()->prepare(\"DELETE FROM `";
    buffer += &table.name;
    buffer += "` WHERE ";

    buffer += &write_query_where(key.iter_columns(table));

    buffer += ";\");\n";

    buffer
}

fn write_query_where<'a>(mut columns: impl Iterator<Item = &'a Column>) -> String {
    let mut buffer = String::new();

    if let Some(column) = columns.next() {
        buffer += "`";
        buffer += &column.name;
        if column.nullable {
            buffer += "` <=> :";
        } else {
            buffer += "` = :";
        }
        buffer += &column.name;
    }

    for column in columns {
        buffer += " AND `";
        buffer += &column.name;
        if column.nullable {
            buffer += "` <=> :";
        } else {
            buffer += "` = :";
        }
        buffer += &column.name;
    }

    buffer
}
