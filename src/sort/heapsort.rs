pub fn heapsort<F>(arr: &mut [i32], visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i, visualize);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        visualize(arr, &[0, i]);
        heapify(&mut arr[..i], i, 0, visualize);
    }
}

fn heapify<F>(arr: &mut [i32], n: usize, i: usize, visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(i, largest);
        visualize(arr, &[i, largest]);
        heapify(arr, n, largest, visualize);
    }
}
