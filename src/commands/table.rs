use prettytable::{format, Cell, Row, Table};

fn create_row(values: Vec<String>) -> Row {
    let cells: Vec<Cell> = values.iter().map(|cell| Cell::new(cell)).collect();

    Row::new(cells)
}

pub fn print_table(titles: Vec<String>, mut rows: Vec<Vec<String>>) {
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(create_row(titles));
    rows.sort();

    for row_values in rows {
        table.add_row(create_row(row_values));
    }

    println!();
    table.printstd();
}
