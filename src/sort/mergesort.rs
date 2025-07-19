pub fn mergesort<F>(arr: &mut [i32], visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        mergesort(&mut arr[..mid], visualize);
        mergesort(&mut arr[mid..], visualize);
        merge(arr, mid, visualize);
    }
}

fn merge<F>(arr: &mut [i32], mid: usize, visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    let mut merged = Vec::with_capacity(arr.len());
    let (left, right) = arr.split_at(mid);
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    for (k, &val) in merged.iter().enumerate() {
        arr[k] = val;
    }

    visualize(arr, &(0..arr.len()).collect::<Vec<_>>());
}
