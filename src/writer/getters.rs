use convert_case::{Case, Casing};

use crate::database::{
    column::{Column, ColumnType},
    keys::Key,
    table::Table,
};

use super::{
    column::{write_full_comment, write_typed_variable, write_variable},
    get_all,
};

pub(crate) fn write_getters<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    if let Some(primary_key) = &table.primary_key {
        buffer += &write_comment(table, primary_key);
        buffer += &write_getter(table, primary_key);
    }

    buffer += &get_all::write_getter(table);

    for key in table.iter_keys() {
        buffer += &write_comment(table, key);
        buffer += &write_getter(table, key);
    }

    buffer
}

fn write_getter<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    buffer += "\tpublic static function get";
    if !key.name.is_empty() {
        buffer += "By";
    }
    buffer += &key.name.to_case(Case::Pascal);
    buffer += "(";

    buffer += &write_parameters(
        key.iter_column_names()
            .map(|column_name| &table.columns[column_name]),
    );

    if key.unique {
        buffer += "): null|";
        buffer += &table.name.to_case(Case::Pascal);
    } else {
        buffer += "): array";
    }
    buffer += "\n\t{\n";

    buffer += &write_body(table, key);

    buffer += "\t}\n";

    buffer
}

fn write_comment<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    buffer += "\n\t/** ";

    if key.name.is_empty() {
        buffer += "Gets a `";
        buffer += &table.name.to_case(Case::Pascal);
        buffer += "` by the primary key.\n";
    } else if key.unique {
        buffer += "Gets a `";
        buffer += &table.name.to_case(Case::Pascal);
        buffer += "` by the `";
        buffer += &key.name;
        buffer += "` key.\n";
    } else {
        buffer += "Gets several `";
        buffer += &table.name.to_case(Case::Pascal);
        buffer += "` by the `";
        buffer += &key.name;
        buffer += "` key.\n";
    }

    for column in key.iter_column_names() {
        buffer += "\t * @param ";
        buffer += &write_full_comment(&table.columns[column]);
        buffer += "\n";
    }

    if key.unique {
        buffer += "\t * @return null|";
        buffer += &table.name.to_case(Case::Pascal);
        buffer += " The corresponding object";

        buffer += ", `null` when the row doesn't exist.\n";
    } else {
        buffer += "\t * @return array zero or more corresponding objects.\n";
    }

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

fn write_body<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    buffer += &write_query_params(&key.column_names);

    buffer += &write_query_prepare(table, key);
    buffer += "\t\t$sth->execute($params);\n\n";

    if key.unique {
        buffer += "\t\tif ($row = $sth->fetch())\n";

        buffer += "\t\t\treturn new ";
        buffer += &table.name.to_case(Case::Pascal);
        buffer += "(";
        buffer += &write_object_arguments(table.iter_columns(), &key.column_names);
        buffer += ");\n\n";

        buffer += "\t\treturn null;\n";
    } else {
        buffer += "\t\t$";
        buffer += &table.name.to_case(Case::Camel);
        buffer += " = array();\n\n";
        buffer += "\t\twhile ($row = $sth->fetch())\n";

        buffer += "\t\t\t$";
        buffer += &table.name.to_case(Case::Camel);
        buffer += "[] = new ";
        buffer += &table.name.to_case(Case::Pascal);
        buffer += "(";
        buffer += &write_object_arguments(table.iter_columns(), &key.column_names);
        buffer += ");\n\n";

        buffer += "\t\treturn $";
        buffer += &table.name.to_case(Case::Camel);
        buffer += ";\n";
    }

    buffer
}

fn write_query_params<'a>(column_names: &'a Vec<String>) -> String {
    let mut buffer = String::new();

    if column_names.len() == 1 {
        let column_name = &column_names[0];

        buffer += "\t\t$params = array(\":";
        buffer += column_name;
        buffer += "\" => $";
        buffer += &column_name.to_case(Case::Camel);
        buffer += ");\n";
    } else if column_names.len() > 1 {
        buffer += "\t\t$params = array(\n";
        for column_name in column_names {
            buffer += "\t\t\t\":";
            buffer += column_name;
            buffer += "\" => $";
            buffer += &column_name.to_case(Case::Camel);
            buffer += ",\n";
        }
        buffer += "\t\t);\n";
    }

    buffer
}

fn write_query_prepare<'a>(table: &'a Table, key: &'a Key) -> String {
    let mut buffer = String::new();

    buffer += "\t\t$sth = getPDO()->prepare(\"SELECT ";

    buffer += &write_query_select(table.iter_column_names(), &key.column_names);

    buffer += " FROM `";
    buffer += &table.name;
    buffer += "` WHERE ";

    buffer += &write_query_where(key.iter_column_names());

    if key.unique {
        buffer += " LIMIT 1;\");\n";
    } else {
        buffer += ";\");\n";
    }

    buffer
}

fn write_query_select<'a>(
    column_names: impl Iterator<Item = &'a String>,
    parameter_column_names: &'a Vec<String>,
) -> String {
    let mut buffer = String::new();

    let mut column_names =
        column_names.filter(|column_name| !parameter_column_names.contains(column_name));

    if let Some(column_name) = column_names.next() {
        buffer += "`";
        buffer += column_name;
        buffer += "`";
    }

    for column_name in column_names {
        buffer += ", `";
        buffer += column_name;
        buffer += "`";
    }

    buffer
}

fn write_query_where<'a>(mut column_names: impl Iterator<Item = &'a String>) -> String {
    let mut buffer = String::new();

    if let Some(column_name) = column_names.next() {
        buffer += "`";
        buffer += column_name;
        buffer += "` <=> :";
        buffer += column_name;
    }

    for column_name in column_names {
        buffer += " AND `";
        buffer += column_name;
        buffer += "` <=> :";
        buffer += column_name;
    }

    buffer
}

fn write_object_arguments<'a>(
    mut columns: impl Iterator<Item = &'a Column>,
    parameter_column_names: &'a Vec<String>,
) -> String {
    let mut buffer = String::new();

    if let Some(column) = columns.next() {
        buffer += &write_object_argument(column, parameter_column_names);
    }

    for column in columns {
        buffer += ", ";
        buffer += &write_object_argument(column, parameter_column_names);
    }

    buffer
}

fn write_object_argument<'a>(
    column: &'a Column,
    parameter_column_names: &'a Vec<String>,
) -> String {
    let mut buffer = String::new();

    if parameter_column_names.contains(&column.name) {
        buffer += &write_variable(&column);
    } else {
        match column.type_ {
            ColumnType::Enum(_) => {
                buffer += &column.name.to_case(Case::Pascal);
                buffer += "Enum::from($row[\"";
                buffer += &column.name;
                buffer += "\"])";
            }
            _ => {
                buffer += "$row[\"";
                buffer += &column.name;
                buffer += "\"]";
            }
        }
    }

    buffer
}
