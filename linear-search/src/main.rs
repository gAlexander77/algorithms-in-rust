fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let arr = [2, 67, 34, 54, 87, 98, 67, 76, 5, 18];
    let target = 67;

    let index = linear_search(&arr, target);
    println!("The index of {} in the array is: {:?}", target, index);
}