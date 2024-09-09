use std::collections::HashMap;

use console::measure_text_width;

#[derive(PartialEq)]
pub enum Alignment {
    Left,
    Right,
}

pub struct Table {
    head: Vec<String>,
    body: HashMap<String, Vec<String>>,
    rows: usize,
    widths: Vec<usize>,
    alignment: Vec<Alignment>,
}

impl Table {

    pub fn from_head(columns: Vec<String>, alignment: Vec<Alignment>) -> Self {
        let mut widths: Vec<usize> = Vec::new();
        let mut body: HashMap<String, Vec<String>> = HashMap::new();
        for name in &columns{
            widths.push(measure_text_width(name));
            body.insert(name.to_string(), Vec::new());
        }
        Self {
            head: columns,
            body: body,
            rows: 0,
            widths: widths,
            alignment: alignment,
        }
    }

    pub fn add_row(&mut self, columns: &Vec<String>) {
        if columns.len() != self.head.len() {
            panic!("table row length mismatch");
        }
        for (
            (name, value), width
        ) in self.head.iter().zip(columns.iter()).zip(self.widths.iter_mut()) {
            self.body.get_mut(name).unwrap().push(value.to_string());
            let value_len = measure_text_width(value);
            if value_len > *width {
                *width = value_len;
            }
        }
        self.rows += 1;
    }

    pub fn print(&mut self) {
        self.print_row(&self.head);
        self.print_bar();
        for idx in 0..self.rows {
            let mut row: Vec<String> = Vec::new();
            for name in &self.head {
                row.push(self.body.get_mut(name).unwrap().get(idx).unwrap().to_string());
            }
            self.print_row(&row)
        }
    }

    fn print_row(&self, row: &Vec<String>) {
        for ((column, width), align) in row.iter().zip(&self.widths).zip(&self.alignment) {
            let diff = width - measure_text_width(column);
            let buff = str::repeat(" ", diff);
            if *align == Alignment::Left {
                print!("| {}{} ", column, buff);
            } else if *align == Alignment::Right {
                print!("| {}{} ", buff, column);
            }
        }
        print!("|\n");
    }

    fn print_bar(&self) {
        for width in &self.widths {
            let buff = str::repeat("-", *width).to_string();
            print!("|-{}-", buff);
        }
        print!("|\n");
    }

}
