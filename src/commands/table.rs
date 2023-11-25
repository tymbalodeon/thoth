use bat::{PagingMode, PrettyPrinter};
use tabled::{builder::Builder, settings::Style};

pub fn print_table(titles: Vec<String>, rows: Vec<Vec<String>>) {
    let mut builder = Builder::default();

    if !titles.is_empty() {
        builder.set_header(&titles);
    }

    for values in rows {
        builder.push_record(values);
    }

    let mut table = &mut builder.build();

    if titles.is_empty() {
        table = table.with(Style::rounded().remove_horizontals());
    } else {
        table = table.with(Style::rounded());
    }

    let table_bytes = table.to_string();

    PrettyPrinter::new()
        .input_from_bytes(table_bytes.as_bytes())
        .colored_output(false)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .print()
        .unwrap();
}
