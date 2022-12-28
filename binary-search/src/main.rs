fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

fn main() {
    let arr = [2, 4, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 5;

    let index = binary_search(&arr, target);
    println!("The index of {} in the array is: {:?}", target, index);
}
