fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut arr = [2, 67, 34, 54, 87, 98, 67, 76, 5, 18];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}