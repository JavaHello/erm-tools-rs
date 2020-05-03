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

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct A {
    name: String,
}
#[derive(Debug)]
struct B {
    name: String,
}
#[derive(Debug)]
struct C {
    list_a: Vec<Rc<RefCell<A>>>,
    list_b: Vec<Rc<B>>,
}

#[test]
fn test_rc() {
    let mut c = C {
        list_a: Vec::new(),
        list_b: Vec::new(),
    };
    let a = Rc::new(RefCell::new(A {
        name: String::from("zhang"),
    }));
    println!("{:p}", a.as_ref());
    c.list_a.push(a);
    f2(&c);
    c.list_b.push(Rc::new(B {
        name: String::from("li"),
    }));
    println!("{:?}", c.list_a);
}

fn f2(c: &C) {
    let mut c2 = C {
        list_a: Vec::new(),
        list_b: Vec::new(),
    };

    for c1a in c.list_a.iter() {
        println!("{:p}", c1a.as_ref());
        let mut a = (*c1a).borrow_mut();
        a.name = String::from("我改了");
        c2.list_a.push(Rc::clone(c1a));
    }
}
