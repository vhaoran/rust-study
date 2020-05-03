#[test]
fn qsort_test() {
    println!("------------before----------");
    let mut a = vec![3, 18, 1, 15, 27, 36];
    println!("{:?}", a);
    let h = a.len() - 1;
    q_sort(&mut a, 0, h);
    println!("----------------------");
    println!("{:?}", a);
}

#[allow(dead_code)]
fn q_sort(arr: &mut Vec<u8>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mut l = left;
    let mut r = right;
    while l < r {
        while l < r && arr[r] >= arr[left] {
            r -= 1;
        }
        while l < r && arr[l] <= arr[left] {
            l += 1;
        }
        arr.swap(l, r);
    }
    arr.swap(left, l);
    if l > 1 {
        q_sort(arr, left, l - 1);
    }

    q_sort(arr, r + 1, right);
}
