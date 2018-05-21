extern crate scoped_threadpool;
extern crate rand;

pub mod csorts {
    use std::vec::Vec;
    use scoped_threadpool::Pool;
    use std::slice;

    pub fn mergesort<T>(arr: &mut Vec<T>)
        where T: Ord + Send + Clone
    {

        let mut block_size= 2;

        let arr_len = arr.len();

        let largest_block_size = 2*arr_len;

        while block_size < largest_block_size {

            let num_blocks = (arr.len() - 1)/block_size + 1;

            let mut pool = Pool::new(num_blocks as u32);

            pool.scoped(|scope| {

                for block in 0..num_blocks {

                    let first_ind = block*block_size;
                    let last_ind = (block + 1) * block_size;
                    let mut slice_len;

                    let slice_ptr = if last_ind >= arr_len + 1 {
                                slice_len = arr[first_ind..].len();
                                arr[first_ind..].as_mut_ptr()
                            } else {
                                slice_len = block_size;
                                arr[first_ind..last_ind].as_mut_ptr()
                            };

                    if slice_len <= block_size/2 {
                        return;

                    } else {

                        let mut arr_slice;
                        unsafe {
                            arr_slice = slice::from_raw_parts_mut(slice_ptr, slice_len);
                        }

                        scope.execute(move || {
                                merge_halves(arr_slice, block_size);
                        });
                    }
                }
            });

            block_size *= 2;
        }
    }

    fn merge_halves<T>(half_sorted: &mut [T], block_size: usize)
        where T: Ord + Clone
    {

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
    use rand;
    use std::vec::Vec;
    use std::fmt::Debug;
    use rand::prelude::*;
    #[test]
    fn tests() {
        fuzzer::<char>();
        fuzzer::<i64>();
        fuzzer::<u64>();
    }

    fn fuzzer<T>()
        where T: Ord + Send + Clone + Debug,
              rand::distributions::Standard: rand::distributions::Distribution<T>
        {

        let mut vector = Vec::<T>::with_capacity(100);
        for _ in 0..100 {
            vector.push(random());
        }

        vec_test(&mut vector);
    }

    fn vec_test<T>(subject: &mut Vec<T>) 
        where T: Ord + Send + Clone + Debug
    {
        let mut mine = subject.clone();

        csorts::mergesort(&mut mine);
        subject.sort();

        assert_eq!(&mut mine, subject);
    }
}
