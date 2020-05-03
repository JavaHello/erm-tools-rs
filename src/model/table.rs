use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
// Table 数据库表结构
#[derive(Debug)]
pub struct Table {
    pub physical_name: String,
    pub logical_name: String,
    pub description: Option<String>,
    pub columns: Vec<Rc<RefCell<Column>>>,
    pub primary_keys: Vec<Rc<RefCell<Column>>>,
    pub indexes: Vec<Rc<RefCell<Index>>>,
}

// Column 字段信息
#[derive(Debug, Clone)]
pub struct Column {
    pub physical_name: String,
    pub logical_name: String,
    pub r#type: String,
    pub auto_increment: bool,
    pub default_value: Option<String>,
    pub length: Option<i32>,
    pub decimal: Option<i32>,
    pub primary_key: bool,
    pub unique_key: bool,
    pub not_null: bool,
    pub description: Option<String>,
    pub desc: bool,
    pub column_type: String,
}

// Index 索引信息
#[derive(Debug, Clone)]
pub struct Index {
    pub name: String,
    pub non_unique: bool,
    pub columns: Vec<Rc<RefCell<Column>>>,
}
impl Index {
    pub fn get_cname(&self) -> String {
        let mut result = String::new();
        let cname = self
            .columns
            .iter()
            .map(|e| (*e).borrow().physical_name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        result.push_str(&cname);
        result
    }
}
impl Table {
    pub fn group_cols(&self) -> HashMap<String, Rc<RefCell<Column>>> {
        let mut gm = HashMap::new();
        for it in self.columns.iter() {
            gm.insert((*it).borrow().physical_name.clone(), Rc::clone(it));
        }
        gm
    }

    pub fn group_pks(&self) -> HashMap<String, Rc<RefCell<Column>>> {
        let mut gm = HashMap::new();
        for it in self.primary_keys.iter() {
            gm.insert((*it).borrow().physical_name.clone(), Rc::clone(it));
        }
        gm
    }

    pub fn group_idxes(&self) -> HashMap<String, Rc<RefCell<Index>>> {
        let mut gm = HashMap::new();
        for it in self.indexes.iter() {
            gm.insert((*it).borrow().get_cname(), Rc::clone(it));
        }
        gm
    }
}
