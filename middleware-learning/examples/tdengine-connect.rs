use taos::*;

fn main() {
    // let builder = TaosBuilder::from_dsn("taos+ws://192.168.1.92:56041/").unwrap();
    // let taos = builder.build().unwrap();
    // futures::executor::block_on(taos.exec("drop database if exists stt1")).unwrap();
    // println!("create database");
    // futures::executor::block_on(taos.query("create database if not exists stt1 keep 36500"))
    //     .unwrap();
    // futures::executor::block_on(taos.exec("use stt1")).unwrap();
    // futures::executor::block_on(taos.exec(
    //     // "create stable if not exists st1(ts timestamp, v int) tags(jt json)"
    //     "create stable if not exists st1(ts timestamp, v int) tags(jt int, t1 varchar(32))",
    // ))
    // .unwrap();

    // let mut stmt = Stmt::init(&taos).unwrap();
    // let sql = "insert into ? using st1 tags(?, ?) values(?, ?)";
    // stmt.prepare(sql).unwrap();

    // // let tags = vec![TaosBind::from_json(r#"{"name":"value"}"#)];
    // let tbname = "tb1";
    // let tags = vec![Value::Int(0), Value::VarChar(String::from("123"))];
    // stmt.set_tbname_tags(tbname, &tags).unwrap();
    // drop(tags);
    // let params = vec![
    //     ColumnView::from_millis_timestamp(vec![0]),
    //     ColumnView::from_ints(vec![0]),
    // ];
    // stmt.bind(&params).unwrap();
    // println!("bind");

    // let params = vec![
    //     ColumnView::from_millis_timestamp(vec![1]),
    //     ColumnView::from_ints(vec![0]),
    // ];
    // stmt.bind(&params);
    // println!("add batch");

    // stmt.add_batch();
    // println!("execute");
    // stmt.execute();

    // assert_eq!(stmt.affected_rows(), 2);
    // println!("done");
}