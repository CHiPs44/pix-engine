//! [SVG 1.0 Color Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).

use super::{Color, ColorMode::*};
use crate::prelude::Scalar;

/// Const constructor helper.
const fn rgb_const(lr: Scalar, lg: Scalar, lb: Scalar, r: u8, g: u8, b: u8) -> Color {
    Color {
        mode: Rgb,
        levels: [lr, lg, lb, 1.0],
        channels: [r, g, b, 255],
    }
}

pub use colors::*;

#[allow(missing_docs)]
mod colors {
    use super::*;

    pub const ALICE_BLUE: Color = rgb_const(0.9411, 0.9725, 1.0, 0xF0, 0xF8, 0xFF);
    pub const ANTIQUE_WHITE: Color = rgb_const(0.9803, 0.9215, 0.8431, 0xFA, 0xEB, 0xD7);
    pub const AQUA: Color = rgb_const(0.0, 1.0, 1.0, 0x0, 0xFF, 0xFF);
    pub const AQUA_MARINE: Color = rgb_const(0.4980, 1.0, 0.8313, 0x7F, 0xFF, 0xD4);
    pub const AZURE: Color = rgb_const(0.9411, 1.0, 1.0, 0xF0, 0xFF, 0xFF);
    pub const BEIGE: Color = rgb_const(0.9607, 0.9607, 0.8627, 0xF5, 0xF5, 0xDC);
    pub const BISQUE: Color = rgb_const(1.0, 0.8941, 0.7686, 0xFF, 0xE4, 0xC4);
    pub const BLACK: Color = rgb_const(0.0, 0.0, 0.0, 0x0, 0x0, 0x0);
    pub const BLANCHE_DALMOND: Color = rgb_const(1.0, 0.9215, 0.8039, 0xFF, 0xEB, 0xCD);
    pub const BLUE: Color = rgb_const(0.0, 0.0, 1.0, 0x0, 0x0, 0xFF);
    pub const BLUE_VIOLET: Color = rgb_const(0.5411, 0.1686, 0.8862, 0x8A, 0x2B, 0xE2);
    pub const BROWN: Color = rgb_const(0.6470, 0.1647, 0.1647, 0xA5, 0x2A, 0x2A);
    pub const BURLY_WOOD: Color = rgb_const(0.8705, 0.7215, 0.5294, 0xDE, 0xB8, 0x87);
    pub const CADET_BLUE: Color = rgb_const(0.3725, 0.6196, 0.6274, 0x5F, 0x9E, 0xA0);
    pub const CHARTREUSE: Color = rgb_const(0.4980, 1.0, 0.0, 0x7F, 0xFF, 0x0);
    pub const CHOCOLATE: Color = rgb_const(0.8235, 0.4117, 0.1176, 0xD2, 0x69, 0x1E);
    pub const CORAL: Color = rgb_const(1.0, 0.4980, 0.3137, 0xFF, 0x7F, 0x50);
    pub const CORNFLOWER_BLUE: Color = rgb_const(0.3921, 0.5843, 0.9294, 0x64, 0x95, 0xED);
    pub const CORN_SILK: Color = rgb_const(1.0, 0.9725, 0.8627, 0xFF, 0xF8, 0xDC);
    pub const CRIMSON: Color = rgb_const(0.8627, 0.0784, 0.2352, 0xDC, 0x14, 0x3C);
    pub const CYAN: Color = rgb_const(0.0, 1.0, 1.0, 0x0, 0xFF, 0xFF);
    pub const DARK_BLUE: Color = rgb_const(0.0, 0.0, 0.5450, 0x0, 0x0, 0x8B);
    pub const DARK_CYAN: Color = rgb_const(0.0, 0.5450, 0.5450, 0x0, 0x8B, 0x8B);
    pub const DARK_GOLDENROD: Color = rgb_const(0.7215, 0.5254, 0.0431, 0xB8, 0x86, 0xB);
    pub const DARK_GRAY: Color = rgb_const(0.6627, 0.6627, 0.6627, 0xA9, 0xA9, 0xA9);
    pub const DARK_GREEN: Color = rgb_const(0.0, 0.3921, 0.0, 0x0, 0x64, 0x0);
    pub const DARK_GREY: Color = rgb_const(0.6627, 0.6627, 0.6627, 0xA9, 0xA9, 0xA9);
    pub const DARK_KHAKI: Color = rgb_const(0.7411, 0.7176, 0.4196, 0xBD, 0xB7, 0x6B);
    pub const DARK_MAGENTA: Color = rgb_const(0.5450, 0.0, 0.5450, 0x8B, 0x0, 0x8B);
    pub const DARK_OLIVE_GREEN: Color = rgb_const(0.3333, 0.4196, 0.1843, 0x55, 0x6B, 0x2F);
    pub const DARK_ORANGE: Color = rgb_const(1.0, 0.5490, 0.0, 0xFF, 0x8C, 0x0);
    pub const DARK_ORCHID: Color = rgb_const(0.6, 0.1960, 0.8, 0x99, 0x32, 0xCC);
    pub const DARK_RED: Color = rgb_const(0.5450, 0.0, 0.0, 0x8B, 0x0, 0x0);
    pub const DARK_SALMON: Color = rgb_const(0.9137, 0.5882, 0.4784, 0xE9, 0x96, 0x7A);
    pub const DARK_SEA_GREEN: Color = rgb_const(0.5607, 0.7372, 0.5607, 0x8F, 0xBC, 0x8F);
    pub const DARK_SLATE_BLUE: Color = rgb_const(0.2823, 0.2392, 0.5450, 0x48, 0x3D, 0x8B);
    pub const DARK_SLATE_GRAY: Color = rgb_const(0.1843, 0.3098, 0.3098, 0x2F, 0x4F, 0x4F);
    pub const DARK_SLATE_GREY: Color = rgb_const(0.1843, 0.3098, 0.3098, 0x2F, 0x4F, 0x4F);
    pub const DARK_TURQUOISE: Color = rgb_const(0.0, 0.8078, 0.8196, 0x0, 0xCE, 0xD1);
    pub const DARK_VIOLET: Color = rgb_const(0.5803, 0.0, 0.8274, 0x94, 0x0, 0xD3);
    pub const DEEP_PINK: Color = rgb_const(1.0, 0.0784, 0.5764, 0xFF, 0x14, 0x93);
    pub const DEEP_SKY_BLUE: Color = rgb_const(0.0, 0.7490, 1.0, 0x0, 0xBF, 0xFF);
    pub const DIM_GRAY: Color = rgb_const(0.4117, 0.4117, 0.4117, 0x69, 0x69, 0x69);
    pub const DIM_GREY: Color = rgb_const(0.4117, 0.4117, 0.4117, 0x69, 0x69, 0x69);
    pub const DODGER_BLUE: Color = rgb_const(0.1176, 0.5647, 1.0, 0x1E, 0x90, 0xFF);
    pub const FIRE_BRICK: Color = rgb_const(0.6980, 0.1333, 0.1333, 0xB2, 0x22, 0x22);
    pub const FLORAL_WHITE: Color = rgb_const(1.0, 0.9803, 0.9411, 0xFF, 0xFA, 0xF0);
    pub const FOREST_GREEN: Color = rgb_const(0.1333, 0.5450, 0.1333, 0x22, 0x8B, 0x22);
    pub const FUCHSIA: Color = rgb_const(1.0, 0.0, 1.0, 0xFF, 0x0, 0xFF);
    pub const GAINSBORO: Color = rgb_const(0.8627, 0.8627, 0.8627, 0xDC, 0xDC, 0xDC);
    pub const GHOST_WHITE: Color = rgb_const(0.9725, 0.9725, 1.0, 0xF8, 0xF8, 0xFF);
    pub const GOLD: Color = rgb_const(1.0, 0.8431, 0.0, 0xFF, 0xD7, 0x0);
    pub const GOLDENROD: Color = rgb_const(0.8549, 0.6470, 0.1254, 0xDA, 0xA5, 0x20);
    pub const GRAY: Color = rgb_const(0.5019, 0.5019, 0.5019, 0x80, 0x80, 0x80);
    pub const GREEN: Color = rgb_const(0.0, 0.5019, 0.0, 0x0, 0x80, 0x0);
    pub const GREEN_YELLOW: Color = rgb_const(0.6784, 1.0, 0.1843, 0xAD, 0xFF, 0x2F);
    pub const GREY: Color = rgb_const(0.5019, 0.5019, 0.5019, 0x80, 0x80, 0x80);
    pub const HONEYDEW: Color = rgb_const(0.9411, 1.0, 0.9411, 0xF0, 0xFF, 0xF0);
    pub const HOTOINK: Color = rgb_const(1.0, 0.4117, 0.7058, 0xFF, 0x69, 0xB4);
    pub const INDIAN_RED: Color = rgb_const(0.8039, 0.3607, 0.3607, 0xCD, 0x5C, 0x5C);
    pub const INDIGO: Color = rgb_const(0.2941, 0.0, 0.5098, 0x4B, 0x0, 0x82);
    pub const IVORY: Color = rgb_const(1.0, 1.0, 0.9411, 0xFF, 0xFF, 0xF0);
    pub const KHAKI: Color = rgb_const(0.9411, 0.9019, 0.5490, 0xF0, 0xE6, 0x8C);
    pub const LAVENDER: Color = rgb_const(0.9019, 0.9019, 0.9803, 0xE6, 0xE6, 0xFA);
    pub const LAVENDER_BLUSH: Color = rgb_const(1.0, 0.9411, 0.9607, 0xFF, 0xF0, 0xF5);
    pub const LAWN_GREEN: Color = rgb_const(0.4862, 0.9882, 0.0, 0x7C, 0xFC, 0x0);
    pub const LEMON_CHIFFON: Color = rgb_const(1.0, 0.9803, 0.8039, 0xFF, 0xFA, 0xCD);
    pub const LIGHT_BLUE: Color = rgb_const(0.6784, 0.8470, 0.9019, 0xAD, 0xD8, 0xE6);
    pub const LIGHT_CORAL: Color = rgb_const(0.9411, 0.5019, 0.5019, 0xF0, 0x80, 0x80);
    pub const LIGHT_CYAN: Color = rgb_const(0.8784, 1.0, 1.0, 0xE0, 0xFF, 0xFF);
    pub const LIGHT_GOLDENROD_YELLOW: Color = rgb_const(0.9803, 0.9803, 0.8235, 0xFA, 0xFA, 0xD2);
    pub const LIGHT_GRAY: Color = rgb_const(0.8274, 0.8274, 0.8274, 0xD3, 0xD3, 0xD3);
    pub const LIGHT_GREEN: Color = rgb_const(0.5647, 0.9333, 0.5647, 0x90, 0xEE, 0x90);
    pub const LIGHT_GREY: Color = rgb_const(0.8274, 0.8274, 0.8274, 0xD3, 0xD3, 0xD3);
    pub const LIGHT_PINK: Color = rgb_const(1.0, 0.7137, 0.7568, 0xFF, 0xB6, 0xC1);
    pub const LIGHT_SALMON: Color = rgb_const(1.0, 0.6274, 0.4784, 0xFF, 0xA0, 0x7A);
    pub const LIGHT_SEA_GREEN: Color = rgb_const(0.1254, 0.6980, 0.6666, 0x20, 0xB2, 0xAA);
    pub const LIGHT_SKY_BLUE: Color = rgb_const(0.5294, 0.8078, 0.9803, 0x87, 0xCE, 0xFA);
    pub const LIGHT_SLATE_GRAY: Color = rgb_const(0.4666, 0.5333, 0.6, 0x77, 0x88, 0x99);
    pub const LIGHT_SLATE_GREY: Color = rgb_const(0.4666, 0.5333, 0.6, 0x77, 0x88, 0x99);
    pub const LIGHT_STEEL_BLUE: Color = rgb_const(0.6901, 0.7686, 0.8705, 0xB0, 0xC4, 0xDE);
    pub const LIGHT_YELLOW: Color = rgb_const(1.0, 1.0, 0.8784, 0xFF, 0xFF, 0xE0);
    pub const LIME: Color = rgb_const(0.0, 1.0, 0.0, 0x0, 0xFF, 0x0);
    pub const LIME_GREEN: Color = rgb_const(0.1960, 0.8039, 0.1960, 0x32, 0xCD, 0x32);
    pub const LINEN: Color = rgb_const(0.9803, 0.9411, 0.9019, 0xFA, 0xF0, 0xE6);
    pub const MAGENTA: Color = rgb_const(1.0, 0.0, 1.0, 0xFF, 0x0, 0xFF);
    pub const MAROON: Color = rgb_const(0.5019, 0.0, 0.0, 0x80, 0x0, 0x0);
    pub const MEDIUMAQUA_MARINE: Color = rgb_const(0.4, 0.8039, 0.6666, 0x66, 0xCD, 0xAA);
    pub const MEDIUM_BLUE: Color = rgb_const(0.0, 0.0, 0.8039, 0x0, 0x0, 0xCD);
    pub const MEDIUM_ORCHID: Color = rgb_const(0.7294, 0.3333, 0.8274, 0xBA, 0x55, 0xD3);
    pub const MEDIUM_PURPLE: Color = rgb_const(0.5764, 0.4392, 0.8588, 0x93, 0x70, 0xDB);
    pub const MEDIUM_SEA_GREEN: Color = rgb_const(0.2352, 0.7019, 0.4431, 0x3C, 0xB3, 0x71);
    pub const MEDIUM_SLATE_BLUE: Color = rgb_const(0.4823, 0.4078, 0.9333, 0x7B, 0x68, 0xEE);
    pub const MEDIUM_SPRING_GREEN: Color = rgb_const(0.0, 0.9803, 0.6039, 0x0, 0xFA, 0x9A);
    pub const MEDIUM_TURQUOISE: Color = rgb_const(0.2823, 0.8196, 0.8, 0x48, 0xD1, 0xCC);
    pub const MEDIUM_VIOLET_RED: Color = rgb_const(0.7803, 0.0823, 0.5215, 0xC7, 0x15, 0x85);
    pub const MIDNIGHT_BLUE: Color = rgb_const(0.0980, 0.0980, 0.4392, 0x19, 0x19, 0x70);
    pub const MINT_CREAM: Color = rgb_const(0.9607, 1.0, 0.9803, 0xF5, 0xFF, 0xFA);
    pub const MISTY_ROSE: Color = rgb_const(1.0, 0.8941, 0.8823, 0xFF, 0xE4, 0xE1);
    pub const MOCCASIN: Color = rgb_const(1.0, 0.8941, 0.7098, 0xFF, 0xE4, 0xB5);
    pub const NAVAJO_WHITE: Color = rgb_const(1.0, 0.8705, 0.6784, 0xFF, 0xDE, 0xAD);
    pub const NAVY: Color = rgb_const(0.0, 0.0, 0.5019, 0x0, 0x0, 0x80);
    pub const OLD_LACE: Color = rgb_const(0.9921, 0.9607, 0.9019, 0xFD, 0xF5, 0xE6);
    pub const OLIVE: Color = rgb_const(0.5019, 0.5019, 0.0, 0x80, 0x80, 0x0);
    pub const OLIVE_DRAB: Color = rgb_const(0.4196, 0.5568, 0.1372, 0x6B, 0x8E, 0x23);
    pub const ORANGE: Color = rgb_const(1.0, 0.64705, 0.0, 0xFF, 0xA5, 0x0);
    pub const ORANGE_RED: Color = rgb_const(1.0, 0.2705, 0.0, 0xFF, 0x45, 0x0);
    pub const ORCHID: Color = rgb_const(0.8549, 0.4392, 0.8392, 0xDA, 0x70, 0xD6);
    pub const PALE_GOLDENROD: Color = rgb_const(0.9333, 0.9098, 0.6666, 0xEE, 0xE8, 0xAA);
    pub const PALE_GREEN: Color = rgb_const(0.5960, 0.9843, 0.5960, 0x98, 0xFB, 0x98);
    pub const PALE_TURQUOISE: Color = rgb_const(0.6862, 0.9333, 0.9333, 0xAF, 0xEE, 0xEE);
    pub const PALE_VIOLET_RED: Color = rgb_const(0.8588, 0.4392, 0.5764, 0xDB, 0x70, 0x93);
    pub const PAPAYA_WHIP: Color = rgb_const(1.0, 0.9372, 0.8352, 0xFF, 0xEF, 0xD5);
    pub const PEACH_PUFF: Color = rgb_const(1.0, 0.85490, 0.7254, 0xFF, 0xDA, 0xB9);
    pub const PERU: Color = rgb_const(0.8039, 0.5215, 0.2470, 0xCD, 0x85, 0x3F);
    pub const PINK: Color = rgb_const(1.0, 0.7529, 0.7960, 0xFF, 0xC0, 0xCB);
    pub const PLUM: Color = rgb_const(0.8666, 0.6274, 0.8666, 0xDD, 0xA0, 0xDD);
    pub const POWDER_BLUE: Color = rgb_const(0.6901, 0.8784, 0.9019, 0xB0, 0xE0, 0xE6);
    pub const PURPLE: Color = rgb_const(0.5019, 0.0, 0.5019, 0x80, 0x0, 0x80);
    pub const REBECCA_PURPLE: Color = rgb_const(0.4, 0.2, 0.6, 0x66, 0x33, 0x99);
    pub const RED: Color = rgb_const(1.0, 0.0, 0.0, 0xFF, 0x0, 0x0);
    pub const ROSY_BROWN: Color = rgb_const(0.7372, 0.5607, 0.5607, 0xBC, 0x8F, 0x8F);
    pub const ROYAL_BLUE: Color = rgb_const(0.2549, 0.4117, 0.8823, 0x41, 0x69, 0xE1);
    pub const SADDLE_BROWN: Color = rgb_const(0.5450, 0.2705, 0.0745, 0x8B, 0x45, 0x13);
    pub const SALMON: Color = rgb_const(0.9803, 0.5019, 0.4470, 0xFA, 0x80, 0x72);
    pub const SANDY_BROWN: Color = rgb_const(0.9568, 0.6431, 0.3764, 0xF4, 0xA4, 0x60);
    pub const SEA_GREEN: Color = rgb_const(0.1803, 0.5450, 0.3411, 0x2E, 0x8B, 0x57);
    pub const SEA_SHELL: Color = rgb_const(1.0, 0.9607, 0.9333, 0xFF, 0xF5, 0xEE);
    pub const SIENNA: Color = rgb_const(0.6274, 0.3215, 0.1764, 0xA0, 0x52, 0x2D);
    pub const SILVER: Color = rgb_const(0.7529, 0.7529, 0.7529, 0xC0, 0xC0, 0xC0);
    pub const SKY_BLUE: Color = rgb_const(0.5294, 0.8078, 0.9215, 0x87, 0xCE, 0xEB);
    pub const SLATE_BLUE: Color = rgb_const(0.4156, 0.3529, 0.8039, 0x6A, 0x5A, 0xCD);
    pub const SLATE_GRAY: Color = rgb_const(0.4392, 0.5019, 0.5647, 0x70, 0x80, 0x90);
    pub const SLATE_GREY: Color = rgb_const(0.4392, 0.5019, 0.5647, 0x70, 0x80, 0x90);
    pub const SNOW: Color = rgb_const(1.0, 0.9803, 0.9803, 0xFF, 0xFA, 0xFA);
    pub const SPRING_GREEN: Color = rgb_const(0.0, 1.0, 0.4980, 0x0, 0xFF, 0x7F);
    pub const STEEL_BLUE: Color = rgb_const(0.2745, 0.5098, 0.7058, 0x46, 0x82, 0xB4);
    pub const TAN: Color = rgb_const(0.8235, 0.7058, 0.5490, 0xD2, 0xB4, 0x8C);
    pub const TEAL: Color = rgb_const(0.0, 0.5019, 0.5019, 0x0, 0x80, 0x80);
    pub const THISTLE: Color = rgb_const(0.8470, 0.7490, 0.8470, 0xD8, 0xBF, 0xD8);
    pub const TOMATO: Color = rgb_const(1.0, 0.3882, 0.2784, 0xFF, 0x63, 0x47);
    pub const TRANSPARENT: Color = rgb_const(0.0, 0.0, 0.0, 0x0, 0x0, 0x0);
    pub const TURQUOISE: Color = rgb_const(0.2509, 0.8784, 0.8156, 0x40, 0xE0, 0xD0);
    pub const VIOLET: Color = rgb_const(0.9333, 0.5098, 0.9333, 0xEE, 0x82, 0xEE);
    pub const WHEAT: Color = rgb_const(0.9607, 0.8705, 0.7019, 0xF5, 0xDE, 0xB3);
    pub const WHITE: Color = rgb_const(1.0, 1.0, 1.0, 0xFF, 0xFF, 0xFF);
    pub const WHITE_SMOKE: Color = rgb_const(0.9607, 0.9607, 0.9607, 0xF5, 0xF5, 0xF5);
    pub const YELLOW: Color = rgb_const(1.0, 1.0, 0.0, 0xFF, 0xFF, 0x0);
    pub const YELLOW_GREEN: Color = rgb_const(0.6039, 0.8039, 0.1960, 0x9A, 0xCD, 0x32);
}