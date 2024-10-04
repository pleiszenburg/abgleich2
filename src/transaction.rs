use crate::cmd::Cmd;

use indexmap::IndexMap;

pub struct SnapshotMeta {

    written: u64,
    parent: String,
    name: String,

}

impl SnapshotMeta {

    pub fn as_table_row(&self) -> IndexMap<String, String> {

        let mut row: IndexMap<String, String> = IndexMap::new();

        row.insert("type".to_string(), "snapshot".to_string());
        row.insert("written".to_string(), format!("{}", self.written));
        row.insert("parent".to_string(), self.parent.clone());
        row.insert("name".to_string(), self.name.clone());

        row

    }

}

pub enum TransactionMeta {

    Snapshot(SnapshotMeta),

}

impl TransactionMeta {

    pub fn new_snapshot(written: u64, parent: String, name: String) -> Self {
        TransactionMeta::Snapshot(SnapshotMeta {
            written: written,
            parent: parent,
            name: name,
        })
    }

    pub fn as_table_row(&self) -> IndexMap<String, String> {
        match self {
            Self::Snapshot(meta) => {
                meta.as_table_row()
            }
        }
    }

}

pub struct Transaction {

    meta: TransactionMeta,
    cmd: Cmd,

}

impl Transaction {

    pub fn new(meta: TransactionMeta, cmd: Cmd) -> Self {
        Self {
            meta: meta,
            cmd: cmd,
        }
    }

    pub fn as_table_row(&self) -> IndexMap<String, String> {
        self.meta.as_table_row()
    }

    pub fn run(&self) -> (String, String) {
        self.cmd.run()
    }

}

pub struct TransactionList {

    transactions: Vec<Transaction>,

}

impl TransactionList {

    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn append(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

}
