fn shell_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            let temp = arr[i];

            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }

            arr[j] = temp;
        }

        gap /= 2;
    }
}

fn main() {
    let mut arr = [2, 67, 34, 54, 87, 98, 67, 76, 5, 18];
    shell_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}