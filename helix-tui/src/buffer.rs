use crate::{
    layout::Rect,
    style::Style,
    text::{Span, Spans},
};

pub use termwiz::surface::Surface as Buffer;

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

    fn set_spans<'a>(&mut self, x: u16, y: u16, spans: &Spans<'a>, width: u16) -> (u16, u16);

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
        (0, 0)
    }

    fn set_spans<'a>(&mut self, x: u16, y: u16, spans: &Spans<'a>, width: u16) -> (u16, u16) {
        (0, 0)
    }
}
