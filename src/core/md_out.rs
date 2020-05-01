use crate::core::OutDiff;
use crate::model::diff_table::{DiffColumn, DiffIndex, DiffTable};
use crate::model::table::Table;
use std::collections::HashMap;
pub struct MdOut {
    pub content: String,
}

const COLUMN_TITLE: &'static str =
    "|new名称|new类型|new长度|new精度||old名称|old类型|old长度|old精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
";

impl MdOut {
    pub fn new() -> Self {
        MdOut {
            content: String::new(),
        }
    }
}

impl OutDiff for MdOut {
    fn write(&mut self, diff_tables: &HashMap<String, DiffTable>) {
        for (k, dtb) in diff_tables.iter() {
            self.content.push_str(&format!("## {}\n", k));
            self.content.push_str(COLUMN_TITLE);
            for dcol in dtb.diff_columns.iter() {
                if let Some(col) = &dcol.new_column {
                    self.content.push_str(&format!(
                        "|{}|{}|{}|{}|",
                        col.physical_name, col.r#type, col.length, col.decimal
                    ));
                } else {
                    self.content.push_str("|||||");
                }
                if let Some(col) = &dcol.old_column {
                    self.content.push_str(&format!(
                        "|{}|{}|{}|{}|",
                        col.physical_name, col.r#type, col.length, col.decimal
                    ));
                } else {
                    self.content.push_str("|||||");
                }
                self.content.push_str("\n");
            }
        }
    }
}
