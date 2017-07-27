fn main() {
    println!("Running mergesort...");
    let array = &mut [5, 1, 7, 9, 3];
    let end = array.len() - 1;
    let sorted_array = mergesort(array, 0, end);
}

fn mergesort(array: &mut [i32], start: usize, end: usize) {
    if start < end {
        let mid = (start + end) / 2;
        mergesort(array, 0, mid);
        mergesort(array, mid + 1, end);
        merge(array, start, end);
    }
}

fn merge(array: &mut [i32], start: usize, end: usize) {

    let mut temp = vec![0; array.len()];
    temp.copy_from_slice(array);

    let mut left_finger = start;
    let mut current = start;
    let left_end = (start + end) / 2;
    let mut right_finger = left_end + 1;

    while (left_finger <= left_end) && (right_finger <= end) {
        if temp[left_finger] < temp[right_finger] {
            array[current] = temp[left_finger];
            left_finger += 1;
        } else {
            array[current] = temp[right_finger];
            right_finger += 1;
        }
        current += 1;
    }

    while left_finger <= left_end {
        array[current] = temp[left_finger];
        left_finger += 1;
        current += 1;
    }
}


#[test]
fn test_mergesort() {
    let array = &mut [-1, 12, 68, 0, -23, 32, 2, 9];
    let sorted_array = &[-23, -1, 0, 2, 9, 12, 32, 68];

    let end = array.len() - 1;
    mergesort(array, 0, end);
    assert_eq!(array, sorted_array);
}
