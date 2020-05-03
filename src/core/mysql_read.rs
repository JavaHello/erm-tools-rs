use crate::core::TbRead;
use crate::model::table::{Column, Index, Table};
use mysql::prelude::*;
use mysql::*;
use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;
pub struct MysqlRead {
    pool: Pool,
    db_name: String,
    pub talbes: HashMap<String, Table>,
}

impl MysqlRead {
    pub fn new(url: &str, db_name: &str) -> Self {
        let mut mrd = MysqlRead {
            pool: Pool::new(url).unwrap_or_else(|_| panic!("初始化mysql连接失败:{}", url)),
            db_name: db_name.to_owned(),
            talbes: HashMap::new(),
        };
        mrd.read_all();
        mrd
    }

    fn read_all(&mut self) {
        let mut conn = self.pool.get_conn().expect("获取连接失败");
        let prep = conn
            .prep("SELECT TABLE_NAME,TABLE_COMMENT FROM TABLES WHERE TABLE_SCHEMA = ?")
            .expect("查询表失败");
        let mut table_list = conn
            .exec_map(prep, (self.db_name.as_str(),), |(name, comment)| Table {
                physical_name: name,
                logical_name: comment,
                description: None,
                columns: Vec::new(),
                primary_keys: Vec::new(),
                indexes: Vec::new(),
            })
            .expect("查询表失败");
        for table in table_list.iter_mut() {
            let table_name = &table.physical_name;
            let mut col_map = HashMap::new();
            let prep = conn
                .prep(
                    "select
                        TABLE_NAME,
                        COLUMN_NAME,
                        IS_NULLABLE,
                        DATA_TYPE,
                        CHARACTER_MAXIMUM_LENGTH,
                        NUMERIC_PRECISION,
                        NUMERIC_SCALE,
                        COLUMN_COMMENT,
                        COLUMN_TYPE,
                        EXTRA,
                        COLUMN_DEFAULT
                    from COLUMNS
                    where TABLE_SCHEMA = ?
                    and TABLE_NAME = ?",
                )
                .unwrap_or_else(|_| panic!("查询表结构失败:{}", table_name));
            let col_list = conn
                .exec_map(
                    prep,
                    (self.db_name.as_str(), table_name),
                    |(
                        table_name,
                        column_name,
                        is_nullable,
                        data_type,
                        character_maximum_length,
                        numeric_precision,
                        numeric_scale,
                        column_comment,
                        column_type,
                        extra,
                        column_default,
                    )| {
                        let _: String = table_name;
                        let column_name: String = column_name;
                        let is_nullable: String = is_nullable;
                        let data_type: String = data_type;
                        let character_maximum_length: Option<i32> = character_maximum_length;
                        let numeric_precision: Option<i32> = numeric_precision;
                        let numeric_scale: Option<i32> = numeric_scale;
                        let column_comment: String = column_comment;
                        let column_type: String = column_type;
                        let extra: String = extra;
                        let column_default: Option<String> = column_default;
                        let len = if let Some(v) = character_maximum_length {
                            Some(v)
                        } else if let Some(v) = numeric_precision {
                            Some(v)
                        } else {
                            None
                        };
                        let col_type = if column_type != "" {
                            column_type
                        } else {
                            data_type.clone()
                        };
                        let col = Rc::new(RefCell::new(Column {
                            physical_name: column_name.clone(),
                            logical_name: column_comment,
                            r#type: data_type,
                            auto_increment: "auto_increment" == extra,
                            default_value: column_default,
                            length: len,
                            decimal: numeric_scale,
                            primary_key: false,
                            unique_key: false,
                            not_null: "NO" == is_nullable,
                            description: None,
                            desc: false,
                            column_type: col_type,
                        }));
                        col_map.insert(column_name, Rc::clone(&col));
                        col
                    },
                )
                .expect("查询表失败");

            let prep = conn
                .prep(
                    "select TABLE_NAME, NON_UNIQUE, INDEX_NAME, COLUMN_NAME
                    from STATISTICS
                    where TABLE_SCHEMA = ?
                      and TABLE_NAME = ?",
                )
                .unwrap_or_else(|_| panic!("查询索引失败:{}", table_name));
            let pks = &mut table.primary_keys;
            let ids = &mut table.indexes;
            let idx_info_list = conn
                .exec::<(String, bool, String, String), _, _>(
                    prep,
                    (self.db_name.as_str(), table_name),
                )
                .expect("查询表失败");
            let mut old_idx_name = String::new();
            let mut idx: Option<Rc<RefCell<Index>>> = None;
            for (_, non_unique, idx_name, col_name) in idx_info_list {
                let col = col_map.get(&col_name).unwrap();
                let mut rcol = col.borrow_mut();
                rcol.unique_key = non_unique;
                if idx_name == "PRIMARY" {
                    pks.push(col.clone());
                    rcol.primary_key = true;
                } else if old_idx_name != idx_name {
                    let mut rx = Index {
                        name: idx_name.clone(),
                        non_unique,
                        columns: Vec::new(),
                    };
                    rx.columns.push(col.clone());
                    let x = Rc::new(RefCell::new(rx));
                    ids.push(Rc::clone(&x));
                    idx = Some(Rc::clone(&x));
                } else if let Some(v) = idx.take() {
                    v.borrow_mut().columns.push(col.clone());
                }
                old_idx_name = idx_name.clone();
            }
            table.columns.extend(col_list);
        }
        for tb in table_list {
            self.talbes.insert(tb.physical_name.clone(), tb);
        }
    }
}
impl TbRead for MysqlRead {
    fn read(&self, name: &str) -> Option<&Table> {
        self.talbes.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::MysqlRead;
    #[test]
    fn test_read_all() {
        let rd = MysqlRead::new(
            "mysql://root:123456@localhost:3306/information_schema",
            "demodb",
        );
        assert!(rd.talbes.contains_key("tm_test"));
    }
}
