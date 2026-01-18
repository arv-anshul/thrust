use std::io::{Write, stdout};
use termimad::crossterm::event::Event;
use termimad::crossterm::style::Color;
use termimad::crossterm::{
    cursor::{Hide, Show},
    event::{self, KeyCode::*, KeyEvent},
    queue,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use termimad::*;

/// Display Markdown text in `stdout` using `termimad` crate.
pub fn display_markdown(markdown: String) {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120);

    let mut skin = MadSkin::default();
    skin.set_headers_fg(Color::DarkMagenta);
    skin.table.align = Alignment::Center;
    skin.bold.set_fg(Color::Yellow);
    skin.italic.set_fg(Color::Blue);

    let mut view = MadView::from(markdown, area, skin);
    make_view_scrollable(&mut view).expect("Error rendering markdown");
}

/// Copied from https://github.com/Canop/termimad/blob/main/examples/scrollable/main.rs
fn make_view_scrollable(view: &mut MadView) -> Result<(), Error> {
    let mut w = stdout();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // Hiding the cursor

    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}
