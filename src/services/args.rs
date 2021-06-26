/*
 * Confdump-Agent - Dump static and runtime system configuration
 * Copyright (C) 2009-2012  Straton IT, SAS
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use clap::{Arg, App};

#[derive(Debug, Clone)]
pub struct Args {
    pub(crate) table: Option<String>,
    pub(crate) tables: Option<String>,
    pub(crate) output_format: String,
    pub(crate) output_file: Option<String>
}

impl Args {
    pub fn new() -> Self {
        let app = App::new("Confdump-Agent")
            .version("2.0.0")
            .about("Dump static and runtime system configuration");

        let table_option = Arg::with_name("table")
            .long("table")
            .takes_value(true)
            .help("tables to query (can be specified multiple times); format: [dumper.]table");

        let tables_option = Arg::with_name("tables-from")
            .long("tables-from")
            .takes_value(true)
            .help("read a list of tables to query from this file");

        let output_format_option = Arg::with_name("output-format")
            .long("output-format")
            .default_value("text")
            .takes_value(true)
            .help("sets output format to one of: text, xml --output-file arg    output to a file instead of standard output");


        let output_file_option = Arg::with_name("output-file")
            .long("output-file")
            .takes_value(true)
            .help("output to a file instead of standard output");

        let app = app.arg(table_option)
            .arg(tables_option)
            .arg(output_format_option)
            .arg(output_file_option);
        let matches = app.get_matches();
        let table = match matches.value_of("table") {
            Some(t) => Some(t.to_string()),
            None => None
        };
        let tables_from = match matches.value_of("tables-from") {
            Some(t) => Some(t.to_string()),
            None => None
        };
        let output_file = match matches.value_of("output-file") {
            Some(t) => Some(t.to_string()),
            None => None
        };
        let output_format = matches.value_of("output-format").unwrap().to_string();

        Args { table, tables: tables_from, output_format, output_file }
    }
}