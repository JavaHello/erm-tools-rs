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
                            self.content.push('\n');
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
                            self.content.push('\n');
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
                        self.content.push('\n');
                    }
                    self.content.push_str(") ");
                    self.content.push_str(&format!("comment '{}'", &dtb.name));
                    self.content.push(';');
                    self.content.push('\n');
                } else {
                    for diff_col in dtb.diff_columns.iter() {
                        match (diff_col.new_column.as_ref(), diff_col.old_column.as_ref()) {
                            (Some(new_col), Some(_old_col)) => {
                                let new_col = new_col.borrow();
                                // alter table {} modify column {} {};
                                self.content.push_str(&format!(
                                    "alter table {} modify column {} {};\n",
                                    &dtb.name, new_col.physical_name, new_col.column_type
                                ));
                            }
                            (None, Some(old_col)) => {
                                let old_col = old_col.borrow();
                                // alter table {} drop column {};
                                self.content.push_str(&format!(
                                    "alter table {} drop column {};\n",
                                    &dtb.name, old_col.physical_name
                                ));
                            }
                            (Some(new_col), None) => {
                                let new_col = new_col.borrow();
                                // alter table {} add column {} {};
                                self.content.push_str(&format!(
                                    "alter table {} add column {} {};\n",
                                    &dtb.name, new_col.physical_name, new_col.column_type
                                ));
                            }
                            (_, _) => {}
                        }
                    }
                    for diff_index in dtb.diff_indexes.iter() {
                        match (diff_index.new_index.as_ref(), diff_index.old_index.as_ref()) {
                            (Some(new_index), Some(old_index)) => {
                                let old_index = old_index.borrow();
                                // drop index {} on {};
                                self.content.push_str(&format!(
                                    "drop index {} on {};\n",
                                    old_index.name, &dtb.name
                                ));
                                let new_index = new_index.borrow();
                                // alter table {} add index/unique {}({});
                                self.content.push_str(&format!(
                                    "alter table {} add {} {}({});\n",
                                    &dtb.name,
                                    index_type(new_index.non_unique),
                                    new_index.name,
                                    new_index.get_cname()
                                ));
                            }
                            (None, Some(old_index)) => {
                                let old_index = old_index.borrow();
                                // drop index {} on {};
                                self.content.push_str(&format!(
                                    "drop index {} on {};\n",
                                    old_index.name, &dtb.name
                                ));
                            }
                            (Some(new_index), None) => {
                                let new_index = new_index.borrow();
                                // alter table {} add index/unique {}({});
                                self.content.push_str(&format!(
                                    "alter table {} add {} {}({});\n",
                                    &dtb.name,
                                    index_type(new_index.non_unique),
                                    new_index.name,
                                    new_index.get_cname()
                                ));
                            }
                            (_, _) => {}
                        }
                    }
                    if !dtb.diff_pks.is_empty() {
                        // alter table {} drop primary key;
                        // alter table {} add primary key({});
                        let primary_key = dtb
                            .diff_pks
                            .iter()
                            .map(|e| e.new_column.as_ref())
                            .filter(|e| e.is_some())
                            .map(|e| e.unwrap().borrow().physical_name.clone())
                            .collect::<Vec<String>>()
                            .join(", ");
                        self.content
                            .push_str(&format!("alter table {} drop primary key;\n", &dtb.name));
                        self.content.push_str(&format!(
                            "alter table {} add primary key({});\n",
                            &dtb.name, primary_key
                        ));
                    }
                }
            }
        }
    }
}

fn index_type(non_unique: bool) -> &'static str {
    if non_unique {
        "index"
    } else {
        "unique"
    }
}
