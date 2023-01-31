//! Palettes and color manipulation.

use iced::Color;

/// A simple box palette constisting of colors for background text and border.
#[derive(Clone, Copy, Debug)]
pub struct ContrastPalette {
    /// Bright color of this palette, in light mode is background.
    pub bright: Color,
    /// Dim color of this palette, in light mode is text and border.
    pub dim: Color,
}

impl ContrastPalette {
    /// Get a monochrome (black and white) color palette.
    #[must_use]
    pub fn monochrome() -> Self {
        Self {
            bright: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            dim: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }

    /// Mute the dim color.
    #[must_use]
    pub fn mute_dim(self, t: Option<f32>) -> Self {
        Self {
            dim: self.dim.mute(t),
            ..self
        }
    }

    /// Swap the dim and bright fields.
    #[must_use]
    pub fn invert(self) -> Self {
        let Self { bright, dim } = self;
        Self {
            bright: dim,
            dim: bright,
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let [a, b, t] = [a, b, t].map(|n| n.clamp(0.0, 1.0));
    (a + t * (b - a)).clamp(0.0, 1.0)
}

impl Default for ContrastPalette {
    fn default() -> Self {
        Self::monochrome()
    }
}

/// Trait to extend [Color][iced::Color] with some methods.
#[allow(clippy::module_name_repetitions)]
pub trait ColorManipExt {
    /// Mute the if given by amount t 0..1.
    fn mute(self, t: Option<f32>) -> Self;

    /// Lerp between two colors.
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl ColorManipExt for Color {
    fn lerp(self, other: Self, t: f32) -> Self {
        let Color {
            r: r1,
            g: g1,
            b: b1,
            a: a1,
        } = self;
        let Color {
            r: r2,
            g: g2,
            b: b2,
            a: a2,
        } = other;
        Color {
            r: lerp(r1, r2, t),
            g: lerp(g1, g2, t),
            b: lerp(b1, b2, t),
            a: lerp(a1, a2, t),
        }
    }
    fn mute(self, t: Option<f32>) -> Color {
        let Color { r, g, b, a } = self;
        let t = t.unwrap_or(0.25);
        Color {
            r: lerp(r, 0.5, t),
            g: lerp(g, 0.5, t),
            b: lerp(b, 0.5, t),
            a,
        }
    }
}
