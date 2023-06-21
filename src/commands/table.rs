use bat::{PagingMode, PrettyPrinter};
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

    let table_bytes = table.to_string();

    PrettyPrinter::new()
        .input_from_bytes(table_bytes.as_bytes())
        .colored_output(false)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .print()
        .unwrap();
}
