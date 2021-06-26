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

use std::io::{Write, Error};
use std::fs::OpenOptions;
use crate::services::query_table::QueryResult;
use uuid::Uuid;
use chrono::prelude::*;

pub fn genrate(output_format: String, output_path: Option<String>, results: Vec<QueryResult>) {

    let outputs = match output_format.as_str() {
        "text" => genrate_text(results),
        "xml" => genrate_xml(results),
        "json" => genrate_json(results),
        _ => { println!("Input output format not equal any value"); Vec::<u8>::new() },
    };

    let outputs_str = String::from_utf8_lossy(&*outputs);
    if output_path.is_some() {
        let mut file = OpenOptions::new().create(true).append(true).open(output_path.unwrap()).expect(
            "cannot open file");
        writeln!(file, "{}", outputs_str).unwrap();
        /*for output in outputs {
            writeln!(file, "{}", output).unwrap();
        }*/
    } else {
        println!("{}", outputs_str);
    }
}
pub fn genrate_text(results: Vec<QueryResult>) -> Vec<u8>  {
    let mut w = Vec::new();
    let write_line = || -> Result<(), Error> {
        for result in results {
            writeln!(&mut w, " ")?;
            writeln!(&mut w, "{}", result.table)?;
            for hash_value in result.results {
                writeln!(&mut w, " - ")?;
                for (key, value) in hash_value {
                    if value.is_array() {
                        writeln!(&mut w, "  {}: ", key)?;
                        let n_value = value.as_array().unwrap();
                        for n_v in n_value {
                            if n_v.is_string() {
                                writeln!(&mut w, "      - {}", n_v.as_str().unwrap())?;
                            } else {
                                writeln!(&mut w, "      - {}", format!("{:#}", n_v))?;
                            }
                        }
                    } else if value.is_string() {
                        writeln!(&mut w, "  {}: {}", key, value.as_str().unwrap())?;
                    } else {
                        writeln!(&mut w, "  {}: {}", key, format!("{:#}", value))?;
                    }
                }
            }
        }

        Ok(())
    };
    if let Err(e) = write_line() {
        eprintln!("Couldn't write to file: {}", e);
    };
    w
}

pub fn genrate_xml(results: Vec<QueryResult>) -> Vec<u8> {
    let mut w = Vec::new();
    let new_uuid = Uuid::new_v4();
    let local_time: DateTime<Local> = Local::now();
    let date = &local_time.format("%d%m%Y%H%M%S").to_string();
    let write_line = || -> Result<(), Error> {
        writeln!(&mut w, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(&mut w, "<collect date={:?} timestamp=\"{:?}\" uuid=\"{}\">", date, local_time, new_uuid)?;
        for result in results {
            writeln!(&mut w, "    <query class=\"{}\">", result.table)?;
            for hash_value in result.results {
                writeln!(&mut w, "        <item>")?;
                for (key, value) in hash_value {
                    if value.is_array() {
                        writeln!(&mut w, "            <attr name=\"{}\" operator=\"=\">", key)?;
                        let n_value = value.as_array().unwrap();
                        for n_v in n_value {
                            if n_v.is_string() {
                                writeln!(&mut w, "                <element>{}</element>", n_v.as_str().unwrap())?;
                            } else {
                                writeln!(&mut w, "                <element>{}</element>", format!("{:#}", n_v))?;
                            }
                        }
                        writeln!(&mut w, "            </attr>")?;
                    } else if value.is_string() {
                        writeln!(&mut w, "            <attr name=\"{}\" operator=\"=\">{}</attr>", key, value.as_str().unwrap())?;
                    } else {
                        writeln!(&mut w, "            <attr name=\"{}\" operator=\"=\">{}</attr>", key, format!("{:#}", value))?;
                    }
                }
                writeln!(&mut w, "        </item>")?;
            }
            writeln!(&mut w, "    </query>")?;
        }
        writeln!(&mut w, "</collect>")?;

        Ok(())
    };
    if let Err(e) = write_line() {
        eprintln!("Couldn't write to file: {}", e);
    };
    w
}
pub fn genrate_json(results: Vec<QueryResult>) -> Vec<u8> {
    let mut w = Vec::new();
    let write_line = || -> Result<(), Error> {
        writeln!(&mut w, "{{")?;
        for result in results {
            writeln!(&mut w, "    {:?}: [", result.table)?;
            for hash_value in result.results {
                writeln!(&mut w, "        {{")?;
                for (key, value) in hash_value {
                    if value.is_array() {
                        writeln!(&mut w, "            {:?}: [", key)?;
                        let n_value = value.as_array().unwrap();
                        for n_v in n_value {
                            writeln!(&mut w, "                    {},", format!("{:#}", n_v))?;
                        }
                        writeln!(&mut w, "            ],")?;
                    } else if value.is_string() {
                        writeln!(&mut w, "            {:?}: {:?},", key, value.as_str().unwrap())?;
                    } else {
                        writeln!(&mut w, "            {:?}: {:?},", key, format!("{:#}", value))?;
                    }
                }
                writeln!(&mut w, "        }},")?;
            }
            writeln!(&mut w, "    ],")?;
        }
        writeln!(&mut w, "}}")?;

        Ok(())
    };
    if let Err(e) = write_line() {
        eprintln!("Couldn't write to file: {}", e);
    };
    w
}

