use std::io::*;

fn main() {
    let bytes: Vec<u8> = (0..10).collect();
    let mut file = Cursor::new(&bytes);
    let file = file.by_ref();
    let mut buf = Vec::new();
    let mut nread = 0;
    loop {
        let read = file.take(4).read_to_end(&mut buf).unwrap();
        println!("read {}", read);
        if read == 0 {
            break;
        }
        nread += read;
    }
    println!("{} {:?}", nread, buf);
}