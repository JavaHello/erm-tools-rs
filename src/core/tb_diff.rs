use crate::core::Diff;
use crate::model::diff_table::{DiffColumn, DiffIndex, DiffTable};
use crate::model::table::Table;
use std::collections::HashMap;

pub struct TableDiff<'a> {
    tb1: &'a mut HashMap<String, Table>,
    tb2: &'a mut HashMap<String, Table>,
    pub diff: HashMap<String, DiffTable>,
}

impl<'a> TableDiff<'a> {
    pub fn new(
        tb1: &'a mut HashMap<String, Table>,
        tb2: &'a mut HashMap<String, Table>,
    ) -> TableDiff<'a> {
        TableDiff {
            tb1,
            tb2,
            diff: HashMap::new(),
        }
    }
}

impl Diff for TableDiff<'_> {
    fn diff(&mut self) {
        for (k1, v1) in self.tb1.iter() {
            if let Some(t2) = self.tb2.remove(k1) {
                let mut col_map = HashMap::new();
                t2.columns.iter().for_each(|e| {
                    col_map.entry(e.physical_name.clone()).or_insert(e);
                });
                for ic1 in v1.columns.iter() {
                    if let Some(ic2) = col_map.remove(&ic1.physical_name) {
                        if ic1.column_type != ic2.column_type {
                            let dtb = self.diff.entry(k1.clone()).or_insert(DiffTable {
                                name: k1.clone(),
                                comment: v1.description.clone(),
                                is_new: false,
                                diff_columns: Vec::new(),
                                diff_indexes: Vec::new(),
                                diff_pks: Vec::new(),
                            });

                            dtb.diff_columns.push(DiffColumn {
                                name: ic1.physical_name.clone(),
                                new_column: Some(ic1.clone()),
                                old_column: Some(ic2.clone()),
                            });
                        }
                    } else {
                        let dtb = self.diff.entry(k1.clone()).or_insert(DiffTable {
                            name: k1.clone(),
                            comment: v1.description.clone(),
                            is_new: false,
                            diff_columns: Vec::new(),
                            diff_indexes: Vec::new(),
                            diff_pks: Vec::new(),
                        });
                        dtb.diff_columns.push(DiffColumn {
                            name: ic1.physical_name.clone(),
                            new_column: Some(ic1.clone()),
                            old_column: None,
                        });
                    }
                }
                for (_, icv2) in col_map.iter() {
                    let dtb = self.diff.entry(k1.clone()).or_insert(DiffTable {
                        name: k1.clone(),
                        comment: v1.description.clone(),
                        is_new: false,
                        diff_columns: Vec::new(),
                        diff_indexes: Vec::new(),
                        diff_pks: Vec::new(),
                    });
                    dtb.diff_columns.push(DiffColumn {
                        name: icv2.physical_name.clone(),
                        new_column: None,
                        old_column: Some((*icv2).clone()),
                    });
                }
            } else {
                self.diff.insert(
                    k1.clone(),
                    DiffTable {
                        name: k1.clone(),
                        comment: v1.description.clone(),
                        is_new: true,
                        diff_columns: v1
                            .columns
                            .iter()
                            .map(|e| DiffColumn {
                                name: e.physical_name.clone(),
                                new_column: Some(e.clone()),
                                old_column: None,
                            })
                            .collect(),
                        diff_indexes: v1
                            .indexes
                            .iter()
                            .map(|e| DiffIndex {
                                name: e.name.clone(),
                                old_index: None,
                                new_index: Some(e.clone()),
                            })
                            .collect(),
                        diff_pks: v1
                            .primary_keys
                            .iter()
                            .map(|e| DiffColumn {
                                name: e.physical_name.clone(),
                                new_column: Some(e.clone()),
                                old_column: None,
                            })
                            .collect(),
                    },
                );
            }
        }
    }
}
