use crate::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
};

use termwiz::color::ColorAttribute;
pub use termwiz::surface::Surface as Buffer;
use termwiz::{cell::*, color::*, surface::*};

impl Into<ColorAttribute> for Color {
    fn into(self) -> ColorAttribute {
        match self {
            Color::Reset => ColorAttribute::Default,
            Color::Black => AnsiColor::Black.into(),
            Color::Gray | Color::DarkGray => AnsiColor::Grey.into(),
            Color::Red => AnsiColor::Maroon.into(),
            Color::LightRed => AnsiColor::Red.into(),
            Color::Green => AnsiColor::Green.into(),
            Color::LightGreen => AnsiColor::Lime.into(),
            Color::Yellow => AnsiColor::Olive.into(),
            Color::LightYellow => AnsiColor::Yellow.into(),
            Color::Magenta => AnsiColor::Purple.into(),
            Color::LightMagenta => AnsiColor::Fuschia.into(),
            Color::Cyan => AnsiColor::Teal.into(),
            Color::LightCyan => AnsiColor::Aqua.into(),
            Color::White => AnsiColor::White.into(),
            Color::Blue => AnsiColor::Navy.into(),
            Color::LightBlue => AnsiColor::Blue.into(),
            Color::Indexed(i) => ColorAttribute::PaletteIndex(i),
            Color::Rgb(r, g, b) => {
                ColorAttribute::TrueColorWithDefaultFallback(RgbColor::new(r, g, b))
            }
        }
    }
}

pub struct Cell {}

impl Cell {
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Cell {
        self
    }
    pub fn set_style(&mut self, style: Style) -> &mut Cell {
        self
    }
}

pub trait SurfaceExt {
    //
    fn set_style(&mut self, area: Rect, style: Style);

    fn clear_with(&mut self, area: Rect, style: Style) {}

    fn set_string<S>(&mut self, x: u16, y: u16, string: S, style: Style)
    where
        S: AsRef<str>,
    {
        self.set_stringn(x, y, string, usize::MAX, style);
    }

    fn set_stringn<S>(
        &mut self,
        x: u16,
        y: u16,
        string: S,
        width: usize,
        style: Style,
    ) -> (u16, u16)
    where
        S: AsRef<str>;

    fn set_spans<'a>(&mut self, x: u16, y: u16, spans: &Spans<'a>, width: u16) -> (u16, u16) {
        let mut remaining_width = width;
        let mut x = x;
        for span in &spans.0 {
            if remaining_width == 0 {
                break;
            }
            let pos = self.set_stringn(
                x,
                y,
                span.content.as_ref(),
                remaining_width as usize,
                span.style,
            );
            let w = pos.0.saturating_sub(x);
            x = pos.0;
            remaining_width = remaining_width.saturating_sub(w);
        }
        (x, y)
    }

    fn set_span<'a>(&mut self, x: u16, y: u16, span: &Span<'a>, width: u16) -> (u16, u16) {
        self.set_stringn(x, y, span.content.as_ref(), width as usize, span.style)
    }

    fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        unimplemented!()
    }
}

impl SurfaceExt for termwiz::surface::Surface {
    //
    fn set_style(&mut self, area: Rect, style: Style) {}

    fn set_stringn<S>(
        &mut self,
        x: u16,
        y: u16,
        string: S,
        width: usize,
        style: Style,
    ) -> (u16, u16)
    where
        S: AsRef<str>,
    {
        // TODO: style and limit to width
        self.add_change(Change::CursorPosition {
            x: Position::Absolute(x as usize),
            y: Position::Absolute(y as usize),
        });
        let fg = style.fg.unwrap_or(Color::Reset);
        self.add_change(Change::Attribute(AttributeChange::Foreground(fg.into())));
        let bg = style.bg.unwrap_or(Color::Reset);
        self.add_change(Change::Attribute(AttributeChange::Background(bg.into())));
        self.add_change(Change::Text(string.as_ref().to_owned()));

        (0, 0)
    }
}
