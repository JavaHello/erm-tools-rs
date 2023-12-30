use quick_xml::de::from_str;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use crate::core::env;
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

fn parse_erm_error_msg(file_name: &str) -> String {
    format!("erm 文件解析失败:{}", file_name)
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
        let cov_type = env::get_mysql_cov_type();
        for file in self.file_list.iter() {
            let data = read_xml(file);
            let erm: Diagram = from_str(&data).unwrap();
            let group_word = erm.group_word();
            for it in erm.contents.table.iter() {
                let pname = it
                    .physical_name
                    .as_ref()
                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                    .clone();
                let lname = it
                    .logical_name
                    .as_ref()
                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                    .clone();
                let desc = it
                    .description
                    .as_ref()
                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                    .clone();
                let mut table = Table {
                    physical_name: pname.clone(),
                    logical_name: lname,
                    description: Some(desc),
                    columns: Vec::new(),
                    primary_keys: Vec::new(),
                    indexes: Vec::new(),
                };
                let mut col_map = HashMap::new();

                for ic in it.columns.normal_column.iter() {
                    let pname = it
                        .physical_name
                        .as_ref()
                        .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                        .clone();
                    let lname = it
                        .logical_name
                        .as_ref()
                        .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                        .clone();
                    let word = match &ic.word_id {
                        Some(word_id) => group_word.get(word_id),
                        None => None,
                    };
                    match word {
                        Some(e) => {
                            let col = Rc::new(RefCell::new(Column {
                                physical_name: e
                                    .physical_name
                                    .as_ref()
                                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                                    .clone(),
                                logical_name: e
                                    .logical_name
                                    .as_ref()
                                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                                    .clone(),
                                r#type: e
                                    .r#type
                                    .as_ref()
                                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                                    .to_owned(),
                                unsigned: e
                                    .unsigned
                                    .as_ref()
                                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                                    .eq("true"),
                                auto_increment: false,
                                default_value: ic.default_value.clone(),
                                length: match &e.length {
                                    Some(e) => match e.parse() {
                                        Ok(v) => Some(v),
                                        Err(_) => None,
                                    },
                                    None => None,
                                },
                                decimal: match &e.decimal {
                                    Some(e) => match e.parse() {
                                        Ok(v) => Some(v),
                                        Err(_) => None,
                                    },
                                    None => None,
                                },
                                primary_key: ic
                                    .primary_key
                                    .as_ref()
                                    .unwrap_or(&"false".to_owned())
                                    .parse()
                                    .unwrap_or_default(),
                                unique_key: ic
                                    .unique_key
                                    .as_ref()
                                    .unwrap_or(&"false".to_owned())
                                    .parse()
                                    .unwrap_or_default(),
                                not_null: ic
                                    .not_null
                                    .as_ref()
                                    .unwrap_or(&"false".to_owned())
                                    .parse()
                                    .unwrap_or_default(),
                                description: e.description.clone(),
                                desc: false,
                                column_type: e
                                    .r#type
                                    .as_ref()
                                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                                    .to_owned(),
                            }));
                            if col.borrow().primary_key {
                                table.primary_keys.push(Rc::clone(&col));
                            }
                            col_map.insert(
                                ic.id
                                    .as_ref()
                                    .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                                    .to_owned(),
                                Rc::clone(&col),
                            );
                            table.columns.push(Rc::clone(&col));
                            let mut col = col.borrow_mut();
                            // 拆分 varchar(n) 这种类型
                            let cidx: usize = col.r#type.find('(').unwrap_or_default();
                            if cidx > 0 {
                                col.r#type = String::from(col.r#type.get(..cidx).unwrap());
                            }
                            let ignore_type =
                                env::get_ignore_len_type().contains(&col.r#type.to_lowercase());
                            if ignore_type {
                                col.length = None;
                                col.decimal = None;
                            }
                            if let Some(cfg_type) = cov_type.get(&col.r#type) {
                                col.r#type = cfg_type.name.clone();
                                if let Some(len) = cfg_type.length {
                                    col.length = Some(len);
                                }
                            }
                            if cidx > 0 && !ignore_type {
                                col.column_type = format!(
                                    "{}{}{}",
                                    col.r#type,
                                    "(",
                                    col.length.unwrap_or_default()
                                );

                                if let Some(decimal) = col.decimal {
                                    col.column_type
                                        .push_str(&format!("{}{}{}", ", ", decimal, ")"));
                                } else {
                                    col.column_type.push(')');
                                }
                            } else {
                                col.column_type = col.r#type.clone();
                            }
                            if col.unsigned {
                                col.column_type = format!("{} {}", "unsigned", col.r#type);
                            }
                        }
                        None => {
                            warn!("无法获取的字段 {} - {}", &pname, &lname);
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
                            Rc::clone(
                                col_map
                                    .get(
                                        &e.id
                                            .as_ref()
                                            .unwrap_or_else(|| {
                                                panic!("{}", parse_erm_error_msg(file))
                                            })
                                            .to_owned(),
                                    )
                                    .unwrap_or_else(|| panic!("索引配置有错误, table: {}", pname)),
                            )
                        })
                        .collect::<Vec<Rc<RefCell<Column>>>>();
                    let index = Rc::new(RefCell::new(Index {
                        name: idx
                            .name
                            .as_ref()
                            .unwrap_or_else(|| panic!("{}", parse_erm_error_msg(file)))
                            .to_owned(),
                        non_unique: idx
                            .non_unique
                            .as_ref()
                            .unwrap_or(&"false".to_owned())
                            .parse()
                            .unwrap_or_default(),
                        columns: cols,
                    }));
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
