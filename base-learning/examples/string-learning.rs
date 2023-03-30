
fn main() {
    let sql = String::from("CREATE TABLE IF NOT EXISTS `meters` (`ts` timestamp,`c1` int,`c2` double,`c3` varchar(100),`__table_name__` varchar(100)) TAGS (`t1` bool,`t2` tinyint,`t3` smallint,`t4` int,`t5` bigint,`t6` tinyint unsigned,`t7` smallint unsigned,`t8` int unsigned,`t9` bigint unsigned,`t10` float,`t11` double,`t12` varchar(10))");
    println!("sql:{}", sql);
    let sql = sql.replace("__table_name__", "");
    println!("sql:{}", sql);
}