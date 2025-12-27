use zhconv::{zhconv, Variant};
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ConversionMode {
    S2T, // Simplified to Traditional
    T2S, // Traditional to Simplified
}

pub fn convert(text: &str, mode: ConversionMode) -> String {
    match mode {
        ConversionMode::S2T => zhconv(text, Variant::ZhHant),
        ConversionMode::T2S => zhconv(text, Variant::ZhHans),
    }
}