pub mod postgres_connector;
use postgres::transaction::Transaction;
use postgres::Connection;

pub trait StorageWithTransaction {
    fn with_connection<F, T>(&self, func: F) -> T
    where
        F: FnOnce(&Connection) -> T;

    fn with_transaction<F, T>(&self, func: F) -> T
    where
        F: FnOnce(&Transaction) -> T;
}
