use quick_xml::de::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::core::TbRead;
use crate::model::erm::Diagram;
use crate::model::table::{Column, Index, Table};
use log::warn;
pub fn read_xml(file_name: &str) -> String {
    let mut file: File =
        File::open(file_name).unwrap_or_else(|_| panic!("读取文件失败：{}", file_name));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("读取文件内容失败：{}", file_name));
    contents
}

pub struct ErmRead {
    pub talbes: HashMap<String, Table>,
    file_list: Vec<String>,
}

impl ErmRead {
    pub fn new(file_list: Vec<String>) -> ErmRead {
        let mut read = ErmRead {
            talbes: HashMap::new(),
            file_list,
        };
        read.init();
        read
    }
    fn init(&mut self) {
        for file in self.file_list.iter() {
            let data = read_xml(&file);
            let erm: Diagram = from_str(&data).unwrap();
            let group_word = erm.group_word();
            for it in erm.contents.table.iter() {
                let pname = it.physical_name.clone();
                let lname = it.logical_name.clone();
                let desc = it.description.clone();
                let mut table = Table {
                    physical_name: pname.clone(),
                    logical_name: lname,
                    description: desc,
                    columns: Vec::new(),
                    primary_keys: Vec::new(),
                    indexes: Vec::new(),
                };
                let mut col_map: HashMap<String, Column> = HashMap::new();

                for ic in it.columns.normal_column.iter() {
                    let word = group_word.get(&ic.word_id);
                    match word {
                        Some(e) => {
                            let mut col = Column {
                                physical_name: e.physical_name.clone(),
                                logical_name: e.logical_name.clone(),
                                r#type: e.r#type.clone(),
                                auto_increment: false,
                                default_value: ic.default_value.clone(),
                                length: e.length.parse().unwrap_or_default(),
                                decimal: e.decimal.parse().unwrap_or_default(),
                                primary_key: ic.primary_key.parse().unwrap(),
                                unique_key: ic.unique_key.parse().unwrap(),
                                not_null: ic.not_null.parse().unwrap(),
                                description: e.description.clone(),
                                desc: false,
                                column_type: e.r#type.clone(),
                            };

                            // 拆分 varchar(n) 这种类型
                            let cidx: usize = col.r#type.find('(').unwrap_or_default();
                            if cidx > 0 {
                                col.r#type = String::from(col.r#type.get(..cidx).unwrap());
                                col.column_type = format!("{}{}{}", col.r#type, "(", col.length);
                                if col.decimal > 0 {
                                    col.column_type
                                        .push_str(&format!("{}{}{}", ", ", col.decimal, ")"));
                                } else {
                                    col.column_type.push_str(")");
                                }
                            }
                            if col.primary_key {
                                table.primary_keys.push(col.clone());
                            }
                            col_map.insert(ic.id.to_owned(), col.clone());
                            table.columns.push(col);
                        }
                        None => {
                            warn!(
                                "无法获取的字段 {} - {}",
                                &ic.physical_name, &ic.logical_name
                            );
                            continue;
                        }
                    }
                }
                for idx in it.indexes.index.iter() {
                    let cols = idx
                        .columns
                        .column
                        .iter()
                        .map(|e| {
                            let col = col_map.get(&e.id).unwrap_or_else(|| {
                                panic!("索引配置有错误, table: {}", it.physical_name)
                            });
                            col.clone()
                        })
                        .collect::<Vec<Column>>();
                    let index = Index {
                        name: idx.name.clone(),
                        non_unique: idx.non_unique.parse().unwrap(),
                        columns: cols,
                    };
                    table.indexes.push(index);
                }
                self.talbes.insert(pname, table);
            }
        }
    }
}

impl TbRead for ErmRead {
    fn read(&self, name: &str) -> Option<&Table> {
        self.talbes.get(name)
    }
}
