//! WebAssembly bindings for LERC compression library.
//!
//! This module provides JavaScript-friendly APIs for encoding and decoding
//! LERC-compressed raster data in the browser or Node.js.

use wasm_bindgen::prelude::*;

use crate::{decode, encode, get_blob_info, BlobInfo, LercError};

/// LERC data types corresponding to the C API.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LercDataTypeWasm {
    Int8 = 0,
    UInt8 = 1,
    Int16 = 2,
    UInt16 = 3,
    Int32 = 4,
    UInt32 = 5,
    Float32 = 6,
    Float64 = 7,
}

/// Metadata about a LERC blob, exposed to JavaScript.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct LercBlobInfo {
    version: u32,
    data_type: u32,
    depth: u32,
    width: u32,
    height: u32,
    bands: u32,
    valid_pixel_count: u32,
    blob_size: u32,
    masks: u32,
}

#[wasm_bindgen]
impl LercBlobInfo {
    /// Codec version used to create the blob.
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Encoded data type (0=i8, 1=u8, 2=i16, 3=u16, 4=i32, 5=u32, 6=f32, 7=f64).
    #[wasm_bindgen(getter)]
    pub fn data_type(&self) -> u32 {
        self.data_type
    }

    /// Number of values per pixel (depth).
    #[wasm_bindgen(getter)]
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Width of the image in pixels.
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height of the image in pixels.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Number of bands.
    #[wasm_bindgen(getter)]
    pub fn bands(&self) -> u32 {
        self.bands
    }

    /// Number of valid pixels.
    #[wasm_bindgen(getter)]
    pub fn valid_pixel_count(&self) -> u32 {
        self.valid_pixel_count
    }

    /// Size of compressed blob in bytes.
    #[wasm_bindgen(getter)]
    pub fn blob_size(&self) -> u32 {
        self.blob_size
    }

    /// Number of validity masks.
    #[wasm_bindgen(getter)]
    pub fn masks(&self) -> u32 {
        self.masks
    }
}

impl From<BlobInfo> for LercBlobInfo {
    fn from(info: BlobInfo) -> Self {
        LercBlobInfo {
            version: info.version,
            data_type: info.data_type,
            depth: info.depth,
            width: info.width,
            height: info.height,
            bands: info.bands,
            valid_pixel_count: info.valid_pixel_count,
            blob_size: info.blob_size,
            masks: info.masks,
        }
    }
}

fn lerc_error_to_js(e: LercError) -> JsValue {
    JsValue::from_str(&format!("{}", e))
}

/// Get metadata from a LERC-compressed blob without decoding.
///
/// @param blob - The compressed LERC data as Uint8Array
/// @returns LercBlobInfo with metadata about the blob
#[wasm_bindgen(js_name = getBlobInfo)]
pub fn get_blob_info_wasm(blob: &[u8]) -> Result<LercBlobInfo, JsValue> {
    get_blob_info(blob)
        .map(LercBlobInfo::from)
        .map_err(lerc_error_to_js)
}

/// Decode a LERC blob to Float32Array.
///
/// @param blob - The compressed LERC data
/// @param width - Image width in pixels
/// @param height - Image height in pixels
/// @param depth - Values per pixel
/// @param bands - Number of bands
/// @param masks - Number of validity masks
/// @returns Float32Array with decoded data
#[wasm_bindgen(js_name = decodeFloat32)]
pub fn decode_f32_wasm(
    blob: &[u8],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
) -> Result<js_sys::Float32Array, JsValue> {
    let (data, _mask) = decode::<f32>(
        blob,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Float32Array::from(&data[..]))
}

/// Decode a LERC blob to Float64Array.
#[wasm_bindgen(js_name = decodeFloat64)]
pub fn decode_f64_wasm(
    blob: &[u8],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
) -> Result<js_sys::Float64Array, JsValue> {
    let (data, _mask) = decode::<f64>(
        blob,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Float64Array::from(&data[..]))
}

/// Decode a LERC blob to Uint8Array.
#[wasm_bindgen(js_name = decodeUint8)]
pub fn decode_u8_wasm(
    blob: &[u8],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
) -> Result<js_sys::Uint8Array, JsValue> {
    let (data, _mask) = decode::<u8>(
        blob,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint8Array::from(&data[..]))
}

/// Decode a LERC blob to Uint16Array.
#[wasm_bindgen(js_name = decodeUint16)]
pub fn decode_u16_wasm(
    blob: &[u8],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
) -> Result<js_sys::Uint16Array, JsValue> {
    let (data, _mask) = decode::<u16>(
        blob,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint16Array::from(&data[..]))
}

/// Decode a LERC blob to Int32Array.
#[wasm_bindgen(js_name = decodeInt32)]
pub fn decode_i32_wasm(
    blob: &[u8],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
) -> Result<js_sys::Int32Array, JsValue> {
    let (data, _mask) = decode::<i32>(
        blob,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Int32Array::from(&data[..]))
}

