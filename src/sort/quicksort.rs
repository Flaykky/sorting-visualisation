pub fn quicksort<F>(arr: &mut [i32], visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    if arr.len() <= 1 {
        return;
    }
    let pivot_idx = partition(arr, visualize);
    let _ = visualize(arr, &[pivot_idx]);
    quicksort(&mut arr[..pivot_idx], visualize);
    quicksort(&mut arr[pivot_idx + 1..], visualize);
}

fn partition<F>(arr: &mut [i32], visualize: &F) -> usize
where
    F: Fn(&[i32], &[usize]),
{
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            let _ = visualize(arr, &[i, j]);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    let _ = visualize(arr, &[i, len - 1]);
    i
}
