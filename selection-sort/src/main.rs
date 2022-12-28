fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in i + 1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn main() {
    let mut arr = [2, 67, 34, 54, 87, 98, 67, 76, 5, 18];
    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
