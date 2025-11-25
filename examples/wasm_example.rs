//! Example of LERC encoding/decoding for WASM compilation.
//!
//! This example demonstrates the use of the lerc library for WASM.
//! Compile with:
//! ```bash
//! cargo build --example wasm_example --target wasm32-unknown-emscripten --release
//! ```

use lerc::{decode, encode, get_blob_info};

fn main() {
    println!("LERC WASM Example");

    // Create sample data: 64x64 gradient image
    let width = 64;
    let height = 64;
    let mut data: Vec<f32> = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            data.push((x as f32 + y as f32) / (width + height) as f32);
        }
    }

    println!("Original data size: {} floats", data.len());
    println!("Original bytes: {} bytes", data.len() * 4);

    // Encode with lossless compression
    // Note: n_masks should be 0 when no validity mask is provided
    let compressed = encode(&data, None, width, height, 1, 1, 0, 0.0).expect("Encoding failed");
    println!("Compressed size: {} bytes", compressed.len());
    println!(
        "Compression ratio: {:.2}x",
        (data.len() * 4) as f64 / compressed.len() as f64
    );

    // Get blob info
    let info = get_blob_info(&compressed).expect("Failed to get blob info");
    println!("Blob info:");
    println!("  Version: {}", info.version);
    println!("  Data type: {}", info.data_type);
    println!("  Dimensions: {}x{}", info.width, info.height);
    println!("  Depth: {}", info.depth);
    println!("  Bands: {}", info.bands);

    // Decode
    let (decoded, _mask): (Vec<f32>, _) = decode(
        &compressed,
        info.width as usize,
        info.height as usize,
        info.depth as usize,
        info.bands as usize,
        info.masks as usize,
    )
    .expect("Decoding failed");

    // Verify
    let mut max_diff: f32 = 0.0;
    for (orig, dec) in data.iter().zip(decoded.iter()) {
        max_diff = max_diff.max((orig - dec).abs());
    }
    println!("Max difference after round-trip: {}", max_diff);

    // Test lossy compression
    let compressed_lossy =
        encode(&data, None, width, height, 1, 1, 0, 0.01).expect("Encoding failed");
    println!("\nLossy compression (max_z_error=0.01):");
    println!("  Compressed size: {} bytes", compressed_lossy.len());
    println!(
        "  Compression ratio: {:.2}x",
        (data.len() * 4) as f64 / compressed_lossy.len() as f64
    );

    println!("\nLERC WASM test completed successfully!");
}
