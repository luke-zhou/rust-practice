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

pub fn quick_sort_old(list: & mut Vec<i64>) {
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
        quick_sort_old(& mut less);
        quick_sort_old(& mut great);
        list.append(& mut less);
        list.push(pivot);
        list.append(& mut great);
    }
}

pub fn quick_sort(list: & mut Vec<i64>, start: usize, end: usize) {
    println!("input:{:?}", list);
    println!("start:{}", start);
    println!("end:{}", end);
    let mut pivot = (end - start + 1)/2;
    let mut left = start;
    let mut right = end;

    while left < right {
        while left < pivot {
            if list[left] > list[pivot] {
                let temp = list[left];
                list[left] = list[pivot];
                list[pivot] = temp;
                pivot = left;
                break;
            }
            else{
                left+=1;
            }
        }
        while right > pivot {
            if list[right] < list[pivot] {
                let temp = list[right];
                list[right] = list[pivot];
                list[pivot] = temp;
                pivot = right;
                break;
            }
            else{
                right-=1;
            }
        }
    }
    println!("pivot:{}", list[pivot]);
    println!("{:?}", list);
    if pivot >0 && pivot-1 > start {
        println!("call less");
        quick_sort(list, start, pivot-1);
    }
    if end > pivot+1 {
        println!("call great");
        quick_sort(list, pivot+1, end);
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
    fn quick_luke_sort_test() {
        let mut list = vec![10,5,8,11,4,7,5,9,3,6];
        let length = list.len();
        quick_sort(& mut list, 0, length-1);
        assert_eq!(list, vec![3, 4, 5, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn quick_sort_test() {
        let mut list = vec![10, 9, 11, 8];
        let length = list.len();
        quick_sort(& mut list, 0, length-1);
        assert_eq!(list, vec![3, 4, 5, 5, 6, 7, 8, 9, 10, 11]);
    }
}
