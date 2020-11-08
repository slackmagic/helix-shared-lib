use postgres::transaction::Transaction;
use postgres::{Connection, TlsMode};

pub struct PostgresConnector {
    db_conn: Connection,
}

impl PostgresConnector {
    pub fn new(conn_string: String) -> PostgresConnector {
        let t_connection: Connection = Connection::connect(conn_string, TlsMode::None).unwrap();
        PostgresConnector {
            db_conn: t_connection,
        }
    }

    pub fn get_connection(&self) -> &Connection {
        &self.db_conn
    }

    pub fn get_transaction(&self) -> Transaction {
        self.db_conn.transaction().unwrap()
    }

    pub fn with_connection<F, T>(&self, func: F) -> T
    where
        F: FnOnce(&Connection) -> T,
    {
        func(&self.db_conn)
    }

    pub fn with_transaction<F, T>(&self, func: F) -> T
    where
        F: FnOnce(&Transaction) -> T,
    {
        let trans: Transaction = self.get_transaction();
        let ret: T = func(&trans);
        &trans.commit();
        ret
    }
}
