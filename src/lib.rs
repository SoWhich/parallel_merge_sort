//! # Parallel Merge Sort
//!
//! `parallel_merge_sort` is an implementation of Merge Sort for a mutable
//! slice that sorts blocks of equal size in parallel threads.
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
pub fn merge_sort<T>(arr: &mut [T])
where
    T: Ord + Send,
{
    let buff_size = arr
        .len()
        .wrapping_sub(1)
        .checked_next_power_of_two()
        .unwrap_or(0);

    let mut buff: Vec<T> = Vec::with_capacity(buff_size);

    unsafe {
        buff.set_len(buff_size);
    }

    for (buff_block_size, arr_block_size) in (0..)
        .map(|num| 1 << num)
        .take_while(|block_size| *block_size <= buff_size)
        .zip((1..).map(|num| 1 << num))
    {
        // the scope of the pooled threads is locked within this lambda, so
        // the program blocks until their completion at its end.
        let merge = |(block, buff): (&mut [T], &mut [T])| unsafe {
            if block.len() <= buff_block_size || is_sorted(block, arr_block_size) {
                return;
            } else {
                merge_halves(block, buff);
            }
        };

        if cfg!(feature = "noparallel") {
            arr.chunks_mut(arr_block_size)
                .zip(buff.chunks_mut(buff_block_size))
                .for_each(merge);
        } else {
            arr.par_chunks_mut(arr_block_size)
                .zip(buff.par_chunks_mut(buff_block_size))
                .for_each(merge);
        }
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

unsafe fn merge_halves<T>(half_sorted: &mut [T], first_block: &mut [T])
where
    T: Ord,
{
    let mut first_block_size = first_block.len();
    let mut first = first_block.as_mut_ptr();
    let mut cur = half_sorted.as_mut_ptr();
    let mut second: *mut T = &mut half_sorted[first_block_size];

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
