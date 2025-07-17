pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut arr_len = array.len();
    let mut mid = arr_len / 2;

    while array[mid] != key {
        if mid == arr_len - 1 || mid == 0 {
            return None;
        }

        if array[mid] > key {
            arr_len = mid;
            mid /= 2;
        } else {
            mid = (mid + arr_len) / 2
        }
    }

    Some(mid)
}
