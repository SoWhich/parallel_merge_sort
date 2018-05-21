extern crate scoped_threadpool;

pub mod csorts {
    use std::vec::Vec;
    use scoped_threadpool::Pool;
    use std::slice;

    fn next_biggest_2pow(size: usize) -> usize {
        let mut ret = 1;
        while ret < size {
            ret *= 2;
        }
        ret
    }

    pub fn mergesort<T: Send + Sync + Ord + Clone>(arr: &mut Vec<T>) {

        let mut block_size= 1;

        // ceiling integer divison
        let largest_block_size = next_biggest_2pow(arr.len());
        let arr_len = arr.len();

        while block_size < largest_block_size {
            block_size *= 2;


            let num_blocks = (arr.len() - 1)/block_size + 1;
            let block_list: Vec<usize> = (0..num_blocks).collect();

            let mut pool = Pool::new(num_blocks as u32);

            pool.scoped(|scope| {

                for block in block_list {

                    let first_ind = block*block_size;
                    let last_ind = (block + 1) * block_size;
                    let mut working_len;

                    let working_slice_ptr = if last_ind >= arr_len + 1 {
                        working_len = arr[first_ind..].len();
                        arr[first_ind..].as_mut_ptr()
                    } else {
                        working_len = block_size;
                        arr[first_ind..last_ind].as_mut_ptr()
                    };

                    let mut working_slice;

                    unsafe {
                        working_slice = slice::from_raw_parts_mut(working_slice_ptr, working_len);
                    }

                    if working_slice.len() == 1 {
                        return;
                    } else {
                        scope.execute(move || {
                                merge_halves(working_slice, block_size);
                        });
                    }
                }
            });
        }
    }

    fn merge_halves<T: Ord + Clone>(half_sorted: &mut [T], block_size: usize) {
        let first = Vec::from(&half_sorted[..(block_size/2)]);
        let last = Vec::from(&half_sorted[(block_size/2)..]);

        let mut first_cur = 0;
        let mut last_cur = 0;

        for elem in half_sorted.iter_mut() {
            if first_cur == first.len() {
                *elem = last[last_cur].clone();
                last_cur += 1;
            } else if last_cur == last.len() {
                *elem = first[first_cur].clone();
                first_cur += 1;
            } else if first[first_cur] <= last[last_cur] {
                *elem = first[first_cur].clone();
                first_cur += 1;
            } else {
                *elem = last[last_cur].clone();
                last_cur += 1;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use csorts;
    #[test]
    fn it_works() {
        {
            let mut vec1 = vec![4, 3, 2, 1];
            csorts::mergesort(&mut vec1);
            assert_eq!(vec1, vec![1, 2, 3, 4]);
        } {
            let mut vec1 = vec![8, 7, 6, 5, 4, 3, 2, 1];
            csorts::mergesort(&mut vec1);
            assert_eq!(vec1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        } {
            let mut vec1 = vec![1, 2, 5, 6, 4, 3, 4, 7];
            csorts::mergesort(&mut vec1);
            assert_eq!(vec1, vec![1, 2, 3, 4, 4, 5, 6, 7]);
        } {
            let mut vec1 = vec![9, 1, 2, 5, 6, 4, 3, 4, 7];
            csorts::mergesort(&mut vec1);
            assert_eq!(vec1, vec![1, 2, 3, 4, 4, 5, 6, 7, 9]);
        }
    }
}
