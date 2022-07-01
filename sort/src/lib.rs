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

pub fn merge_sort(list: & mut Vec<i64>) {
    list.sort();
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

    #[test]
    fn merge_sort_test() {
        let mut list = vec![10,9,8,7,6,5,4];
        merge_sort(& mut list);
        assert_eq!(list, vec![4, 5, 6, 7, 8, 9, 10]);
    }
}
