
// #[test]
fn qsort_test() {
    println!("----------------------");
    let a = 2;
    let b = a;
    let c = a;
    println!("----------------------");
    println!("b {},c {}", b, c);
}

#[allow(dead_code)]
fn q_sort(l: &mut Vec<i64>,left:usize,right:usize){
    let tag = l[left];
    //
    let mut low = left;
    let mut high = right;

    //
    while low < high{
        if l[high] < tag{
            swap(l,high,low)
        }
        high.add(-1);

        left.add(1);



    }
}

fn swap(l: &mut Vec<i64>, a:usize, b:usize){
    let t = l[a];
    l[a]  = l[b];
    l[b] = t;
}
