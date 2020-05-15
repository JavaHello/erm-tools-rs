use crate::core::OutDiff;
use crate::model::diff_table::DiffTable;

use std::collections::BTreeMap;
#[derive(Default)]
pub struct DdlOut {
    pub content: String,
    pub db_name: String,
}

// const CREATE_TABLE: &str = "create table ";
const INDENT_STR: &str = "    ";
impl DdlOut {
    pub fn new(db_name: &str) -> Self {
        DdlOut {
            content: String::new(),
            db_name: db_name.to_string(),
        }
    }
}

fn order(f: bool) -> &'static str {
    if f {
        "desc"
    } else {
        "asc"
    }
}

impl OutDiff for DdlOut {
    fn write(&mut self, diff_tables: &BTreeMap<String, DiffTable>) {
        if !diff_tables.is_empty() {
            self.content.push_str(&format!("-- {}\n", self.db_name));
            for (tn, dtb) in diff_tables.iter() {
                if dtb.is_new {
                    self.content.push_str(&format!("create table {} ", tn));
                    self.content.push_str("(\n");
                    for ntb in dtb.diff_columns.iter() {
                        if let Some(col) = &ntb.new_column {
                            let col = col.borrow();
                            self.content.push_str(INDENT_STR);
                            self.content.push_str(&format!(
                                "{} {} comment '{}',",
                                &col.physical_name, &col.column_type, &col.logical_name
                            ));
                            self.content.push_str("\n");
                        }
                    }

                    for nidx in dtb.diff_indexes.iter() {
                        if let Some(idx) = &nidx.new_index {
                            let idx = idx.borrow();
                            self.content.push_str(INDENT_STR);
                            if idx.non_unique {
                                self.content.push_str("key ");
                            } else {
                                self.content.push_str("unique key ");
                            }
                            let col_str = idx
                                .columns
                                .iter()
                                .map(|e| {
                                    format!(
                                        "{} {}",
                                        e.borrow().physical_name,
                                        order(e.borrow().desc)
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join(", ");
                            self.content.push_str(&format!("({}),", col_str));
                            self.content.push_str("\n");
                        }
                    }

                    let mut pk_cols: Option<String> = None;
                    for npk in dtb.diff_pks.iter() {
                        let mut col_str = String::new();
                        if let Some(col) = &npk.new_column {
                            let col = col.borrow();
                            col_str.push_str(&col.physical_name);
                        }
                        pk_cols = Some(col_str);
                    }
                    if let Some(col_str) = pk_cols {
                        self.content.push_str(INDENT_STR);
                        self.content.push_str(&format!("primary key ({})", col_str));
                        self.content.push_str("\n");
                    }
                    self.content.push_str(") ");
                    self.content.push_str(&format!("comment '{}'", &dtb.name));
                    self.content.push_str(";");
                    self.content.push_str("\n");
                }
            }
        }
    }
}
