use crate::model::table::{Column, Index};
use std::cell::RefCell;
use std::rc::Rc;
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
    pub old_column: Option<Rc<RefCell<Column>>>,
    pub new_column: Option<Rc<RefCell<Column>>>,
}
#[derive(Debug)]
pub struct DiffIndex {
    pub name: String,
    pub old_index: Option<Rc<RefCell<Index>>>,
    pub new_index: Option<Rc<RefCell<Index>>>,
}
