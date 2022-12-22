use std::{collections::HashMap, fs, io, path::Path};

use sql_parse::{
    AlterSpecification, CreateDefinition, DataTypeProperty, Identifier, IndexCol, IndexType,
    ParseOptions, SQLDialect, Statement, Type,
};

use crate::database::{
    column::{Column, ColumnType},
    keys::{ForeignKey, Key},
    table::Table,
};

pub(crate) fn parse_file<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, Table>> {
    let contents = fs::read_to_string(path);

    contents.map(|s| {
        let options = ParseOptions::new().dialect(SQLDialect::MariaDB);
        let statements = sql_parse::parse_statements(s.as_str(), &mut Vec::new(), &options);

        parse_statements(statements)
    })
}

fn parse_statements(statements: Vec<Statement>) -> HashMap<String, Table> {
    let mut tables = HashMap::new();

    for statement in statements {
        parse_statement(statement, &mut tables);
    }

    tables
}

fn parse_statement(statement: Statement, tables: &mut HashMap<String, Table>) {
    match statement {
        Statement::CreateTable(value) => {
            let name = value.identifier.value.to_string();

            let mut column_names = Vec::new();
            let mut columns = HashMap::new();
            for create_definition in value.create_definitions {
                let column = parse_create_definition(create_definition);

                column_names.push(column.name.clone());
                columns.insert(column.name.clone(), column);
            }

            let table = Table::new(
                name.clone(),
                column_names,
                columns,
                Vec::new(),
                HashMap::new(),
                None,
                Vec::new(),
                HashMap::new(),
            );

            tables.insert(name, table);
        }
        Statement::AlterTable(value) => {
            let table_name = value.table.value.to_string();

            for alter_specification in value.alter_specifications {
                parse_alter_specification(alter_specification, &table_name, tables)
            }
        }
        _ => (),
    }
}

fn parse_create_definition(create_definition: CreateDefinition) -> Column {
    match create_definition {
        CreateDefinition::ColumnDefinition {
            identifier,
            data_type,
        } => {
            let name = identifier.value.to_string();

            let type_ = parse_type(data_type.type_);

            let mut nullable = true;
            let mut auto_increment = false;
            let mut comment = "".into();

            for property in data_type.properties {
                match property {
                    DataTypeProperty::NotNull(_) => nullable = false,
                    DataTypeProperty::AutoIncrement(_) => auto_increment = true,
                    DataTypeProperty::Comment(value) => comment = value.value.to_string(),
                    _ => (),
                }
            }

            Column::new(name, type_, nullable, auto_increment, comment)
        }
    }
}

fn parse_type(type_: Type) -> ColumnType {
    match type_ {
        sql_parse::Type::Boolean => ColumnType::Boolean,
        sql_parse::Type::TinyInt(_)
        | sql_parse::Type::SmallInt(_)
        | sql_parse::Type::Integer(_)
        | sql_parse::Type::Int(_)
        | sql_parse::Type::BigInt(_) => ColumnType::Int,
        sql_parse::Type::Char(_)
        | sql_parse::Type::VarChar(_)
        | sql_parse::Type::TinyText(_)
        | sql_parse::Type::MediumText(_)
        | sql_parse::Type::Text(_)
        | sql_parse::Type::LongText(_) => ColumnType::String,
        sql_parse::Type::Enum(value) => {
            ColumnType::Enum(value.into_iter().map(|s| s.value.into()).collect())
        }
        sql_parse::Type::Float8 | sql_parse::Type::Float(_) | sql_parse::Type::Double(_) => {
            ColumnType::Float
        }
        sql_parse::Type::DateTime(_)
        | sql_parse::Type::Timestamp(_)
        | sql_parse::Type::Time(_)
        | sql_parse::Type::Date => ColumnType::String,
        _ => unimplemented!(),
    }
}

fn parse_alter_specification(
    alter_specification: AlterSpecification,
    table_name: &String,
    tables: &mut HashMap<String, Table>,
) {
    match alter_specification {
        AlterSpecification::AddIndex {
            add_span: _,
            index_type,
            if_not_exists: _,
            name,
            constraint: _,
            cols,
            index_options: _,
        } => parse_add_index(index_type, cols, name, table_name, tables),
        AlterSpecification::Modify {
            modify_span: _,
            if_exists: _,
            col,
            definition,
        } => {
            let column_name = col.value.to_string();

            let mut column = tables
                .get_mut(table_name)
                .unwrap()
                .columns
                .get_mut(&column_name)
                .unwrap();

            column.type_ = parse_type(definition.type_);

            for property in definition.properties {
                match property {
                    DataTypeProperty::NotNull(_) => column.nullable = false,
                    DataTypeProperty::AutoIncrement(_) => column.auto_increment = true,
                    DataTypeProperty::Comment(value) => column.comment = value.value.to_string(),
                    _ => (),
                }
            }
        }
        AlterSpecification::AddForeignKey {
            add_span: _,
            constraint,
            foreign_key_span: _,
            if_not_exists: _,
            name: _,
            cols,
            references_span: _,
            references_table,
            references_cols,
            ons: _,
        } => {
            let name = constraint.unwrap().1.unwrap().value.to_string();

            let columns = cols.into_iter().map(|c| c.name.value.into()).collect();

            let foreign_table = references_table.value.to_string();

            let foreign_columns = references_cols
                .into_iter()
                .map(|c| c.value.into())
                .collect();

            let foreign_key = ForeignKey::new(
                name,
                table_name.clone(),
                columns,
                foreign_table,
                foreign_columns,
            );

            let table = tables.get_mut(table_name).unwrap();
            table.foreign_key_names.push(foreign_key.name.clone());
            table
                .foreign_keys
                .insert(foreign_key.name.clone(), foreign_key);
        }
        _ => unimplemented!(),
    }
}

fn parse_add_index(
    index_type: IndexType,
    cols: Vec<IndexCol>,
    name: Option<Identifier>,
    table_name: &String,
    tables: &mut HashMap<String, Table>,
) {
    let columns: Vec<String> = cols.into_iter().map(|c| c.name.value.into()).collect();
    let name = name.map(|name| name.value.to_string()).unwrap_or("".into());

    let unique = match index_type {
        sql_parse::IndexType::Primary(_) | sql_parse::IndexType::Unique(_) => true,
        sql_parse::IndexType::Index(_) => false,
        _ => {
            println!("Encountered unkown column type");
            false
        }
    };

    let key = Key::new(name, unique, table_name.clone(), columns);

    let table = tables.get_mut(table_name).unwrap();

    if matches!(index_type, sql_parse::IndexType::Primary(_)) {
        table.primary_key = Some(key);
    } else {
        table.key_names.push(key.name.clone());
        table.keys.insert(key.name.clone(), key);
    }
}
