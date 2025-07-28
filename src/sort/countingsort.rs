fn counting_sort(arr: &mut [usize], max_value: usize) {
    let mut count = vec![0; max_value + 1];

    // Count occurrences of each value
    for &num in arr.iter() {
        count[num] += 1;
    }

    // Reconstruct the sorted array
    let mut index = 0;
    for (value, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[index] = value;
            index += 1;
        }
    }
}

