use std::{
    env,
    fs::{create_dir_all, File},
    io::Write,
    time::Instant,
};

use convert_case::{Case, Casing};

use crate::parsers::parse_file;
use crate::writer::table::write_table;

mod database;
mod parsers;
mod writer;

fn main() {
    let mut args = env::args();
    args.next();

    let Some(file_name) = args.next() else {
        println!("Expected file");
        std::process::exit(1);
    };

    let tables = parse_file(file_name).unwrap();
    create_dir_all("php").unwrap();

    for (table_name, table) in &tables {
        let file_name = format!("php/{}.php", table_name.to_case(Case::Camel));
        let mut file = File::create(file_name).unwrap();

        let time = Instant::now();

        let mut buffer = write_file_start();
        buffer += &write_table(table);

        println!("Generated {} in {:?}", table_name, time.elapsed());

        file.write(buffer.replace("\t", "    ").as_bytes()).unwrap();
    }
}

fn write_file_start() -> String {
    "<?php\n\ndeclare(strict_types=1);\n\n".into()
}
