use convert_case::{Case, Casing};

use crate::database::{column::Column, table::Table};

use super::column::{write_full_comment, write_typed_variable, write_variable};

pub(crate) fn write_updater<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += &write_comment(table);

    buffer += "\tpublic static function update(";

    buffer += &write_parameters(table.iter_columns());

    buffer += "): ";
    buffer += &table.name.to_case(Case::Pascal);
    buffer += "\n\t{\n";

    buffer += &write_body(table);

    buffer += "\t}\n";

    buffer
}

fn write_comment<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += "\n\t/** Updates a `";
    buffer += &table.name.to_case(Case::Pascal);
    buffer += "` by the primary key.\n";

    for column in table.iter_columns() {
        buffer += "\t * @param ";
        buffer += &write_full_comment(column);
        buffer += "\n";
    }

    buffer += "\t * @return ";
    buffer += &table.name.to_case(Case::Pascal);
    buffer += " The updated object.\n";

    buffer += "\t */\n";

    buffer
}

fn write_parameters<'a>(mut columns: impl Iterator<Item = &'a Column>) -> String {
    let mut buffer = String::new();

    if let Some(column) = columns.next() {
        buffer += &write_typed_variable(column);
    }

    for column in columns {
        buffer += ", ";
        buffer += &write_typed_variable(column);
    }

    buffer
}

fn write_body<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += &write_query_params(table.iter_columns().collect());

    buffer += &write_query_prepare(table);
    buffer += "\t\t$sth->execute($params);\n\n";

    buffer += "\t\treturn Self::get(";
    let primary_key = table.primary_key.as_ref().unwrap();
    buffer += &write_object_arguments(primary_key.iter_columns(table));
    buffer += ");\n";

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

fn write_query_prepare<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += "\t\t$sth = getPDO()->prepare(\"UPDATE `";
    buffer += &table.name;
    buffer += "` SET ";

    let primary_key = table.primary_key.as_ref().unwrap();
    let non_primary_columns = table
        .iter_columns()
        .filter(|column| !primary_key.column_names.contains(&column.name));
    buffer += &write_query_update(non_primary_columns);

    buffer += " WHERE ";

    buffer += &write_query_where(primary_key.iter_columns(table));

    buffer += ";\");\n";

    buffer
}

fn write_query_update<'a>(mut columns: impl Iterator<Item = &'a Column>) -> String {
    let mut buffer = String::new();

    if let Some(column) = columns.next() {
        buffer += "`";
        buffer += &column.name;
        buffer += "` = :";
        buffer += &column.name;
    }

    for column in columns {
        buffer += ", `";
        buffer += &column.name;
        buffer += "` = :";
        buffer += &column.name;
    }

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

fn write_object_arguments<'a>(mut columns: impl Iterator<Item = &'a Column>) -> String {
    let mut buffer = String::new();

    if let Some(column) = columns.next() {
        buffer += &write_variable(column);
    }

    for column in columns {
        buffer += ", ";
        buffer += &write_variable(column);
    }

    buffer
}
