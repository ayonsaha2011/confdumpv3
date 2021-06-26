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

mod services;

use tokio::io::AsyncBufReadExt;
use tokio::stream::StreamExt;
use crate::services::{args, query_table, genrate_output};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let arguments = args::Args::new();
    let mut tables_list = Vec::<String>::new();

    if arguments.table.is_some() {
        tables_list.push(arguments.table.unwrap().to_string());
    } else if arguments.tables.is_some() {
        let tables_from_path = arguments.tables.unwrap().to_string();
        println!("tables_from_path {}", &tables_from_path);

        let file = tokio::fs::File::open(tables_from_path).await?;
        let buffered = tokio::io::BufReader::new(file);
        let mut lines = buffered.lines();

        while let Some(line) = lines.next().await {
            let line = line?;
            tables_list.push(line);
        }
    }

    if tables_list.len() > 0 {
        let results = query_table::query(tables_list).await;
        match results {
            Ok(query_results) => genrate_output::genrate(arguments.output_format, arguments.output_file, query_results),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}