use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Write, Read};


fn compress_file(input: &str, output: &str) -> std::io::Result<()> {
    let mut input_file = File::open(input)?;
    let output_file = File::create(output)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let mut buffer = Vec::new();
    
    // Read input file
    input_file.read_to_end(&mut buffer)?;
    
    // Write
    encoder.write_all(&buffer)?;
    
    encoder.finish()?;
    
    Ok(())
}

fn decompress_file(input: &str, output: &str) -> std::io::Result<()> {
    let input_file = File::open(input)?;
    let mut decoder = GzDecoder::new(input_file);
    let mut output_file = File::create(output)?;
    let mut buffer = Vec::new();  

    // Read the compressed data
    decoder.read_to_end(&mut buffer)?;
    
    // Write the decompressed data
    output_file.write_all(&buffer)?;
    
    Ok(())
}

fn main() {
    compress_file("inp.pdf", "compressedfile.gz").expect("Compression failed");
    decompress_file("compressedfile.gz", "decompressed.pdf").expect("Decompression failed");
}
