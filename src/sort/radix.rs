pub fn radixsort<F>(arr: &mut [i32], visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    let max = match arr.iter().max() {
        Some(&m) => m,
        None => return,
    };
    let mut exp = 1;
    while exp <= max {
        counting_sort(arr, exp, visualize);
        exp *= 10;
    }
}

fn counting_sort<F>(arr: &mut [i32], exp: i32, visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    for &num in arr {
        let digit = ((num / exp) % 10) as usize;
        count[digit] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        let digit = ((num / exp) % 10) as usize;
        output[count[digit] - 1] = num;
        count[digit] -= 1;
    }

    for i in 0..n {
        arr[i] = output[i];
    }
    visualize(arr, &(0..arr.len()).collect::<Vec<_>>());
}
