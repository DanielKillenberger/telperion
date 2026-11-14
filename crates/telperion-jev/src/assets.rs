//! Image assets for a question about a picture: read from disk, held to the
//! API's documented limits before any call, and recorded by digest only.
//!
//! Limits from https://docs.typesafe.ai/images.md ("Limits"): PNG or JPEG,
//! 16 MiB per file, 280 to 1568 pixels on each side. An asset's name is its
//! reference string: wherever it appears in the state, instructions or
//! criteria, the API puts the image in its place.

use std::fs;

use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::sha256_hex;

pub const MAX_FILE_BYTES: usize = 16 * 1024 * 1024;
pub const MIN_SIDE: u32 = 280;
pub const MAX_SIDE: u32 = 1568;
/// Images are off unless every request that carries one sends this header.
pub const IMAGES_HEADER: &str = "TypeSafe-Danger-Zone-Images";

/// What the ledger keeps of an asset: never its bytes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetRef {
    pub name: String,
    pub path: String,
    pub sha256: String,
}

pub struct ImageAsset {
    pub record: AssetRef,
    format: &'static str,
    bytes: Vec<u8>,
}

/// Reads every asset of a `{name: path}` map, refusing the first one outside
/// the limits by name.
pub fn load_assets(map: &Map<String, Value>) -> Result<Vec<ImageAsset>, String> {
    map.iter()
        .map(|(name, path)| {
            let path = path
                .as_str()
                .ok_or_else(|| format!("asset {name}: expected a file path"))?;
            load_asset(name, path)
        })
        .collect()
}

pub fn load_asset(name: &str, path: &str) -> Result<ImageAsset, String> {
    let bytes = fs::read(path).map_err(|err| format!("asset {name} ({path}): {err}"))?;
    if bytes.len() > MAX_FILE_BYTES {
        return Err(format!(
            "asset {name} ({path}): {} bytes, over the 16 MiB per-image limit",
            bytes.len()
        ));
    }
    let (format, width, height) =
        image_dims(&bytes).ok_or_else(|| format!("asset {name} ({path}): not a PNG or JPEG"))?;
    let side = MIN_SIDE..=MAX_SIDE;
    if !side.contains(&width) || !side.contains(&height) {
        return Err(format!(
            "asset {name} ({path}): {width}x{height}, each side must be {MIN_SIDE} to {MAX_SIDE} pixels"
        ));
    }
    Ok(ImageAsset {
        record: AssetRef {
            name: name.to_string(),
            path: path.to_string(),
            sha256: sha256_hex(&bytes),
        },
        format,
        bytes,
    })
}

/// The request's `assets` map.
pub fn payload(assets: &[ImageAsset]) -> Value {
    let engine = base64::engine::general_purpose::STANDARD;
    let map = assets
        .iter()
        .map(|asset| {
            let entry = json!({
                "type": "image",
                "format": asset.format,
                "data_base64": engine.encode(&asset.bytes),
            });
            (asset.record.name.clone(), entry)
        })
        .collect();
    Value::Object(map)
}

/// The state checksum, extended by each asset's name and digest, so two
/// calls on the same state with different pictures keep distinct identities.
pub fn content_sha256(state_sha256: &str, assets: &[AssetRef]) -> String {
    if assets.is_empty() {
        return state_sha256.to_string();
    }
    let mut text = state_sha256.to_string();
    for asset in assets {
        text.push_str(&format!("\n{}={}", asset.name, asset.sha256));
    }
    sha256_hex(text.as_bytes())
}

/// Format and pixel size from the file's own header.
fn image_dims(bytes: &[u8]) -> Option<(&'static str, u32, u32)> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") && bytes.get(12..16)? == b"IHDR" {
        let width = u32::from_be_bytes(bytes.get(16..20)?.try_into().ok()?);
        let height = u32::from_be_bytes(bytes.get(20..24)?.try_into().ok()?);
        return Some(("png", width, height));
    }
    if bytes.starts_with(&[0xFF, 0xD8]) {
        return jpeg_dims(bytes).map(|(width, height)| ("jpg", width, height));
    }
    None
}

/// Walks the JPEG segments to the first start-of-frame marker.
fn jpeg_dims(bytes: &[u8]) -> Option<(u32, u32)> {
    let mut at = 2;
    loop {
        if *bytes.get(at)? != 0xFF {
            return None;
        }
        let marker = *bytes.get(at + 1)?;
        if marker == 0xFF {
            at += 1;
            continue;
        }
        let len = u16::from_be_bytes([*bytes.get(at + 2)?, *bytes.get(at + 3)?]) as usize;
        let start_of_frame = matches!(marker, 0xC0..=0xCF) && !matches!(marker, 0xC4 | 0xC8 | 0xCC);
        if start_of_frame {
            let height = u16::from_be_bytes([*bytes.get(at + 5)?, *bytes.get(at + 6)?]);
            let width = u16::from_be_bytes([*bytes.get(at + 7)?, *bytes.get(at + 8)?]);
            return Some((width as u32, height as u32));
        }
        at += 2 + len;
    }
}
