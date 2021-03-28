extern crate encoding;
use std::io::*;
use std::fs::File;
// use std::io::prelude::*;
use encoding::all::GB18030;
use encoding::{Encoding, DecoderTrap};

// 读文件到bytes
fn read_file(filename:&String) -> std::io::Result<Vec<u8>>{
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = vec![];
    buf_reader.read_to_end(&mut contents)?;
    Ok(contents)
}
fn write_file(filename:&String,contents:String) -> std::io::Result<()>{
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
fn main() {
    let args = std::env::args();
    let len = args.len();
    if len < 2 {
        println!("请输入文件名！");
    }
    let input_file = args.last().expect("文件名有误！");
    println!("filename={}", input_file);
    let contents = read_file(&input_file).expect("读文件错误!");
    // bytes -> string
    let result = GB18030.decode(&contents[..],DecoderTrap::Strict).expect("gb18030解码失败!");
    println!("result={:?}", result);
    let output_file = input_file+".utf8";
    write_file(&output_file,result);
    println!("newfilename={:?}", output_file);
    // println!("GBK={:?}", GBK);
    // for arg in args.iter() {
    //     println!("{:?}", arg);
    // }
    // println!("{:?}", args.iter());
}