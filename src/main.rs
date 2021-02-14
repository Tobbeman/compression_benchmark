use smaz::{compress};
use lz4_flex::{compress_prepend_size};

fn main() {

    println!("smaz:");
    let original_length = *&compression_test::INPUT.as_bytes().len() as f64;
    println!("original length: {:?}", original_length);
    let compressed = compress(&compression_test::INPUT.as_bytes());
    let compressed_length = *&compressed.len() as f64;
    println!("compressed length: {:?}", compressed_length);
    println!("compression ratio: {:?}", compressed_length/original_length);

    println!("---");

    println!("lz4_flex:");
    let original_length = *&compression_test::INPUT.as_bytes().len() as f64;
    println!("original length: {:?}", original_length);
    let compressed = compress_prepend_size(&compression_test::INPUT.as_bytes());
    let compressed_length = *&compressed.len() as f64;
    println!("compressed length: {:?}", compressed_length);
    println!("compression ratio: {:?}", compressed_length/original_length)
}