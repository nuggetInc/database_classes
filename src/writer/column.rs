use convert_case::{Case, Casing};

use crate::database::column::{Column, ColumnType};

pub(crate) fn write_full_comment<'a>(column: &'a Column) -> String {
    let mut buffer = String::new();

    if column.nullable {
        buffer += "null|";
    }
    match column.type_ {
        ColumnType::Boolean => buffer += "bool",
        ColumnType::Int => buffer += "int",
        ColumnType::Float => buffer += "float",
        ColumnType::String => buffer += "string",
        ColumnType::Enum(_) => {
            buffer += &column.name.to_case(Case::Pascal);
            buffer += "Enum";
        }
    };

    buffer += " $";
    buffer += &column.name.to_case(Case::Camel);

    buffer += " ";
    buffer += &column.comment;

    buffer
}

pub(crate) fn write_typed_variable<'a>(column: &'a Column) -> String {
    let mut buffer = String::new();

    if column.nullable {
        buffer += "null|";
    }
    match column.type_ {
        ColumnType::Boolean => buffer += "bool",
        ColumnType::Int => buffer += "int",
        ColumnType::Float => buffer += "float",
        ColumnType::String => buffer += "string",
        ColumnType::Enum(_) => {
            buffer += &column.name.to_case(Case::Pascal);
            buffer += "Enum";
        }
    };

    buffer += " $";
    buffer += &column.name.to_case(Case::Camel);

    buffer
}

pub(crate) fn write_variable<'a>(column: &'a Column) -> String {
    let mut buffer = String::new();

    buffer += "$";
    buffer += &column.name.to_case(Case::Camel);

    buffer
}
