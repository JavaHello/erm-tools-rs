mod erm_read;
mod md_out;
mod tb_diff;

use crate::model::diff_table::DiffTable;
use crate::model::table::Table;
use std::collections::HashMap;
pub trait TbRead {
    fn read(&self, naem: &str) -> Option<&Table>;
}
pub use crate::core::erm_read::ErmRead;
pub use crate::core::md_out::MdOut;
pub use crate::core::tb_diff::TableDiff;
pub trait Diff {
    fn diff(&mut self);
}

pub trait OutDiff {
    fn write(&mut self, diff_tables: &HashMap<String, DiffTable>);
}
