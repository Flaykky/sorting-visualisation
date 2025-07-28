fn gnome_sort(arr: &mut [i32]) {
    let mut index = 0;

    while index < arr.len() {
        if index == 0 || arr[index] >= arr[index - 1] {
            index += 1;
        } else {
            arr.swap(index, index - 1);
            index -= 1;
        }
    }
}

