fn radix_sort(arr: &mut [i32]) {
    let max = *arr.iter().max().unwrap();

    let mut exp = 1;
    while max / exp > 0 {
        count_sort(arr, exp);
        exp *= 10;
    }
}

fn count_sort(arr: &mut [i32], exp: i32) {
    let n = arr.len();
    let mut output = [0; 10];
    let mut count = [0; 10];

    for i in 0..n {
        count[((arr[i] / exp) % 10) as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for i in (0..n).rev() {
        output[count[((arr[i] / exp) % 10) as usize] - 1] = arr[i];
        count[((arr[i] / exp) % 10) as usize] -= 1;
    }

    for i in 0..n {
        arr[i] = output[i];
    }
}

fn main() {
    let mut arr = [2, 67, 34, 54, 87, 98, 67, 76, 5, 18];
    radix_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}