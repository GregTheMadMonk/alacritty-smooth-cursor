//! Convert a cursor into an iterator of rects.

use alacritty_terminal::vte::ansi::CursorShape;

use crate::display::SizeInfo;
use crate::display::color::Rgb;
use crate::display::content::RenderableCursor;
use crate::renderer::rects::RenderRect;

/// Trait for conversion into the iterator.
pub trait IntoRects {
    /// Consume the cursor for an iterator of rects.
    fn rects(self, size_info: &SizeInfo, thickness: f32, block_replace: Option<CursorShape>) -> CursorRects;
}

impl IntoRects for RenderableCursor {
    fn rects(self, size_info: &SizeInfo, thickness: f32, block_replace: Option<CursorShape>) -> CursorRects {
        let point = self.point();
        let x = point.column.0 as f32 * size_info.cell_width() + size_info.padding_x();
        let y = point.line as f32 * size_info.cell_height() + size_info.padding_y();

        let mut width = size_info.cell_width();
        let height = size_info.cell_height();

        let thickness = (thickness * width).round().max(1.);

        width *= self.width().get() as f32;

        let shape = match block_replace {
            None => self.shape(),
            Some(block_replace) => match self.shape() {
                CursorShape::Beam
                | CursorShape::Underline
                | CursorShape::HollowBlock => self.shape(),
                _ => block_replace
            }
        };
        match shape {
            CursorShape::Beam => beam(x, y, height, thickness, self.color()),
            CursorShape::Underline => underline(x, y, width, height, thickness, self.color()),
            CursorShape::HollowBlock => hollow(x, y, width, height, thickness, self.color()),
            CursorShape::Block => RenderRect::new_cur(x, y, width, height, self.color(), 1.0).into(),
            CursorShape::Hidden => RenderRect::new_cur(x, y, width, height, self.color(), 0.0).into(),
        }
    }
}

/// Cursor rect iterator.
#[derive(Default, Clone, Copy)]
pub struct CursorRects {
    rects: [Option<RenderRect>; 4],
    index: usize,
}

impl CursorRects {
    pub fn interpolate(
        &mut self,
        other: &Self,
        fps_cur: f32,
        factor: f32,
        spring: f32,
        max_s_x: f32,
        max_s_y: f32
    ) -> bool {
        let mut changed = false;

        let adjust = if fps_cur > 0.0 { 60.0 / fps_cur } else { 1.0 };
        let true_factor = (factor * adjust).min(1.0);
        for (mine, theirs) in self.rects.iter_mut().zip(other.rects.iter()) {
            *mine = match &mine {
                Some(mine_v) => match theirs {
                    Some(theirs_v) => Some(
                        mine_v.interpolate(
                            theirs_v, true_factor, spring, max_s_x, max_s_y,
                            &mut changed
                        )
                    ),
                    None => {
                        changed = true;
                        None
                    }
                }
                None => {
                    changed |= !theirs.is_none();
                    *theirs
                }
            }
        }

        changed
    }
}

impl From<RenderRect> for CursorRects {
    fn from(rect: RenderRect) -> Self {
        Self { rects: [Some(rect), None, None, None], index: 0 }
    }
}

impl Iterator for CursorRects {
    type Item = RenderRect;

    fn next(&mut self) -> Option<Self::Item> {
        let rect = self.rects.get_mut(self.index)?;
        self.index += 1;
        rect.take()
    }
}

/// Create an iterator yielding a single beam rect.
fn beam(x: f32, y: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    RenderRect::new_cur(x, y, thickness, height, color, 1.).into()
}

/// Create an iterator yielding a single underline rect.
fn underline(x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    let y = y + height - thickness;
    RenderRect::new_cur(x, y, width, thickness, color, 1.).into()
}

/// Create an iterator yielding a rect for each side of the hollow block cursor.
fn hollow(x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    let top_line = RenderRect::new_cur(x, y, width, thickness, color, 1.);

    let vertical_y = y + thickness;
    let vertical_height = height - 2. * thickness;
    let left_line = RenderRect::new_cur(x, vertical_y, thickness, vertical_height, color, 1.);

    let bottom_y = y + height - thickness;
    let bottom_line = RenderRect::new_cur(x, bottom_y, width, thickness, color, 1.);

    let right_x = x + width - thickness;
    let right_line = RenderRect::new_cur(right_x, vertical_y, thickness, vertical_height, color, 1.);

    CursorRects {
        rects: [Some(top_line), Some(bottom_line), Some(left_line), Some(right_line)],
        index: 0,
    }
}
