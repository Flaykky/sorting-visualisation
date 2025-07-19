pub fn graph_visualization( &[i32], indices: &[usize]) {
    if data.is_empty() {
        return;
    }

    let max_height = *data.iter().max().unwrap_or(&0);
    for row in (1..=max_height).rev() {
        for (i, &value) in data.iter().enumerate() {
            if value >= row {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }

    let mut underline = vec![' '; data.len() * 2];
    for &idx in indices {
        if idx < underline.len() / 2 {
            underline[idx * 2] = '-';
        }
    }
    println!("{}", underline.iter().collect::<String>());
}