/// Decode a LERC blob to Uint32Array.
#[wasm_bindgen(js_name = decodeUint32)]
pub fn decode_u32_wasm(
    blob: &[u8],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
) -> Result<js_sys::Uint32Array, JsValue> {
    let (data, _mask) = decode::<u32>(
        blob,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint32Array::from(&data[..]))
}

/// Auto-decode a LERC blob based on embedded type info, returns Float64Array.
/// Uses the data type from the blob metadata.
#[wasm_bindgen(js_name = decodeAuto)]
pub fn decode_auto_wasm(blob: &[u8]) -> Result<js_sys::Float64Array, JsValue> {
    let info = get_blob_info(blob).map_err(lerc_error_to_js)?;

    // Decode as the native type and convert to f64 for JS compatibility
    let data: Vec<f64> = match info.data_type {
        0 => {
            // i8
            let (d, _) = decode::<i8>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d.into_iter().map(|v| v as f64).collect()
        }
        1 => {
            // u8
            let (d, _) = decode::<u8>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d.into_iter().map(|v| v as f64).collect()
        }
        2 => {
            // i16
            let (d, _) = decode::<i16>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d.into_iter().map(|v| v as f64).collect()
        }
        3 => {
            // u16
            let (d, _) = decode::<u16>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d.into_iter().map(|v| v as f64).collect()
        }
        4 => {
            // i32
            let (d, _) = decode::<i32>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d.into_iter().map(|v| v as f64).collect()
        }
        5 => {
            // u32
            let (d, _) = decode::<u32>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d.into_iter().map(|v| v as f64).collect()
        }
        6 => {
            // f32
            let (d, _) = decode::<f32>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d.into_iter().map(|v| v as f64).collect()
        }
        7 => {
            // f64
            let (d, _) = decode::<f64>(
                blob,
                info.width as usize,
                info.height as usize,
                info.depth as usize,
                info.bands as usize,
                info.masks as usize,
            )
            .map_err(lerc_error_to_js)?;
            d
        }
        _ => return Err(JsValue::from_str("Unknown data type")),
    };

    Ok(js_sys::Float64Array::from(&data[..]))
}

/// Encode Float32 data to LERC format.
///
/// @param data - Float32Array with raw pixel data
/// @param width - Image width in pixels
/// @param height - Image height in pixels
/// @param depth - Values per pixel (default 1)
/// @param bands - Number of bands (default 1)
/// @param masks - Number of validity masks (default 1)
/// @param max_z_error - Maximum error for lossy compression (0 = lossless)
/// @returns Uint8Array with compressed LERC data
#[wasm_bindgen(js_name = encodeFloat32)]
pub fn encode_f32_wasm(
    data: &[f32],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
    max_z_error: f64,
) -> Result<js_sys::Uint8Array, JsValue> {
    let compressed = encode::<f32>(
        data,
        None,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
        max_z_error,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint8Array::from(&compressed[..]))
}

/// Encode Float64 data to LERC format.
#[wasm_bindgen(js_name = encodeFloat64)]
pub fn encode_f64_wasm(
    data: &[f64],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
    max_z_error: f64,
) -> Result<js_sys::Uint8Array, JsValue> {
    let compressed = encode::<f64>(
        data,
        None,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
        max_z_error,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint8Array::from(&compressed[..]))
}

/// Encode Uint8 data to LERC format.
#[wasm_bindgen(js_name = encodeUint8)]
pub fn encode_u8_wasm(
    data: &[u8],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
    max_z_error: f64,
) -> Result<js_sys::Uint8Array, JsValue> {
    let compressed = encode::<u8>(
        data,
        None,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
        max_z_error,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint8Array::from(&compressed[..]))
}

/// Encode Uint16 data to LERC format.
#[wasm_bindgen(js_name = encodeUint16)]
pub fn encode_u16_wasm(
    data: &[u16],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
    max_z_error: f64,
) -> Result<js_sys::Uint8Array, JsValue> {
    let compressed = encode::<u16>(
        data,
        None,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
        max_z_error,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint8Array::from(&compressed[..]))
}

/// Encode Int32 data to LERC format.
#[wasm_bindgen(js_name = encodeInt32)]
pub fn encode_i32_wasm(
    data: &[i32],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
    max_z_error: f64,
) -> Result<js_sys::Uint8Array, JsValue> {
    let compressed = encode::<i32>(
        data,
        None,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
        max_z_error,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint8Array::from(&compressed[..]))
}

/// Encode Uint32 data to LERC format.
#[wasm_bindgen(js_name = encodeUint32)]
pub fn encode_u32_wasm(
    data: &[u32],
    width: u32,
    height: u32,
    depth: u32,
    bands: u32,
    masks: u32,
    max_z_error: f64,
) -> Result<js_sys::Uint8Array, JsValue> {
    let compressed = encode::<u32>(
        data,
        None,
        width as usize,
        height as usize,
        depth as usize,
        bands as usize,
        masks as usize,
        max_z_error,
    )
    .map_err(lerc_error_to_js)?;

    Ok(js_sys::Uint8Array::from(&compressed[..]))
}
