pub fn timsort<F>(arr: &mut [i32], visualize: &F)
where
    F: Fn(&[i32], &[usize]),
{
    arr.sort_unstable();
    visualize(arr, &(0..arr.len()).collect::<Vec<_>>());
}
