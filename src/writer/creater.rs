use convert_case::{Case, Casing};

use crate::database::{column::Column, table::Table};

use super::column::{write_full_comment, write_typed_variable, write_variable};

pub(crate) fn write_creater<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += &write_comment(table);

    buffer += "\tpublic static function register(";

    buffer += &write_parameters(table);

    buffer += "): ";
    buffer += &table.name.to_case(Case::Pascal);
    buffer += "\n\t{\n";

    buffer += &write_body(table);

    buffer += "\t}\n";

    buffer
}

fn write_comment<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += "\n\t/** Create a `";
    buffer += &table.name.to_case(Case::Pascal);
    buffer += "`\n";

    for column in table.iter_columns().filter(|column| !column.auto_increment) {
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

fn write_parameters<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    let mut columns = table.iter_columns().filter(|column| !column.auto_increment);

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

    buffer += &write_query_params(table);

    buffer += &write_query_prepare(table);
    buffer += "\t\t$sth->execute($params);\n\n";

    buffer += "\t\treturn Self::get(";
    buffer += &write_object_arguments(table.iter_columns());
    buffer += ");\n";

    buffer
}

fn write_query_params<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    let columns = table
        .iter_columns()
        .filter(|column| !column.auto_increment)
        .collect::<Vec<_>>();

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

    buffer += "\t\t$sth = getPDO()->prepare(\"INSERT INTO `";
    buffer += &table.name;
    buffer += "` (";

    buffer += &write_query_create(table);

    buffer += ") VALUES (";

    buffer += &write_query_values(table);

    buffer += ");\");\n";

    buffer
}

fn write_query_create<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    let mut columns = table.iter_columns().filter(|column| !column.auto_increment);

    if let Some(column) = columns.next() {
        buffer += "`";
        buffer += &column.name;
        buffer += "`";
    }

    for column in columns {
        buffer += ", `";
        buffer += &column.name;
        buffer += "`";
    }

    buffer
}

fn write_query_values<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    let mut columns = table.iter_columns().filter(|column| !column.auto_increment);

    if let Some(column) = columns.next() {
        buffer += ":";
        buffer += &column.name;
    }

    for column in columns {
        buffer += ", :";
        buffer += &column.name;
    }

    buffer
}
fn write_object_arguments<'a>(
    mut columns: impl Iterator<Item = &'a Column>,
    // parameter_column_names: &'a Vec<String>,
) -> String {
    let mut buffer = String::new();

    if let Some(column) = columns.next() {
        if column.auto_increment {
            buffer += "(int)getPDO()->lastInsertId()";
        } else {
            buffer += &write_variable(&column);
        }
    }

    for column in columns {
        buffer += ", ";

        if column.auto_increment {
            buffer += "(int)getPDO()->lastInsertId()";
        } else {
            buffer += &write_variable(&column);
        }
    }

    buffer
}
