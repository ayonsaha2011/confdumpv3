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
#![allow(dead_code)]

use wmi::{COMLibrary, WMIConnection};
use std::error::Error;
use std::fmt;
use serde_json::Value;
use std::collections::HashMap;

pub struct QueryResult {
    pub table: String,
    pub results: Vec<HashMap<String, Value>>
}

#[derive(Debug)]
pub struct CDError {
    details: String
}

impl CDError {
    fn new(msg: &str) -> CDError {
        CDError{details: msg.to_string()}
    }
}

impl fmt::Display for CDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for CDError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub async fn query(tables: Vec<String>) -> Result<Vec<QueryResult>, CDError> {
    let mut output = Vec::<QueryResult>::new();
    for table in tables {
        println!("table - {}", table);
        let wmi_result = exec_async_query(table).await?;
        output.push(wmi_result);
    }

    Ok(output)
}

async fn exec_async_query(table: String) -> Result<QueryResult, CDError> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();
    let results: Vec<HashMap<String, Value>> = match wmi_con.async_raw_query("SELECT * FROM ".to_string() + &table.as_str()).await {
        Ok(result)  => result,
        Err(e) => {println!("table- {}, {:#?}", &table, e); Vec::<HashMap<String, Value>>::new() },
    };

    Ok(QueryResult{ table, results })
}
