use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use serde::{Deserialize, Serialize};

pub enum SplitOptions {
    NumberOfParts(usize),
    SizeOfParts(usize)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SplitInfo {
    filename: String,
    number_of_parts: usize,
    hash: u32,
    part_hashes: Vec<u32>
}

pub fn split_file(f: &str, options: SplitOptions) {
    let mut file = File::open(f).unwrap();
    let mut content = vec![];
    file.read_to_end(&mut content).unwrap();
    let fsize = content.len();

    match options {
        SplitOptions::NumberOfParts(n) => {
            let nsize = fsize / n;
            println!("Splitting file in {} parts with size {}B", n, nsize);
            make_split_files(&mut file, content, n, nsize, f);
        }
        SplitOptions::SizeOfParts(s) => {
            let n = fsize / s;
            println!("Splitting file in {} parts with size {}B", n, s);
            make_split_files(&mut file, content, n, s, f);
        }
    }
}

fn make_split_files(file: &mut File, content: Vec<u8>, n: usize, s: usize, f: &str) {
    let mut info = SplitInfo{
        filename: String::from(std::path::Path::new(&f).file_name().unwrap().to_str().unwrap()),
        hash: crc32fast::hash(&content),
        number_of_parts: n,
        part_hashes: vec![]
    };

    for i in 0..n {
        println!("Writing Part {}", i);
        let mut part = File::create(format!("{}.{}.part", &f, i)).unwrap();
        let mut read_buf: Vec<u8> = vec![0u8; s as usize];
        file.seek(std::io::SeekFrom::Start((s*i) as u64)).unwrap();
        if i == n-1 {
            read_buf = vec![];
            file.read_to_end(&mut read_buf).unwrap();
        } else {
            file.read_exact(&mut read_buf).unwrap();
        }

        // PART + HASH
        let phash = crc32fast::hash(&read_buf);
        info.part_hashes.push(phash);

        // WRITE FILE
        part.write_all(&mut read_buf).unwrap();
    }

    let mut info_f = File::create(format!("{}.partinfo", &f)).unwrap();
    let mut info_s = serde_json::to_vec(&info).unwrap();
    info_f.write_all(&mut info_s).unwrap();
}

pub fn combine_file(f: &str) {
    let mut info_c = vec![];
    File::open(&f).unwrap().read_to_end(&mut info_c).unwrap();

    let info: SplitInfo = serde_json::from_slice(&info_c).unwrap();
    let basename = std::path::Path::new(&f).file_stem().unwrap().to_str().unwrap();

    let mut final_content: Vec<u8> = vec![];

    for i in 0..info.number_of_parts {
        let mut read = vec![];
        let mut part_f = File::open(format!("{}.{}.part", basename, i)).unwrap();
        part_f.read_to_end(&mut read).unwrap();

        // CHECKSUM TEST
        if crc32fast::hash(&read) != *info.part_hashes.get(i).unwrap() {
            println!("Checksum error on part {}", i);
            std::process::exit(1);
        }

        final_content.append(&mut read);
    }
    if crc32fast::hash(&final_content) != info.hash {
        println!("Checksum error on completed file");
        std::process::exit(1);
    }
    println!("File successfile combined");
    let mut ff = File::create(basename).unwrap();
    ff.write_all(&mut final_content).unwrap();
}