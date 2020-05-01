// Table 数据库表结构
#[derive(Debug)]
pub struct Table {
    pub physical_name: String,
    pub logical_name: String,
    pub description: String,
    pub columns: Vec<Column>,
    pub primary_keys: Vec<Column>,
    pub indexes: Vec<Index>,
}

// Column 字段信息
#[derive(Debug, Clone)]
pub struct Column {
    pub physical_name: String,
    pub logical_name: String,
    pub r#type: String,
    pub auto_increment: bool,
    pub default_value: String,
    pub length: i32,
    pub decimal: i32,
    pub primary_key: bool,
    pub unique_key: bool,
    pub not_null: bool,
    pub description: String,
    pub desc: bool,
    pub column_type: String,
}

// Index 索引信息
#[derive(Debug, Clone)]
pub struct Index {
    pub name: String,
    pub non_unique: bool,
    pub columns: Vec<Column>,
}
