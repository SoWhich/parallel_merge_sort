pub mod csorts {
    //use std::thread;
    use std::vec::Vec;
    pub fn mergesort<T: Send + Ord + Clone>(arr: &mut Vec<T>) {

        let mut block_size= 2;

        // ceiling integer divison
        let largest_block_size = (arr.len() + 1) / 2;

        while block_size <= largest_block_size {
            //let mut thread_stack = Vec::new();

            let last_index = (arr.len() - 1)/block_size;

            for index in 0..last_index {

                let (first_ind, mut last_ind) = (index*block_size, (index + 1)*block_size);

                if last_ind >= arr.len() {
                    last_ind = arr.len();
                }

                let working_slice = &mut arr[first_ind..last_ind];

                let first = Vec::from(&working_slice[..(block_size/2)]);
                let last = Vec::from(&working_slice[(block_size/2)..]);

                let mut first_cur = 0;
                let mut last_cur = 0;
                let mut working_cur = 0;

                while working_cur < working_slice.len() {
                    if first_cur == first.len() {
                        while last_cur != last.len() {
                            working_slice[working_cur] = last[last_cur].clone();
                            working_cur += 1;
                            last_cur += 1;
                        }
                    } else if last_cur == last.len() {
                        while first_cur != first.len() {
                            working_slice[working_cur] = first[first_cur].clone();
                            working_cur += 1;
                            first_cur += 1;
                        }
                    } else if first[first_cur] <= last[last_cur] {
                        working_slice[working_cur] = first[first_cur].clone();
                        working_cur += 1;
                        first_cur += 1;
                    } else {
                        working_slice[working_cur] = last[last_cur].clone();
                        working_cur += 1;
                        last_cur += 1;
                    }
                }
            }
            //for thd in thread_stack {
                //thd.join();
            //}

            block_size *= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use csorts;
    #[test]
    fn it_works() {
        let mut vec1 = vec![9, 1, 2, 5, 6, 4, 3, 4, 7];
        csorts::mergesort(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 4, 4, 5, 6, 7, 9]);
    }
}
