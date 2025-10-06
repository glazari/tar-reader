mod tar;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), anyhow::Error> {
    let file = File::open("test-data/multi_file_plot.tar")?;
    let mut buf_reader = BufReader::new(file);

    loop {
        let header = read_header(&mut buf_reader)?;
        if header.is_none() {
            break; // EOF
        }
        let header = header.unwrap();
        println!("Header: {}", header.name());

        let file_content = read_file(&header, &mut buf_reader)?;
        let content = String::from_utf8_lossy(&file_content);
        println!("First 200 chars: {}", &content[..200.min(content.len())]);
    }

    Ok(())
}

fn read_header(reader: &mut BufReader<File>) -> Result<Option<tar::PosixHeader>, anyhow::Error> {
    let mut buf = [0; 512];
    let read = reader.read(&mut buf)?;
    if read == 0 {
        // EOF
        return Ok(None);
    }
    if read < 512 {
        println!("Read less than 512 bytes for header: {}", read);
        return Err(anyhow::anyhow!("Incomplete header read"));
    }

    if buf.iter().all(|&b| b == 0) {
        return Ok(None); // End of archive
    }

    let header: tar::PosixHeader = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
    Ok(Some(header))
}

fn read_file(
    header: &tar::PosixHeader,
    reader: &mut BufReader<File>,
) -> Result<Vec<u8>, anyhow::Error> {
    let size_in_blocks = (header.size() + 511) / 512;

    let mut file_content = Vec::with_capacity(header.size() as usize);
    for _ in 0..size_in_blocks {
        let mut content_buf = [0; 512];
        reader.read_exact(&mut content_buf)?;
        if file_content.len() + content_buf.len() > header.size() as usize {
            let remaining = header.size() as usize - file_content.len();
            file_content.extend_from_slice(&content_buf[..remaining]);
            break;
        }

        file_content.extend_from_slice(&content_buf);
    }
    Ok(file_content)
}
