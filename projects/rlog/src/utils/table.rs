use tabled::{
    Table, Tabled,
    settings::{Color, Style, object::Rows},
};

/// Display table from `records` using `tabled::Table`.
pub fn display_table<T: Tabled>(records: &[T]) {
    let mut table = Table::new(records);
    table
        .with(Style::modern_rounded())
        .modify(Rows::first(), Color::FG_BRIGHT_BLUE);

    println!("{}", table);
}
