//! # Parallel Merge Sort
//!
//! `parallel_merge_sort` is an implementation of Merge Sort for an
//! `std::vec::Vec` that sorts blocks of equal size in parallel threads.

extern crate scoped_threadpool;
extern crate rand;

use std::vec::Vec;
use scoped_threadpool::Pool;
use std::slice;


/// # Merge Sort
/// The mergesort function contains the actual implementation of the parallel
/// merge sort.
///
/// This merge sort generates a thread pool for each block of a common
/// size--2 on the inital pass, doubling each time--and generates a thread that
/// merges each sorted half of the block it's given.
///
/// The pool is scope locked so that each thread must complete before the block
/// size is doubled again.
///
/// This particular function demands a mutable vector, which is used as both
/// an input and output parameter. If you would like the function to generate
/// a new (sorted) vector, please use the function `gen_and_sort<T>(&Vec<T>)
/// -> Vec<T>`.
pub fn merge_sort<T>(arr: &mut Vec<T>)
    where T: Ord + Send + Clone
{

    let mut block_size= 2;

    let largest_block_size = 2*arr.len();

    while block_size < largest_block_size {

        let num_blocks = (arr.len() - 1)/block_size + 1;

        let mut pool = Pool::new(num_blocks as u32);

        pool.scoped(|scope| {

            for block in 0..num_blocks {

                let first_ind = block*block_size;
                let last_ind = (block + 1) * block_size;
                let mut slice_len;

                let slice_ptr = if last_ind >= arr.len() + 1 {
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

/// # Generate and Sort
/// This function generates a new (now sorted) vector when given an immutable
/// Vector reference.
///
/// Because this function is merely light function overhead for the prior
/// `merge_sort<T>(arr: &mut Vec<T>)`, the use of that function should be
/// encouraged over this.
pub fn gen_and_sort<T>(arr: &Vec<T>) -> Vec<T>
    where T: Ord + Send + Clone
{
    let mut ret = arr.clone();
    merge_sort(&mut ret);
    ret
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

#[cfg(test)]
mod tests {
    use rand;
    use super::*;
    use std::vec::Vec;
    use std::fmt::Debug;
    use rand::prelude::*;

    #[test]
    fn in_order() {
        vec_test(vec![1]);
        vec_test(vec![1, 2]);
        vec_test(vec![1, 2, 3, 4, 5]);
        vec_test(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        vec_test(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        vec_test(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn reverse_order() {
        vec_test(vec![2, 1]);
        vec_test(vec![5, 4, 3, 2, 1]);
        vec_test(vec![8, 7, 6, 5, 4, 3, 2, 1]);
        vec_test(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        vec_test(vec![15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn fuzz() {
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

        vec_test(vector);
    }

    fn vec_test<T>(mut subject: Vec<T>) 
        where T: Ord + Send + Clone + Debug
    {
        let mut mine = subject.clone();

        merge_sort(&mut mine);
        subject.sort();

        assert_eq!(&mut mine, &mut subject);
    }
}
