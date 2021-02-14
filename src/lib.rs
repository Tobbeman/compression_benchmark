#![feature(test)]

extern crate test;
use smaz::{compress};
use lz4_flex::{compress_prepend_size};

pub const INPUT: &str = "Put request on \"/boot-source\" with body \"{\\n            \\\"kernel_image_path\\\": \\\"/home/elavtob/tmp/hello-vmlinux.bin\\\",\\n            \\\"boot_args\\\": \\\"console=ttyS0 reboot=k panic=1 pci=off\\\"\\n       }\"";


pub fn no_compress(input: String, list: &mut Vec<String>) {
    list.push(input);
}

pub fn smaz_compress(input: String, list: &mut Vec<Vec<u8>>) {
    let compressed = compress(&input.as_bytes());
    list.push(compressed);
}

pub fn lz4_flex_compress(input: String, list: &mut Vec<Vec<u8>>) {
    let compressed = compress_prepend_size(&input.as_bytes());
    list.push(compressed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    const NBR_RUNS: usize = 1;

    #[bench]
    fn bench_no_compress(b: &mut Bencher) {
        let mut list: Vec<String> = Vec::new();
        b.iter(|| {
            for _ in 0..NBR_RUNS {
                no_compress(INPUT.to_string(), &mut list)
            }
        });
    }

    #[bench]
    fn bench_smaz_compress(b: &mut Bencher) {
        let mut list: Vec<Vec<u8>> = Vec::new();
        b.iter(|| {
            for _ in 0..NBR_RUNS {
                smaz_compress(INPUT.to_string(), &mut list)
            }
        });
    }

    #[bench]
    fn bench_lz4_flex_compress(b: &mut Bencher) {
        let mut list: Vec<Vec<u8>> = Vec::new();
        b.iter(|| {
            for _ in 0..NBR_RUNS {
                lz4_flex_compress(INPUT.to_string(), &mut list)
            }
        });
    }
}


