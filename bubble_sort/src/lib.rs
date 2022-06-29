pub fn bubble_sort(list: & mut Vec<i64>) {
    for i in 0..list.capacity(){
        let mut swapped = false;
        for j in 1..(list.capacity()-i) {
            if list[j-1] > list[j] {
                let temp = list[j-1];
                list[j-1] = list[j];
                list[j] = temp; 
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut list = vec![10,9,8,7,6,5,4];
        bubble_sort(& mut list);
        assert_eq!(list, vec![4, 5, 6, 7, 8, 9, 10]);
    }

    // #[bench]
    // fn bubble_sort_bench(b: &mut Bencher) {
    //     let mut list: Vec<i64> = Vec::with_capacity(1_0000);
    //     for _ in 0..list.capacity() {
    //         list.push(rand::random());
    //     };
    //     bubble_sort(& mut list);
    //     b.iter(|| bubble_sort(& mut list));
    // }
}
