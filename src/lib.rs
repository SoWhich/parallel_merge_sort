//! # Parallel Merge Sort
//!
//! `parallel_merge_sort` is an implementation of Merge Sort for a mutable
//! slice that sorts blocks of equal size in parallel threads.
extern crate rand;
extern crate rayon;

use rayon::prelude::*;
use std::ptr;
use std::vec::Vec;

/// # Merge Sort
/// The mergesort function contains the actual implementation of the parallel
/// merge sort.
///
/// This merge sort generates a thread pool using the `num_cpus` crate.
///
/// The threads are scope locked so that each thread must complete before the
/// block size is doubled again.
///
/// This particular function demands a mutable slice, which is used as both
/// an input and output parameter. If you would like the function to generate
/// a new (sorted) vector, please use the function `gen_and_sort<T>(&[T])
/// -> Vec<T>`.
///
/// The `merge_sort` function can be sorted without allocating extra memory if
/// compiled with `--cfg inplace`.
pub fn merge_sort<T>(arr: &mut [T])
where
    T: Ord + Send,
{
    let mut block_size = 2;

    let largest_block_size = 2 * arr.len();

    let mut buff: Vec<T> = Vec::with_capacity((arr.len() + 1) / 2);
    unsafe {
        buff.set_len((arr.len() + 1) / 2);
    }
    while block_size < largest_block_size {
        // the scope of the pooled threads is locked within this lambda, so
        // the program blocks until their completion at its end.
        if cfg!(feature = "noparallel") {
            for (block, buff) in arr
                .chunks_mut(block_size)
                .zip(buff.chunks_mut(block_size / 2))
            {
                if block.len() > block_size / 2 && !is_sorted(block, block_size) {
                    merge_halves(block, buff, block_size);
                }
            }
        } else {
            arr.par_chunks_mut(block_size)
                .zip(buff.par_chunks_mut(block_size / 2))
                .for_each(|(block, buff)| {
                    if block.len() > block_size / 2 && !is_sorted(block, block_size) {
                        merge_halves(block, buff, block_size);
                    }
                });
        }

        block_size *= 2;
    }
    unsafe {
        buff.set_len(0);
    }
}

#[inline]
fn is_sorted<T>(block: &[T], block_size: usize) -> bool
where
    T: Ord,
{
    block.get(block_size / 2 - 1) <= block.get(block_size / 2)
}

/// # Generate and Sort
/// This function generates a new (now sorted) vector when given an immutable
/// slice reference.
///
/// Because this function is merely light function overhead for the prior
/// `merge_sort<T>(arr: &mut Vec<T>)`, the use of that function should be
/// encouraged over this.
pub fn gen_and_sort<T>(arr: &[T]) -> Vec<T>
where
    T: Ord + Send + Clone,
{
    let mut ret = Vec::from(arr);
    merge_sort(&mut ret);
    ret
}

fn merge_halves<T>(half_sorted: &mut [T], first_block: &mut [T], block_size: usize)
where
    T: Ord,
{
    let mut first_block_size = block_size / 2;
    let mut first = first_block.as_mut_ptr();
    let mut cur = half_sorted.as_mut_ptr();
    let mut second: *mut T = &mut half_sorted[first_block_size];

    unsafe {
        ptr::copy_nonoverlapping(cur, first, first_block_size);

        // end points to the first instance of invalid memory beyond the end of
        // the slice and is NEVER dereferenced
        let end = (&mut half_sorted[half_sorted.len() - 1] as *mut T).add(1);

        while cur != end {
            if *first <= *second {
                cur.write(first.read());
                cur = cur.add(1);
                first = first.add(1);
                first_block_size -= 1;

                if first_block_size == 0 {
                    return;
                }
            } else {
                cur.write(second.read());
                cur = cur.add(1);
                second = second.add(1);

                if second == end {
                    ptr::copy_nonoverlapping(first, cur, first_block_size);
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand;
    use rand::prelude::*;
    use std::cmp::Ordering;
    use std::fmt::Debug;
    use std::vec::Vec;

    #[derive(Debug, Clone)]
    struct Dropper {
        num: i32,
        is_dropped: Box<bool>,
    }

    impl Dropper {
        fn new(num: i32) -> Dropper {
            Dropper {
                num: num,
                is_dropped: Box::from(false),
            }
        }
    }

    impl Drop for Dropper {
        fn drop(&mut self) {
            if !*self.is_dropped {
                *self.is_dropped = true;
            } else {
                panic!();
            }
        }
    }

    impl Ord for Dropper {
        fn cmp(&self, other: &Dropper) -> Ordering {
            self.num.cmp(&other.num)
        }
    }

    impl PartialOrd for Dropper {
        fn partial_cmp(&self, other: &Dropper) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Dropper {
        fn eq(&self, other: &Dropper) -> bool {
            self.num == other.num
        }
    }

    impl Eq for Dropper {}

    #[test]
    fn in_order() {
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

    #[test]
    fn edge() {
        vec_test(vec![1]);
        vec_test(Vec::<u32>::new());
        vec_test(vec![
            String::from("z"),
            String::from("y"),
            String::from("x"),
            String::from("w"),
        ]);
        vec_test(vec![
            Dropper::new(0),
            Dropper::new(1),
            Dropper::new(2),
            Dropper::new(3),
            Dropper::new(4),
        ]);
    }

    fn fuzzer<T>()
    where
        T: Ord + Send + Clone + Debug,
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        let fuzz_cap = 100;
        let mut vector = Vec::<T>::with_capacity(fuzz_cap);
        for _ in 0..fuzz_cap {
            vector.push(random());
        }

        vec_test(vector);
    }

    fn vec_test<T>(mut subject: Vec<T>)
    where
        T: Ord + Send + Clone + Debug,
    {
        let mut mine = subject.clone();

        merge_sort(&mut mine);
        subject.sort();

        assert_eq!(&mut mine, &mut subject);
    }
}
