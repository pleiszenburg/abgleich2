use std::{collections::HashMap, hash::Hash};

pub struct Table {
    head: Vec<String>,
    body: HashMap<String, Vec<String>>,
    rows: usize,
    widths: Vec<usize>,
}

impl Table {

    pub fn from_head(columns: Vec<String>) -> Self {
        let mut widths: Vec<usize> = Vec::new();
        let mut body: HashMap<String, Vec<String>> = HashMap::new();
        for name in &columns{
            widths.push(name.len());
            body.insert(name.to_string(), Vec::new());
        }
        Self {
            head: columns,
            body: body,
            rows: 0,
            widths: widths,
        }
    }

    pub fn add_row(&mut self, columns: Vec<String>) {
        if columns.len() != self.head.len() {
            panic!("table row length mismatch");
        }
        for (
            (name, value), width
        ) in self.head.iter().zip(columns.iter()).zip(self.widths.iter_mut()) {
            self.body.get_mut(name).unwrap().push(value.to_string());
            if value.len() > *width {
                *width = value.len();
            }
        }
        self.rows += 1;
    }

    pub fn print(&mut self) {
        self.print_row(&self.head);
        for idx in 0..self.rows {
            let mut row: Vec<String> = Vec::new();
            for name in &self.head {
                row.push(self.body.get_mut(name).unwrap().get(idx).unwrap().to_string());
            }
            self.print_row(&row)
        }
    }

    fn print_row(&self, row: &Vec<String>) {
        for column in row {
            print!("| {} ", column);
        }
        print!("|\n");
    }

}
