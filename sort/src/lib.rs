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

pub fn quick_sort(list: & mut Vec<i64>) {
    let pivot = list.pop();
    if let Some(pivot) = pivot {
        // println!("pivot:{}", pivot);
        let mut less: Vec<i64> = Vec::with_capacity(list.capacity());
        let mut great: Vec<i64> = Vec::with_capacity(list.capacity());
        let mut test_num = list.pop();
        while let Some(num) = test_num {
            if num > pivot {
                // println!("great:{}", num);
                great.push(num);
            }
            else {
                // println!("less:{}", num);
                less.push(num);
            }
            test_num = list.pop();
        }
        quick_sort(& mut less);
        quick_sort(& mut great);
        list.append(& mut less);
        list.push(pivot);
        list.append(& mut great);
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

    #[test]
    fn merge_sort_test() {
        let mut list = vec![10,9,8,7,6,5,4];
        merge_sort(& mut list);
        assert_eq!(list, vec![4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn quick_sort_test() {
        let mut list = vec![10,5,8,11,7,4,5,9,3,6];
        quick_sort(& mut list);
        assert_eq!(list, vec![3, 4, 5, 5, 6, 7, 8, 9, 10, 11]);
    }
}
