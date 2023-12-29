use sqlx::mysql::MySqlRow;
use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, MySqlPool};
use crate::model::{Group, User};

// 2. 定义 Table 结构体，它代表一个数据库表。
// 它有一个 MySqlPool（数据库连接池），
// 一个从 MySqlRow 转换到 T 类型的函数，
// 和一个 PhantomData（用于生命周期标记）：
pub struct Table<'c, T>
    where T: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'c MySqlRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
    where T: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}


// 定义 JoinTable 结构体，它代表一个连接两个数据库表的表。
// 它有一个 MySqlPool，
// 两个从 MySqlRow 转换到 T1 和 T2 类型的函数，和两个 PhantomData：
pub struct JoinTable<'c, T1, T2>
    where T1: FromRow<'c, MySqlRow>,
          T2: FromRow<'c, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: (
        fn(&'c MySqlRow) -> Result<T1, sqlx::Error>,
        fn(&'c MySqlRow) -> Result<T2, sqlx::Error>,
    ),
    _marker_t1: PhantomData<&'c T1>,
    _marker_t2: PhantomData<&'c T2>,
}

impl<'c, T1, T2> JoinTable<'c, T1, T2>
    where T1: FromRow<'c, MySqlRow>,
          T2: FromRow<'c, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        JoinTable {
            pool,
            _from_row: (T1::from_row, T2::from_row),
            _marker_t1: PhantomData,
            _marker_t2: PhantomData,
        }
    }
}

// 6. 定义 Database 结构体，它代表一个数据库。
// 它有三个字段，
// 分别代表 Group 表、User 表和连接 User 和 Group 的表：
pub struct Database<'c> {
    pub groups: Arc<Table<'c, Group>>,
    pub users: Arc<Table<'c, User>>,
    pub users_to_groups: Arc<JoinTable<'c, User, Group>>,
}

// 7. 为 Database 结构体实现 new 方法，用于创建一个新的 Database 实例。
// 这个方法首先连接到数据库，然后创建 Group 表、User 表和连接 User 和 Group 的表：
impl<'a> Database<'a> {
    pub async fn new(sql_url: &String) -> Database<'a> {
        let connection = MySqlPool::connect(&sql_url).await.unwrap();
        let pool = Arc::new(connection);

        Database {
            groups: Arc::from(Table::new(pool.clone())),
            users: Arc::from(Table::new(pool.clone())),
            users_to_groups: Arc::from(JoinTable::new(pool.clone())),
        }
    }
}