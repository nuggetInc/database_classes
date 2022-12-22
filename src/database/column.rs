use sql_parse::Type;

#[derive(Debug)]
pub(crate) struct Column {
    pub(crate) name: String,
    pub(crate) type_: ColumnType,
    pub(crate) nullable: bool,
    pub(crate) auto_increment: bool,
    pub(crate) comment: String,
}

impl Column {
    pub(crate) fn new(
        name: String,
        type_: ColumnType,
        nullable: bool,
        auto_increment: bool,
        comment: String,
    ) -> Self {
        Self {
            name,
            type_,
            nullable,
            comment,
            auto_increment,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ColumnType {
    Boolean,
    Int,
    Float,
    String,
    Enum(Vec<String>),
}

impl From<Type<'_>> for ColumnType {
    fn from(value: Type) -> Self {
        match value {
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
}
