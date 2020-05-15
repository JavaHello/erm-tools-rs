mod ddl_out;
mod env;
mod erm_read;
mod md_out;
mod mysql_read;
mod tb_diff;

use crate::model::diff_table::DiffTable;
use crate::model::table::Table;
use std::collections::BTreeMap;
pub trait TbRead {
    fn read(&self, naem: &str) -> Option<&Table>;
}
pub use crate::core::ddl_out::DdlOut;
pub use crate::core::env::EnvConfig;
pub use crate::core::erm_read::ErmRead;
pub use crate::core::md_out::MdOut;
pub use crate::core::mysql_read::MysqlRead;
pub use crate::core::tb_diff::TableDiff;
pub trait Diff {
    fn diff(&mut self);
}

pub trait OutDiff {
    fn write(&mut self, diff_tables: &BTreeMap<String, DiffTable>);
}

pub fn load_env(config_path: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    env::load_env(config_path)
}

pub fn env() -> EnvConfig {
    env::get_env()
        .as_ref()
        .read()
        .unwrap()
        .as_ref()
        .unwrap()
        .clone()
}

pub fn exec(env: &mut EnvConfig) {
    match env.diff_type.as_str() {
        "" => panic!("diffType 必须配置"),
        "erm-erm" => {
            let source_erm_cfg = env.source_erm.take().expect("erm 源文件必须配置");
            let target_erm_cfg = env.target_erm.take().expect("erm 目标文件必须配置");
            let source_erm_list = source_erm_cfg
                .erm_files
                .iter()
                .map(|e| format!("{}{}", source_erm_cfg.erm_path, e))
                .collect();

            let target_erm_list = target_erm_cfg
                .erm_files
                .iter()
                .map(|e| format!("{}{}", target_erm_cfg.erm_path, e))
                .collect();

            let mut source_erm = ErmRead::new(source_erm_list);
            let mut target_erm = ErmRead::new(target_erm_list);

            let mut diff = TableDiff::new(&mut source_erm.talbes, &mut target_erm.talbes);
            diff.diff();
            let mut out = MdOut::new(&source_erm_cfg.db_name);
            out.write(&diff.diff);
            println!("{}", out.content);
        }
        "erm-db" => {
            let source_erm_cfg = env.source_erm.take().expect("erm 源文件必须配置");
            let target_db_cfg = env
                .target_db_list
                .take()
                .unwrap_or_else(|| vec![env.target_db.take().expect("target db 配置错误")]);
            let source_erm_list = source_erm_cfg
                .erm_files
                .iter()
                .map(|e| format!("{}{}", source_erm_cfg.erm_path, e))
                .collect();

            let mut source_erm = ErmRead::new(source_erm_list);

            for target_db_cfg in target_db_cfg {
                if "mysql".to_uppercase() == target_db_cfg.db_type.to_uppercase() {
                    let mut target_db = MysqlRead::new(
                        &format!(
                            "mysql://{}:{}@{}:{}/information_schema",
                            target_db_cfg.db_user,
                            target_db_cfg.db_password,
                            target_db_cfg.db_host,
                            target_db_cfg.db_port
                        ),
                        &target_db_cfg.db_name,
                    );
                    let mut diff = TableDiff::new(&mut source_erm.talbes, &mut target_db.talbes);
                    diff.diff();
                    let mut out = MdOut::new(&target_db_cfg.db_name);
                    out.write(&diff.diff);
                    println!("{}", out.content);

                    let mut out = DdlOut::new(&target_db_cfg.db_name);
                    out.write(&diff.diff);
                    println!("{}", out.content);
                } else {
                    panic!("不支持的数据库类型");
                }
            }
        }
        "db-db" => {
            let source_db_cfg = env.source_db.take().expect("erm 源文件必须配置");
            let target_db_cfg = env
                .target_db_list
                .take()
                .unwrap_or_else(|| vec![env.target_db.take().expect("target db 配置错误")]);

            let mut source_db = MysqlRead::new(
                &format!(
                    "mysql://{}:{}@{}:{}/information_schema",
                    source_db_cfg.db_user,
                    source_db_cfg.db_password,
                    source_db_cfg.db_host,
                    source_db_cfg.db_port
                ),
                &source_db_cfg.db_name,
            );

            for target_db_cfg in target_db_cfg {
                if "mysql".to_uppercase() == target_db_cfg.db_type.to_uppercase() {
                    let mut target_db = MysqlRead::new(
                        &format!(
                            "mysql://{}:{}@{}:{}/information_schema",
                            target_db_cfg.db_user,
                            target_db_cfg.db_password,
                            target_db_cfg.db_host,
                            target_db_cfg.db_port
                        ),
                        &target_db_cfg.db_name,
                    );
                    let mut diff = TableDiff::new(&mut source_db.talbes, &mut target_db.talbes);
                    diff.diff();
                    let mut out = MdOut::new(&target_db_cfg.db_name);
                    out.write(&diff.diff);
                    println!("{}", out.content);
                } else {
                    panic!("不支持的数据库类型");
                }
            }
        }
        "db-erm" => {
            let source_db_cfg = env.source_db.take().expect("erm 源文件必须配置");
            let target_erm_cfg = env.target_erm.take().expect("erm 目标文件必须配置");

            let target_erm_list = target_erm_cfg
                .erm_files
                .iter()
                .map(|e| format!("{}{}", target_erm_cfg.erm_path, e))
                .collect();

            let mut target_erm = ErmRead::new(target_erm_list);
            let mut source_db = MysqlRead::new(
                &format!(
                    "mysql://{}:{}@{}:{}/information_schema",
                    source_db_cfg.db_user,
                    source_db_cfg.db_password,
                    source_db_cfg.db_host,
                    source_db_cfg.db_port
                ),
                &source_db_cfg.db_name,
            );

            let mut diff = TableDiff::new(&mut source_db.talbes, &mut target_erm.talbes);
            diff.diff();
            let mut out = MdOut::new(&source_db_cfg.db_name);
            out.write(&diff.diff);
            println!("{}", out.content);
        }
        _ => panic!("diffType 未知配置项"),
    }
}
