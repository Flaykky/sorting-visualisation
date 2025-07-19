pub fn list_visualization( &[i32], indices: &[usize]) {
    print!("[");
    for (i, &val) in data.iter().enumerate() {
        print!("{}", val);
        if i != data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    let mut underline = vec![' '; data.len() * 3];
    for &idx in indices {
        if idx < data.len() {
            let pos = idx * 3;
            if pos < underline.len() {
                underline[pos + 1] = '-';
            }
        }
    }
    println!("{}", underline.iter().collect::<String>());
}
