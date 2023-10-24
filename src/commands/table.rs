use bat::{PagingMode, PrettyPrinter};
use tabled::{builder::Builder, settings::Style};

pub fn print_table(titles: Vec<String>, rows: Vec<Vec<String>>) {
    let mut table = Builder::default();

    table.set_header(titles);

    for values in rows {
        table.push_record(values);
    }

    let table_bytes = table.build().with(Style::rounded()).to_string();

    PrettyPrinter::new()
        .input_from_bytes(table_bytes.as_bytes())
        .colored_output(false)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .print()
        .unwrap();
}
