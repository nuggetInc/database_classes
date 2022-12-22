use convert_case::{Case, Casing};

use crate::database::{keys::ForeignKey, table::Table};

pub(crate) fn write_foreign_getters<'a>(
    table: &'a Table,
    foreign_keys: impl Iterator<Item = &'a ForeignKey>,
) -> String {
    let mut buffer = String::new();

    for foreign_key in foreign_keys {
        let nullable_column_names = foreign_key
            .iter_column_names()
            .filter(|column_name| table.columns[*column_name].nullable)
            .collect::<Vec<_>>();

        buffer += &write_comment(table, foreign_key);

        buffer += "\tpublic function get";
        buffer += &foreign_key.name.to_case(Case::Pascal);

        if !nullable_column_names.is_empty() {
            buffer += "(): false|";
        } else {
            buffer += "(): ";
        }

        buffer += &foreign_key.foreign_table_name.to_case(Case::Pascal);

        buffer += "\n\t{\n";

        if !nullable_column_names.is_empty() {
            buffer += "\t\tif (!isset(";

            buffer += &write_arguments(nullable_column_names.into_iter());

            buffer += "))\n";
            buffer += "\t\t\treturn false;\n\n";
        }

        buffer += "\t\treturn ";
        buffer += &foreign_key.foreign_table_name.to_case(Case::Pascal);
        buffer += "::get(";
        buffer += &write_arguments(foreign_key.iter_column_names());
        buffer += ");\n\t}\n";
    }

    buffer
}

fn write_arguments<'a>(mut arguments: impl Iterator<Item = &'a String>) -> String {
    let mut buffer = String::new();

    if let Some(argument) = arguments.next() {
        buffer += "$this->";
        buffer += argument;
    }

    for argument in arguments {
        buffer += ", $this->";
        buffer += argument;
    }

    buffer
}

fn write_comment<'a>(table: &'a Table, foreign_key: &'a ForeignKey) -> String {
    let mut buffer = String::new();

    let nullable = foreign_key
        .iter_columns(table)
        .any(|column| column.nullable);

    buffer += "\n\t/** Gets the associated `";
    buffer += &table.name;
    buffer += "` by the `";
    buffer += &foreign_key.name;
    buffer += "` key.\n";

    buffer += "\t * @return ";
    if nullable {
        buffer += "false|";
    }
    buffer += &table.name.to_case(Case::Pascal);
    buffer += " The corresponding object";

    if nullable {
        buffer += ", `false` when one of the keys columns equal `null`";
    }
    buffer += ".\n";

    buffer += "\t */\n";

    buffer
}
