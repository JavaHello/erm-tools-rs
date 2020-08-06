use crate::core::OutDiff;
use crate::model::diff_table::DiffTable;

use std::collections::BTreeMap;
#[derive(Default)]
pub struct MdOut {
    pub content: String,
    pub db_name: String,
}

const COLUMN_TITLE: &str = "|new名称|new类型|new长度|new精度||old名称|old类型|old长度|old精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
";

const INDEX_TITLE: &str = "|new名称|new字段|new类型||old名称|old字段|old类型|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
";

impl MdOut {
    pub fn new(db_name: &str) -> Self {
        MdOut {
            content: String::new(),
            db_name: db_name.to_string(),
        }
    }
}

fn i32_to_str(i: Option<i32>) -> String {
    if let Some(i) = i {
        format!("{}", i)
    } else {
        String::new()
    }
}

impl OutDiff for MdOut {
    fn write(&mut self, diff_tables: &BTreeMap<String, DiffTable>) {
        if !diff_tables.is_empty() {
            self.content
                .push_str(&format!("# {}差异输出\n", self.db_name));
            for (k, dtb) in diff_tables.iter() {
                self.content.push_str(&format!("\n## {}\n", k));
                self.content.push_str(COLUMN_TITLE);
                for dcol in dtb.diff_columns.iter() {
                    if let Some(col) = &dcol.new_column {
                        let col = (*col).borrow();
                        self.content.push_str(&format!(
                            "|{}|{}|{}|{}|",
                            col.physical_name,
                            col.r#type,
                            i32_to_str(col.length),
                            i32_to_str(col.decimal)
                        ));
                    } else {
                        self.content.push_str("|||||");
                    }
                    if let Some(col) = &dcol.old_column {
                        let col = (*col).borrow();
                        self.content.push_str(&format!(
                            "|{}|{}|{}|{}|",
                            col.physical_name,
                            col.r#type,
                            i32_to_str(col.length),
                            i32_to_str(col.decimal)
                        ));
                    } else {
                        self.content.push_str("|||||");
                    }
                    self.content.push_str("\n");
                }

                if !dtb.diff_pks.is_empty() || !dtb.diff_indexes.is_empty() {
                    self.content.push_str(&format!("\n## {} 索引差异\n", k));
                    self.content.push_str(INDEX_TITLE);
                }

                for dcol in dtb.diff_pks.iter() {
                    if let Some(col) = &dcol.new_column {
                        let col = (*col).borrow();
                        self.content
                            .push_str(&format!("|pk|{}|主键|", col.physical_name));
                    } else {
                        self.content.push_str("||||");
                    }
                    if let Some(col) = &dcol.old_column {
                        let col = (*col).borrow();
                        self.content
                            .push_str(&format!("|pk|{}|主键|", col.physical_name));
                    } else {
                        self.content.push_str("||||");
                    }
                    self.content.push_str("\n");
                }

                for dcol in dtb.diff_indexes.iter() {
                    if let Some(col) = &dcol.new_index {
                        let col = (*col).borrow();
                        if col.non_unique {
                            self.content.push_str(&format!(
                                "|{}|{}|普通|",
                                col.name,
                                col.get_cname()
                            ));
                        } else {
                            self.content.push_str(&format!(
                                "|{}|{}|唯一|",
                                col.name,
                                col.get_cname()
                            ));
                        }
                    } else {
                        self.content.push_str("||||");
                    }
                    if let Some(col) = &dcol.old_index {
                        let col = (*col).borrow();
                        if col.non_unique {
                            self.content.push_str(&format!(
                                "|{}|{}|普通|",
                                col.name,
                                col.get_cname()
                            ));
                        } else {
                            self.content
                                .push_str(&format!("|{}|{}||", col.name, col.get_cname()));
                        }
                    } else {
                        self.content.push_str("||||");
                    }
                    self.content.push_str("\n");
                }
            }
        }
    }
}
