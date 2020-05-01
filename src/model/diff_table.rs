use crate::model::table::{Column, Index};
#[derive(Debug)]
pub struct DiffTable {
    pub name: String,
    pub comment: String,
    pub is_new: bool,
    pub diff_columns: Vec<DiffColumn>,
    pub diff_indexes: Vec<DiffIndex>,
    pub diff_pks: Vec<DiffColumn>,
}

#[derive(Debug)]
pub struct DiffColumn {
    pub name: String,
    pub old_column: Option<Column>,
    pub new_column: Option<Column>,
}
#[derive(Debug)]
pub struct DiffIndex {
    pub name: String,
    pub old_index: Option<Index>,
    pub new_index: Option<Index>,
}
