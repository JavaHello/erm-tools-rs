use crate::core::OutDiff;
use crate::model::diff_table::DiffTable;
use crate::model::table::Column;

use std::collections::BTreeMap;
#[derive(Default)]
pub struct MdOut {
    pub content: String,
    pub db_name: String,
}

const COLUMN_TITLE: &str = "|S名称|S类型|S长度|S精度||T名称|T类型|T长度|T精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
";

const INDEX_TITLE: &str = "|S名称|S字段|S类型||T名称|T字段|T类型|
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

fn to_str(i: Option<i64>) -> String {
    if let Some(i) = i {
        format!("{}", i)
    } else {
        String::new()
    }
}

fn type_out(col: &Column) -> String {
    if col.unsigned {
        format!("{} {}", col.data_type, "unsigned")
    } else {
        col.data_type.to_string()
    }
}

impl OutDiff for MdOut {
    fn write(&mut self, diff_tables: &BTreeMap<String, DiffTable>) {
        if !diff_tables.is_empty() {
            self.content
                .push_str(&format!("# {}差异输出\n", self.db_name));
            for (k, dtb) in diff_tables.iter() {
                if !dtb.diff_columns.is_empty() {
                    self.content.push_str(&format!("\n## {}\n", k));
                    self.content.push_str(COLUMN_TITLE);
                }
                for dcol in dtb.diff_columns.iter() {
                    if let Some(col) = &dcol.source_column {
                        let col = (*col).borrow();
                        self.content.push_str(&format!(
                            "|{}|{}|{}|{}|",
                            col.physical_name,
                            type_out(&col),
                            to_str(col.length),
                            to_str(col.decimal)
                        ));
                    } else {
                        self.content.push_str("|||||");
                    }
                    if let Some(col) = &dcol.target_column {
                        let col = (*col).borrow();
                        self.content.push_str(&format!(
                            "|{}|{}|{}|{}|",
                            col.physical_name,
                            type_out(&col),
                            to_str(col.length),
                            to_str(col.decimal)
                        ));
                    } else {
                        self.content.push_str("|||||");
                    }
                    self.content.push('\n');
                }

                if !dtb.diff_pks.is_empty() || !dtb.diff_indexes.is_empty() {
                    self.content.push_str(&format!("\n## {} 索引差异\n", k));
                    self.content.push_str(INDEX_TITLE);
                }

                for dcol in dtb.diff_pks.iter() {
                    if let Some(col) = &dcol.source_column {
                        let col = (*col).borrow();
                        self.content
                            .push_str(&format!("|pk|{}|主键|", col.physical_name));
                    } else {
                        self.content.push_str("||||");
                    }
                    if let Some(col) = &dcol.target_column {
                        let col = (*col).borrow();
                        self.content
                            .push_str(&format!("|pk|{}|主键|", col.physical_name));
                    } else {
                        self.content.push_str("||||");
                    }
                    self.content.push('\n');
                }

                for dcol in dtb.diff_indexes.iter() {
                    if let Some(col) = &dcol.source_index {
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
                    if let Some(col) = &dcol.target_index {
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
                    self.content.push('\n');
                }
            }
        }
    }
}
