use crate::cmd::Cmd;
use crate::misc::colorized_storage_si_suffix;
use crate::table::{Alignment, Table};

pub struct SnapshotMeta {

    written: u64,
    parent: String,
    name: String,

}

impl SnapshotMeta {

    pub fn to_string(&self) -> String {

        format!(
            "New Snapshot: {}@{} ({})",
            self.parent,
            self.name,
            colorized_storage_si_suffix(self.written),
        )

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

    pub fn to_string(&self) -> String {
        match self {
            Self::Snapshot(meta) => {
                meta.to_string()
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

    pub fn to_string(&self) -> (String, String) {
        (self.meta.to_string(), self.cmd.to_string())
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

    pub fn print_table(&self) {
        let mut table = Table::from_head(
            vec![
                "description".to_string(),
                "command".to_string(),
            ],
            vec![
                Alignment::Left,
                Alignment::Left,
            ]
        );
        for transaction in self.transactions.iter() {
            let (description, command) = transaction.to_string();
            table.add_row(&vec![description, command]);
        }

        table.print();
    }

}
