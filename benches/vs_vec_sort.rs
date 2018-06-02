#![feature(test)]

extern crate parallel_merge_sort;
extern crate rand;
extern crate test;

#[cfg(test)]
mod tests {

    use parallel_merge_sort::merge_sort;
    use rand;
    use rand::prelude::*;
    use std::vec::Vec;
    use test::Bencher;
    const CAP: usize = 100000;

    fn sort<T>(thing: &mut Vec<T>)
    where
        T: Ord + Send + Clone,
    {
        if cfg!(feature = "builtin") {
            thing.sort();
        } else {
            merge_sort(thing);
        }
    }

    fn fuzzy_vec_generate<T>() -> Vec<T>
    where
        T: Ord + Send,
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        let mut vector = Vec::<T>::with_capacity(CAP);
        for _ in 0..CAP {
            vector.push(random());
        }

        vector
    }

    fn rev_vec_generate<T>() -> Vec<T>
    where
        T: Ord + Send + From<usize>,
    {
        let mut vector = Vec::<T>::with_capacity(CAP);
        for elem in 0..CAP {
            vector.push(T::from(elem));
        }

        vector
    }

    fn ordered_vec_generate<T>() -> Vec<T>
    where
        T: Ord + Send + From<usize>,
    {
        let mut vector = Vec::<T>::with_capacity(CAP);
        for elem in CAP..0 {
            vector.push(T::from(elem));
        }

        vector
    }

    #[bench]
    fn bench_rand(b: &mut Bencher) {
        b.iter(|| {
            let mut vec = fuzzy_vec_generate::<usize>();
            sort(&mut vec);
        });
    }

    #[bench]
    fn bench_ordered(b: &mut Bencher) {
        b.iter(|| {
            let mut vec = ordered_vec_generate::<usize>();
            sort(&mut vec);
        });
    }

    #[bench]
    fn bench_rev(b: &mut Bencher) {
        b.iter(|| {
            let mut vec = rev_vec_generate::<usize>();
            sort(&mut vec);
        });
    }
}
