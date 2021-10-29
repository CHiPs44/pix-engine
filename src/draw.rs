//! Drawing functions.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;
use std::iter::Iterator;

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw shape to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()>;
}

impl PixState {
    /// Clears the render target to the current background [Color] set by [PixState::background].
    #[inline]
    pub fn clear(&mut self) -> PixResult<()> {
        let color = self.settings.background;
        self.renderer.set_draw_color(self.settings.background)?;
        self.renderer.clear()?;
        self.renderer.set_draw_color(color)?;
        Ok(())
    }

    /// Draw a wireframe to the current canvas., translated to a given [Point]
    pub fn wireframe<V, P, T>(&mut self, vertexes: V, pos: P, angle: T, scale: T) -> PixResult<()>
    where
        V: AsRef<[PointF2]>,
        P: Into<PointF2>,
        T: AsPrimitive<Scalar>,
    {
        let s = &self.settings;
        let pos: PointF2 = pos.into();
        let scale: Scalar = scale.as_();
        let mut angle: Scalar = angle.as_();
        if let AngleMode::Degrees = s.angle_mode {
            angle = angle.to_radians();
        };
        let (sin, cos) = angle.sin_cos();
        let vs: Vec<PointI2> = vertexes
            .as_ref()
            .iter()
            .map(|v| {
                point!(
                    ((v.x() * cos - v.y() * sin) * scale + pos.x()).round(),
                    ((v.x() * sin + v.y() * cos) * scale + pos.y()).round()
                )
                .into()
            })
            .collect();
        self.renderer.polygon(&vs, s.fill, s.stroke)
    }
}