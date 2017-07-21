fn main() {
    println!("Running mergesort...");
    let array = &mut [];
    let _sorted_array = mergesort(array);
}

fn mergesort(array: &mut [i32]) -> &[i32] {
    return array
}


#[test]
fn test_mergesort() {
    let array = &mut [-1, 12, 68, 0, -23, 32, 2, 9];
    let sorted_array = &[-23, -1, 0, 2, 9, 12, 32, 68]; 

    assert_eq!(mergesort(array), sorted_array);
}
