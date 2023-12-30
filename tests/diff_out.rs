use erm_tools::core;
use erm_tools::core::{DdlOut, MdOut};
use erm_tools::core::{Diff, OutDiff, TableDiff};
use erm_tools::core::{ErmRead, MysqlRead};

#[test]
fn test_diff_out() {
    core::load_env("./erm-tools.json").unwrap();
    let mut erm1 = ErmRead::new(vec!["erms/db.erm".to_owned()]);
    let mut erm2 = ErmRead::new(vec!["erms/db2.erm".to_owned()]);
    let mut diff = TableDiff::new(&mut erm1.talbes, &mut erm2.talbes);
    diff.diff();
    let mut out = MdOut::new("demodb");
    out.write(&diff.diff);
    assert_eq!(
        "# demodb差异输出

## tm_test
|S名称|S类型|S长度|S精度||T名称|T类型|T长度|T精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|name|char|32|||name|char|64||
|id_no|varchar|64|||||||
||||||email|varchar|64||

## tm_test 索引差异
|S名称|S字段|S类型||T名称|T字段|T类型|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|idx_tm_test_name_01|name, age|普通|||||
|||||idx_tm_test_name_01|name|普通|

## tm_test2
|S名称|S类型|S长度|S精度||T名称|T类型|T长度|T精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|id|int||||||||
|name|char|64|||||||
|age|int||||||||
|birthday|datetime||||||||
|create_datetime|datetime||||||||
|last_update_datetime|datetime||||||||
|email|varchar|64|||||||

## tm_test2 索引差异
|S名称|S字段|S类型||T名称|T字段|T类型|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|pk|id|主键|||||
|idx_tm_test_name_01||普通|||||

## tm_test_all
|S名称|S类型|S长度|S精度||T名称|T类型|T长度|T精度|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|id|int||||||||
|name|char|32|||||||
|last_update_datetime|datetime||||||||
|id_no|varchar|64|||||||
|p1|bigint||||||||
|p2|bigint||||||||
|p3|binary1||||||||
|p4|bit||||||||
|p5|bit|2|||||||
|p6|blob||||||||
|p7|boolean||||||||

## tm_test_all 索引差异
|S名称|S字段|S类型||T名称|T字段|T类型|
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|pk|id|主键|||||
|Idx_id_no_01|id_no|普通|||||
",
        out.content
    );
    let mut out = DdlOut::new("demodb");
    out.write(&diff.diff);
    assert_eq!(
        "-- demodb
alter table tm_test modify column name char(32);
alter table tm_test add column id_no varchar(64);
alter table tm_test drop column email;
alter table tm_test add index idx_tm_test_name_01(name, age);
drop index idx_tm_test_name_01 on tm_test;
create table tm_test2 (
    id int comment '主键id',
    name char(64) comment '名称',
    age int comment '年龄',
    birthday datetime comment '生日',
    create_datetime datetime comment '创建时间',
    last_update_datetime datetime comment '最后修改时间',
    email varchar(64) comment '邮箱',
    key (),
    primary key (id)
) comment 'tm_test2';
create table tm_test_all (
    id int comment '主键id',
    name char(32) comment '名称',
    last_update_datetime datetime comment '最后修改时间',
    id_no varchar(64) comment '身份证号',
    p1 bigint comment 'p1',
    p2 bigint comment 'p2',
    p3 binary1 comment 'p3',
    p4 bit comment 'p4',
    p5 bit(2) comment 'p5',
    p6 blob comment 'p6',
    p7 boolean comment 'p7',
    key (id_no asc),
    primary key (id)
) comment 'tm_test_all';
",
        out.content
    );
}

#[test]
#[ignore]
fn test_db_diff_out() {
    let mut erm1 = ErmRead::new(vec!["erms/db.erm".to_owned()]);
    let mut db = MysqlRead::new(
        "mysql://root:123456@localhost:3306/information_schema",
        "demodb",
    );
    let mut diff = TableDiff::new(&mut erm1.talbes, &mut db.talbes);
    diff.diff();
    let mut out = MdOut::new("demodb");
    out.write(&diff.diff);
    println!("{}", out.content);
}

// 场景 多个 C 对象需要共享  A, B 数据，对A需要有修改权限
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct A {
    name: String,
}
impl Drop for A {
    fn drop(&mut self) {
        println!("A drop: {}", self.name);
    }
}

impl Drop for B {
    fn drop(&mut self) {
        println!("B drop: {}", self.name);
    }
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
    let a = A {
        name: String::from("zhang"),
    };
    let a = Rc::new(RefCell::new(a));
    let b = Rc::new(B {
        name: String::from("wu"),
    });
    println!("a ptr: {:p}", a);
    println!("b ptr: {:p}", b);
    c.list_a.push(a);
    c.list_b.push(b);
    f2(&c);
    c.list_b.push(Rc::new(B {
        name: String::from("li"),
    }));
    println!("{:?}", c);
}

fn f2(c: &C) {
    let mut c2 = C {
        list_a: Vec::new(),
        list_b: Vec::new(),
    };

    for c1a in c.list_a.iter() {
        println!("a ptr: {:p}", *c1a);
        let mut a = (*c1a).borrow_mut();
        a.name = String::from("我改了");
        c2.list_a.push(Rc::clone(c1a));
    }
    for c1b in c.list_b.iter() {
        println!("b ptr: {:p}", *c1b);
        c2.list_b.push(Rc::clone(c1b));
    }
    println!("{:?}", c2);
}
