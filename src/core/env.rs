use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

use std::sync::{Arc, OnceLock, RwLock};

const COV_TYPE_FILE_PREFIX: &str = "cov_type_";
const COV_TYPE_FILE_EXT: &str = ".json";
const COV_TYPE_PATH: &str = "./conf/";

static ENV: OnceLock<EnvConfig> = OnceLock::new();
type CovRw = Arc<RwLock<HashMap<String, HashMap<String, CovType>>>>;
static COV_SQL_TYPE: OnceLock<CovRw> = OnceLock::new();

fn init() {
    COV_SQL_TYPE
        .set(Arc::new(RwLock::new(HashMap::new())))
        .unwrap();
}

#[derive(Deserialize, Clone, Debug)]
pub struct CovType {
    pub name: String,
    pub length: Option<i32>,
    pub decimal: Option<i32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EnvConfig {
    #[serde(rename = "diffType")]
    pub diff_type: String,
    #[serde(rename = "dbType")]
    pub db_type: String,
    #[serde(rename = "outPath")]
    pub out_path: String,
    #[serde(rename = "genDdl")]
    pub gen_ddl: bool,
    #[serde(rename = "genMd", default)]
    pub gen_md: bool,
    #[serde(rename = "sourceErmFile")]
    pub source_erm: Option<ErmConfig>,
    #[serde(rename = "targetErmFile")]
    pub target_erm: Option<ErmConfig>,
    #[serde(rename = "sourceDb")]
    pub source_db: Option<DbConfig>,
    #[serde(rename = "targetDb")]
    pub target_db: Option<DbConfig>,
    #[serde(rename = "targetDbList")]
    pub target_db_list: Option<Vec<DbConfig>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ErmConfig {
    #[serde(rename = "dbName")]
    pub db_name: String,
    #[serde(rename = "ermFiles")]
    pub erm_files: Vec<String>,
    #[serde(rename = "ermPath")]
    pub erm_path: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DbConfig {
    #[serde(rename = "dbName")]
    pub db_name: String,
    #[serde(rename = "dbHost")]
    pub db_host: String,
    #[serde(rename = "dbUser")]
    pub db_user: String,
    #[serde(rename = "dbPassword")]
    pub db_password: String,
    #[serde(rename = "dbPort")]
    pub db_port: String,
}

impl DbConfig {
    pub fn get_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/information_schema",
            self.db_user, self.db_password, self.db_host, self.db_port
        )
    }
}
fn is_cov_type_file(s: &str) -> bool {
    // println!("查找的目录：{}", s);
    s.starts_with(COV_TYPE_FILE_PREFIX) && s.ends_with(COV_TYPE_FILE_EXT)
}
fn cov_type(file_name: &str) -> String {
    let file_name = file_name.replace(COV_TYPE_FILE_PREFIX, "");
    file_name.replace(COV_TYPE_FILE_EXT, "")
}

pub fn load_env(config_path: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    init();
    let config = fs::read_to_string(config_path)?; //.expect("读取配置文件失败!")
    let v: EnvConfig = serde_json::from_str(&config)?; //.expect("解析配置文件失败!")
    ENV.set(v).unwrap();
    for entry in fs::read_dir(COV_TYPE_PATH)? {
        let entry = entry.unwrap();
        let mtf = fs::File::open(entry.path());
        if let Some(fname) = entry.file_name().to_str() {
            if is_cov_type_file(fname) {
                let file_name = cov_type(fname);
                if let Ok(mut f) = mtf {
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)?;
                    let mt: HashMap<String, CovType> = serde_json::from_str(&contents)?;
                    cov_sql_type().write()?.insert(file_name, mt);
                };
            }
        }
    }

    Ok(())
}

fn cov_sql_type<'a>() -> &'a Arc<RwLock<HashMap<String, HashMap<String, CovType>>>> {
    COV_SQL_TYPE.get().unwrap()
}

pub fn get_env() -> EnvConfig {
    ENV.get().unwrap().clone()
}

pub fn get_mysql_cov_type() -> HashMap<String, CovType> {
    if let Some(t) = cov_sql_type().write().unwrap().get("mysql") {
        t.clone()
    } else {
        HashMap::<String, CovType>::new()
    }
}
