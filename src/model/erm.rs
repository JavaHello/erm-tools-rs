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
            gm.insert(it.id.clone(), it);
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
    #[serde(rename = "id", default)]
    id: String,
    #[serde(rename = "physical_name", default)]
    pub physical_name: String,
    #[serde(rename = "logical_name", default)]
    pub logical_name: String,
    #[serde(rename = "description", default)]
    pub description: String,
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
    #[serde(rename = "full_text", default)]
    full_text: String,
    #[serde(rename = "non_unique", default)]
    pub non_unique: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "type", default)]
    r#type: String,
    #[serde(rename = "description", default)]
    description: String,
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
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "desc", default)]
    desc: String,
}

#[derive(Debug, Deserialize)]
pub struct NormalColumn {
    #[serde(rename = "word_id", default)]
    pub word_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub unique_key_name: String,
    #[serde(default)]
    pub logical_name: String,
    #[serde(default)]
    pub physical_name: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(default)]
    pub constraint: String,
    #[serde(default)]
    pub default_value: String,
    #[serde(default)]
    pub auto_increment: String,
    #[serde(default)]
    pub foreign_key: String,
    #[serde(default)]
    pub not_null: String,
    #[serde(default)]
    pub primary_key: String,
    #[serde(default)]
    pub unique_key: String,
    #[serde(default)]
    pub character_set: String,
    #[serde(default)]
    pub collation: String,
}

#[derive(Debug, Deserialize)]
pub struct Word {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub length: String,
    #[serde(default)]
    pub decimal: String,
    #[serde(default)]
    pub array: String,
    #[serde(default)]
    pub array_dimension: String,
    #[serde(default)]
    pub unsigned: String,
    #[serde(default)]
    pub zerofill: String,
    #[serde(default)]
    pub binary: String,
    #[serde(default)]
    pub args: String,
    #[serde(default)]
    pub char_semantics: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub physical_name: String,
    #[serde(default)]
    pub logical_name: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
}
