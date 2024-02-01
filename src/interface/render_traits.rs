use crate::engine::Color as EngineColor;
use sdl2::pixels::Color as SdlColor;

pub trait ScreenColor {
    fn screen_color(&self) -> SdlColor;
}

impl ScreenColor for EngineColor {
    fn screen_color(&self) -> SdlColor {
        match self {
            //NOTE: Tango color palette
            EngineColor::Yellow => SdlColor::RGB(0xed, 0xd4, 0x00),
            EngineColor::Cyan => SdlColor::RGB(0x72, 0x9f, 0xcf),
            EngineColor::Purple => SdlColor::RGB(0x75, 0x50, 0x7b),
            EngineColor::Orange => SdlColor::RGB(0xf5, 0x79, 0x00),
            EngineColor::Blue => SdlColor::RGB(0x34, 0x65, 0xa4),
            EngineColor::Green => SdlColor::RGB(0x73, 0xd2, 0x16),
            EngineColor::Red => SdlColor::RGB(0xef, 0x29, 0x29),
        }
    }
}
