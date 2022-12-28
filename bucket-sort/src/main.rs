fn bucket_sort(arr: &mut [i32]) {
    let max = arr.iter().max().unwrap();
    let mut buckets: Vec<Vec<i32>> = (0..=*max).map(|_| Vec::new()).collect();

    for x in arr.iter() {
        buckets[*x as usize].push(*x);
    }

    let mut i = 0;
    for bucket in buckets.iter() {
        for x in bucket.iter() {
            arr[i] = *x;
            i += 1;
        }
    }
}

fn main() {
    let mut arr = [2, 67, 34, 54, 87, 98, 67, 76, 5, 18];
    bucket_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}