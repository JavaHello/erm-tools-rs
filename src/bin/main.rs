use erm_tools::core::MdOut;
use erm_tools::core::{Diff, OutDiff, TableDiff};
use erm_tools::core::{ErmRead, MysqlRead};

use clap::Clap;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clap)]
#[clap(version = "1.0", author = "luokai")]
struct Opts {
    #[clap(short, long, default_value = "erm-tools.json")]
    config: String,
    // #[clap(subcommand)]
    // subcmd: SubCommand,
}

#[derive(Deserialize, Debug)]
struct EnvConfig {
    #[serde(rename = "diffType")]
    diff_type: String,
    #[serde(rename = "outPath")]
    out_path: String,
    #[serde(rename = "genDdl")]
    gen_ddl: bool,
    #[serde(rename = "genMd", default)]
    gen_md: bool,
    #[serde(rename = "sourceErmFile")]
    source_erm: Option<ErmConfig>,
    #[serde(rename = "targetErmFile")]
    target_erm: Option<ErmConfig>,
    #[serde(rename = "sourceDb")]
    source_db: Option<DbConfig>,
    #[serde(rename = "targetDb")]
    target_db: Option<DbConfig>,
    #[serde(rename = "targetDbList")]
    target_db_list: Option<Vec<DbConfig>>,
}
impl EnvConfig {
    fn exec(&mut self) {
        match self.diff_type.as_str() {
            "" => panic!("diffType 必须配置"),
            "erm-erm" => {
                let source_erm_cfg = self.source_erm.take().expect("erm 源文件必须配置");
                let target_erm_cfg = self.target_erm.take().expect("erm 目标文件必须配置");
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
                let source_erm_cfg = self.source_erm.take().expect("erm 源文件必须配置");
                let target_db_cfg = self
                    .target_db_list
                    .take()
                    .unwrap_or_else(|| vec![self.target_db.take().expect("target db 配置错误")]);
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
                        let mut diff =
                            TableDiff::new(&mut source_erm.talbes, &mut target_db.talbes);
                        diff.diff();
                        let mut out = MdOut::new(&target_db_cfg.db_name);
                        out.write(&diff.diff);
                        println!("{}", out.content);
                    } else {
                        panic!("不支持的数据库类型");
                    }
                }
            }
            "db-db" => {
                let source_db_cfg = self.source_db.take().expect("erm 源文件必须配置");
                let target_db_cfg = self
                    .target_db_list
                    .take()
                    .unwrap_or_else(|| vec![self.target_db.take().expect("target db 配置错误")]);

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
                let source_db_cfg = self.source_db.take().expect("erm 源文件必须配置");
                let target_erm_cfg = self.target_erm.take().expect("erm 目标文件必须配置");

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
}

#[derive(Deserialize, Debug)]
struct ErmConfig {
    #[serde(rename = "dbName")]
    db_name: String,
    #[serde(rename = "ermFiles")]
    erm_files: Vec<String>,
    #[serde(rename = "ermPath")]
    erm_path: String,
}

#[derive(Deserialize, Debug)]
struct DbConfig {
    #[serde(rename = "dbName")]
    db_name: String,
    #[serde(rename = "dbHost")]
    db_host: String,
    #[serde(rename = "dbUser")]
    db_user: String,
    #[serde(rename = "dbPassword")]
    db_password: String,
    #[serde(rename = "dbPort")]
    db_port: String,
    #[serde(rename = "dbType")]
    db_type: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let config = fs::read_to_string(opts.config).expect("读取配置文件失败!");
    let mut v: EnvConfig = serde_json::from_str(&config).expect("解析配置文件失败!");
    v.exec();
}
