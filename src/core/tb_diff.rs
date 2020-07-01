use crate::core::Diff;
use crate::model::diff_table::{DiffColumn, DiffIndex, DiffTable};
use crate::model::table::Table;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub type DiffMap = BTreeMap<String, DiffTable>;

pub struct TableDiff<'a> {
    tb1: &'a mut HashMap<String, Table>,
    tb2: &'a mut HashMap<String, Table>,
    pub diff: DiffMap,
}

impl<'a> TableDiff<'a> {
    pub fn new(
        tb1: &'a mut HashMap<String, Table>,
        tb2: &'a mut HashMap<String, Table>,
    ) -> TableDiff<'a> {
        TableDiff {
            tb1,
            tb2,
            diff: BTreeMap::new(),
        }
    }
    fn get_diff(diff: &mut DiffMap, name: String, desc: String) -> &mut DiffTable {
        diff.entry(name.clone()).or_insert(DiffTable {
            name,
            comment: desc,
            is_new: false,
            diff_columns: Vec::new(),
            diff_indexes: Vec::new(),
            diff_pks: Vec::new(),
        })
    }
}

impl Diff for TableDiff<'_> {
    fn diff(&mut self) {
        for (k1, v1) in self.tb1.iter() {
            if let Some(t2) = self.tb2.remove(k1) {
                let mut col_map = t2.group_cols();
                for ic1 in v1.columns.iter() {
                    let ric1 = (*ic1).borrow();
                    if let Some(ic2) = col_map.remove(&ric1.physical_name) {
                        let ric2 = (*ic2).borrow();
                        if ric1.column_type != ric2.column_type {
                            let dtb = TableDiff::get_diff(
                                &mut self.diff,
                                k1.clone(),
                                v1.logical_name.clone(),
                            );

                            dtb.diff_columns.push(DiffColumn {
                                name: ric1.physical_name.clone(),
                                new_column: Some(ic1.clone()),
                                old_column: Some(ic2.clone()),
                            });
                        }
                    } else {
                        let dtb = TableDiff::get_diff(
                            &mut self.diff,
                            k1.clone(),
                            v1.logical_name.clone(),
                        );
                        dtb.diff_columns.push(DiffColumn {
                            name: ric1.physical_name.clone(),
                            new_column: Some(ic1.clone()),
                            old_column: None,
                        });
                    }
                }
                for (_, icv2) in col_map.iter() {
                    let ricv2 = (*icv2).borrow();
                    let dtb =
                        TableDiff::get_diff(&mut self.diff, k1.clone(), v1.logical_name.clone());
                    dtb.diff_columns.push(DiffColumn {
                        name: ricv2.physical_name.clone(),
                        new_column: None,
                        old_column: Some((*icv2).clone()),
                    });
                }

                let mut pk_map = t2.group_pks();
                for ic1 in v1.primary_keys.iter() {
                    let ric1 = (*ic1).borrow();
                    if let Some(_ic2) = pk_map.remove(&ric1.physical_name) {
                    } else {
                        let dtb = TableDiff::get_diff(
                            &mut self.diff,
                            k1.clone(),
                            v1.logical_name.clone(),
                        );
                        dtb.diff_pks.push(DiffColumn {
                            name: ric1.physical_name.clone(),
                            new_column: Some(ic1.clone()),
                            old_column: None,
                        });
                    }
                }
                for (_, icv2) in pk_map.iter() {
                    let ricv2 = (*icv2).borrow();
                    let dtb =
                        TableDiff::get_diff(&mut self.diff, k1.clone(), v1.logical_name.clone());
                    dtb.diff_pks.push(DiffColumn {
                        name: ricv2.physical_name.clone(),
                        new_column: None,
                        old_column: Some((*icv2).clone()),
                    });
                }

                let mut idx_map = t2.group_idxes();
                for ic1 in v1.indexes.iter() {
                    let ric1 = (*ic1).borrow();
                    if let Some(ic2) = idx_map.remove(&ric1.get_cname()) {
                        let ric2 = (*ic2).borrow();
                        if ric1.non_unique != ric2.non_unique {
                            let dtb = TableDiff::get_diff(
                                &mut self.diff,
                                k1.clone(),
                                v1.logical_name.clone(),
                            );

                            dtb.diff_indexes.push(DiffIndex {
                                name: ric1.name.clone(),
                                new_index: Some(ic1.clone()),
                                old_index: Some(ic2.clone()),
                            });
                        }
                    } else {
                        let dtb = TableDiff::get_diff(
                            &mut self.diff,
                            k1.clone(),
                            v1.logical_name.clone(),
                        );
                        dtb.diff_indexes.push(DiffIndex {
                            name: ric1.name.clone(),
                            new_index: Some(ic1.clone()),
                            old_index: None,
                        });
                    }
                }
                for (_, icv2) in idx_map.iter() {
                    let ricv2 = (*icv2).borrow();
                    let dtb =
                        TableDiff::get_diff(&mut self.diff, k1.clone(), v1.logical_name.clone());
                    dtb.diff_indexes.push(DiffIndex {
                        name: ricv2.name.clone(),
                        new_index: None,
                        old_index: Some((*icv2).clone()),
                    });
                }
            } else {
                self.diff.insert(
                    k1.clone(),
                    DiffTable {
                        name: k1.clone(),
                        comment: v1.logical_name.clone(),
                        is_new: true,
                        diff_columns: v1
                            .columns
                            .iter()
                            .map(|e| DiffColumn {
                                name: e.borrow().physical_name.clone(),
                                new_column: Some(e.clone()),
                                old_column: None,
                            })
                            .collect(),
                        diff_indexes: v1
                            .indexes
                            .iter()
                            .map(|e| DiffIndex {
                                name: e.borrow().name.clone(),
                                old_index: None,
                                new_index: Some(e.clone()),
                            })
                            .collect(),
                        diff_pks: v1
                            .primary_keys
                            .iter()
                            .map(|e| DiffColumn {
                                name: e.borrow().physical_name.clone(),
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
