extern crate parallel_merge_sort;
extern crate rand;

#[cfg(test)]
mod tests {

    use parallel_merge_sort::merge_sort;
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
