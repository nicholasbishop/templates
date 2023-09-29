fn dump_bytes(note: &str, data: &[u8]) {
    let wrap = 8;
    println!("{note}");
    for (i, b) in data.iter().enumerate() {
        print!("{b:02x} ");
        if ((i + 1) % wrap) == 0 {
            println!();
        }
    }
}
