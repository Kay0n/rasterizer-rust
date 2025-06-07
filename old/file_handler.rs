

// use std::fs::File;
// use std::io::{Write, Result};
// use crate::image::FrameBuffer;

// pub fn write_bitmap(img: FrameBuffer, path: &str) -> Result<()> {


//     let row_padding = (4 - (img.width * 3) % 4) % 4; // pad each row to multiple of 4 bytes
//     let image_size = (img.width * 3 + row_padding) * img.height;
//     let file_size = 14 + 40 + image_size;

//     let mut file = File::create(path)?;

//     // === BMP Header (14 bytes) ===
//     file.write_all(b"BM")?;                                 // signature
//     file.write_all(&(file_size as u32).to_le_bytes())?;     // file size
//     file.write_all(&[0, 0, 0, 0])?;                         // reserved?
//     file.write_all(&54u32.to_le_bytes())?;                  // pixel data offset (14 + 40)

//     // === DIB Header (40 bytes) ===
//     file.write_all(&40u32.to_le_bytes())?;                  // DIB header size
//     file.write_all(&(img.width as i32).to_le_bytes())?;     // width
//     file.write_all(&(img.height as i32).to_le_bytes())?;    // height 
//     file.write_all(&1u16.to_le_bytes())?;                   // planes
//     file.write_all(&24u16.to_le_bytes())?;                  // bits per pixel (24 = RGB)
//     file.write_all(&0u32.to_le_bytes())?;                   // no compression
//     file.write_all(&(image_size as u32).to_le_bytes())?;    // image size
//     file.write_all(&[0; 16])?;                              // resolution and color info (unused)


//     // === Pixel Data ===
//     let row_width = (img.width * 3) as usize;
//     let pad = vec![0u8; row_padding as usize];

//     for y in (0..img.height).rev() {
//         let start = (y as usize) * row_width;
//         let end = start + row_width;
//         let row = &img.pixels[start..end];
//         file.write_all(row)?;        
//         file.write_all(&pad)?;       
//     }

//     return Ok(());
// }



