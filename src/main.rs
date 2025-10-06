mod tar;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), anyhow::Error> {
    let file = File::open("test-data/multi_file_plot.tar")?;
    let mut buf_reader = BufReader::new(file);
    let mut buf = [0; 512];
    buf_reader.read(&mut buf)?;

    println!("Read {} bytes", buf.len());
    println!("First 100 bytes: {:?}", &buf);
    let s = String::from_utf8_lossy(&buf);
    println!("As string: {}", s);
    let header: tar::PosixHeader = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
    println!("Header: {}", header);



    Ok(())
}
