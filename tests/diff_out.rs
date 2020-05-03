use erm_tools::core::MdOut;
use erm_tools::core::{Diff, OutDiff, TableDiff};
use erm_tools::core::{ErmRead, MysqlRead};

#[test]
fn test_diff_out() {
    let mut erm1 = ErmRead::new(vec!["erms/db.erm".to_owned()]);
    let mut erm2 = ErmRead::new(vec!["erms/db2.erm".to_owned()]);
    let mut diff = TableDiff::new(&mut erm1.talbes, &mut erm2.talbes);
    diff.diff();
    let mut out = MdOut::new();
    out.write(&diff.diff);
    assert_eq!(
        "# 差异输出

## tm_test
|new名称|new类型|new长度|new精度||old名称|old类型|old长度|old精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|name|character|32|0||name|character|64|0|
|id_no|varchar|64|0||||||
||||||email|varchar|64|0|

## tm_test 索引差异
|new名称|new字段|new类型||old名称|old字段|old类型|
|:-:|:-:|:-:|:-:|:-:|:-:|
|idx_tm_test_name_01|name, age|普通|||||
|||||idx_tm_test_name_01|name|普通|

## tm_test2
|new名称|new类型|new长度|new精度||old名称|old类型|old长度|old精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|id|integer|0|0||||||
|name|character|64|0||||||
|age|integer|0|0||||||
|birthday|datetime|0|0||||||
|create_datetime|datetime|0|0||||||
|last_update_datetime|datetime|0|0||||||
|email|varchar|64|0||||||

## tm_test2 索引差异
|new名称|new字段|new类型||old名称|old字段|old类型|
|:-:|:-:|:-:|:-:|:-:|:-:|
|pk|id|主键|||||
|idx_tm_test_name_01||普通|||||

## tm_test_all
|new名称|new类型|new长度|new精度||old名称|old类型|old长度|old精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|id|integer|0|0||||||
|name|character|32|0||||||
|last_update_datetime|datetime|0|0||||||
|id_no|varchar|64|0||||||
|p1|bigint|0|0||||||
|p2|bigint|1|0||||||
|p3|binary1|0|0||||||
|p4|bit|0|0||||||
|p5|bit|2|0||||||
|p6|blob|0|0||||||
|p7|boolean|0|0||||||

## tm_test_all 索引差异
|new名称|new字段|new类型||old名称|old字段|old类型|
|:-:|:-:|:-:|:-:|:-:|:-:|
|pk|id|主键|||||
|Idx_id_no_01|id_no|普通|||||
",
        out.content
    );
}

#[test]
fn test_db_diff_out() {
    let mut erm1 = ErmRead::new(vec!["erms/db.erm".to_owned()]);
    let mut db = MysqlRead::new(
        "mysql://root:123456@localhost:3306/information_schema",
        "demodb",
    );
    let mut diff = TableDiff::new(&mut erm1.talbes, &mut db.talbes);
    diff.diff();
    let mut out = MdOut::new();
    out.write(&diff.diff);
    println!("{}", out.content);
}
