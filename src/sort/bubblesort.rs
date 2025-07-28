fn bubblesort(arr: &mut [i32]) {
    let len = arr.len();
    let mut swapped;

    for i in 0..len {
        swapped = false;
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // If no two elements were swapped by inner loop, then break
        if !swapped {
            break;
        }
    }
}
