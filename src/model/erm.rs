use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ErmObj {
    file_name: String,
    diagram: Diagram,
}

#[derive(Debug, Deserialize)]
pub struct Diagram {
    #[serde(rename = "dictionary")]
    dictionary: Dictionary,
    #[serde(rename = "contents")]
    pub contents: Contents,
}

impl Diagram {
    pub fn group_word(&self) -> HashMap<String, &Word> {
        let mut gm = HashMap::new();
        for it in self.dictionary.words.iter() {
            let id = match &it.id {
                Some(e) => e,
                None => panic!("erm 文件解析失败, word_id 为空"),
            };
            gm.insert(id.clone(), it);
        }
        gm
    }
}

#[derive(Debug, Deserialize)]
pub struct Dictionary {
    #[serde(rename = "word")]
    words: Vec<Word>,
}

#[derive(Debug, Deserialize)]
pub struct Contents {
    #[serde(rename = "table")]
    pub table: Vec<ErmTable>,
}

#[derive(Debug, Deserialize)]
pub struct ErmTable {
    id: Option<String>,
    pub physical_name: Option<String>,
    pub logical_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "columns")]
    pub columns: Columns,
    #[serde(rename = "indexes")]
    pub indexes: Indexes,
}
#[derive(Debug, Deserialize)]
pub struct Columns {
    #[serde(rename = "normal_column", default)]
    pub normal_column: Vec<NormalColumn>,
}

#[derive(Debug, Deserialize)]
pub struct Indexes {
    #[serde(rename = "inidex", default)]
    pub index: Vec<ErmInidex>,
}

#[derive(Debug, Deserialize)]
pub struct ErmInidex {
    full_text: Option<String>,
    pub non_unique: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    r#type: Option<String>,
    description: Option<String>,
    #[serde(rename = "columns")]
    pub columns: IndexColumns,
}

#[derive(Debug, Deserialize)]
pub struct IndexColumns {
    #[serde(rename = "column", default)]
    pub column: Vec<IndexColumn>,
}

#[derive(Debug, Deserialize)]
pub struct IndexColumn {
    pub id: Option<String>,
    desc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NormalColumn {
    pub word_id: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub unique_key_name: Option<String>,
    pub logical_name: Option<String>,
    pub physical_name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub constraint: Option<String>,
    pub default_value: Option<String>,
    pub auto_increment: Option<String>,
    pub foreign_key: Option<String>,
    pub not_null: Option<String>,
    pub primary_key: Option<String>,
    pub unique_key: Option<String>,
    pub character_set: Option<String>,
    pub collation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Word {
    pub id: Option<String>,
    pub length: Option<String>,
    pub decimal: Option<String>,
    pub array: Option<String>,
    pub array_dimension: Option<String>,
    pub unsigned: Option<String>,
    pub zerofill: Option<String>,
    pub binary: Option<String>,
    pub args: Option<String>,
    pub char_semantics: Option<String>,
    pub description: Option<String>,
    pub physical_name: Option<String>,
    pub logical_name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
