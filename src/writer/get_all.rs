use convert_case::{Case, Casing};

use crate::database::{
    column::{Column, ColumnType},
    table::Table,
};

pub(crate) fn write_getter<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += &write_comment(table);

    buffer += "\tpublic static function getAll(): array\n";
    buffer += "\t{\n";

    buffer += &write_body(table);

    buffer += "\t}\n";

    buffer
}

fn write_comment<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += "\n\t/** ";

    buffer += "Gets all rows in `";
    buffer += &table.name;
    buffer += "`.\n";

    buffer += "\t * @return array all objects in `";
    buffer += &table.name;
    buffer += "`.\n";

    buffer += "\t */\n";

    buffer
}

fn write_body<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += &write_query_prepare(table);
    buffer += "\t\t$sth->execute();\n\n";

    buffer += "\t\t$";
    buffer += &table.name.to_case(Case::Camel);
    buffer += " = array();\n\n";
    buffer += "\t\twhile ($row = $sth->fetch())\n";

    buffer += "\t\t\t$";
    buffer += &table.name.to_case(Case::Camel);
    buffer += "[] = new ";
    buffer += &table.name.to_case(Case::Pascal);
    buffer += "(";
    buffer += &write_object_arguments(table);
    buffer += ");\n\n";

    buffer += "\t\treturn $";
    buffer += &table.name.to_case(Case::Camel);
    buffer += ";\n";

    buffer
}

fn write_query_prepare<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    buffer += "\t\t$sth = getPDO()->prepare(\"SELECT * FROM `";
    buffer += &table.name;
    buffer += "`;\");\n";

    buffer
}

fn write_object_arguments<'a>(table: &'a Table) -> String {
    let mut buffer = String::new();

    let mut columns = table.iter_columns();

    if let Some(column) = columns.next() {
        buffer += &write_object_argument(column);
    }

    for column in columns {
        buffer += ", ";
        buffer += &write_object_argument(column);
    }

    buffer
}

fn write_object_argument<'a>(column: &'a Column) -> String {
    let mut buffer = String::new();

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

    buffer
}
