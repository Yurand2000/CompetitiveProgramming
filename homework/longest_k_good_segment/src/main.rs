use std::collections::HashMap;

fn find_kgood_segment(vec: &[i32], k: usize) -> (usize, usize) {
    let mut occurrencies = HashMap::<i32, usize>::new();

    let mut curr_min = 0;
    let mut curr_max = 0;
    let mut best_size = 0;
    let mut curr_best = (0, 0);

    for &val in vec.iter()
    {
        //increase occurrencies counters
        if let Some(val_occs) = occurrencies.get_mut(&val) {
            *val_occs += 1
        } else {
            occurrencies.insert(val, 1);
        }

        //if too many different values, shorten the segment from the left
        while occurrencies.len() > k {
            let curr_min_val = vec[curr_min];
            if let Some(val_occs) = occurrencies.get_mut(&curr_min_val) {
                *val_occs -= 1;
                if *val_occs == 0 {
                    occurrencies.remove(&curr_min_val);
                }
            }
            curr_min += 1;
        }

        //check if the new segment is bigger then the previous max
        let curr_size = curr_max - curr_min + 1;
        if curr_size > best_size {
            best_size = curr_size;
            curr_best = (curr_min, curr_max);
        }
        
        //lenghten the segment to the right
        curr_max += 1;
    }

    curr_best
}

fn main() {
    //Example 1
    let a = [1, 2, 3, 4, 5];
    let k = 5;

    let kgood_segment = find_kgood_segment(&a, k);
    let expected_segment = (0, 4);
    assert_eq!(kgood_segment, expected_segment);

    //Example 2
    let a = [6, 5, 1, 2, 3, 2, 1, 4, 5];
    let k = 3;

    let kgood_segment = find_kgood_segment(&a, k);
    let expected_segment = (2, 6);
    assert_eq!(kgood_segment, expected_segment);

    //Example 3
    let a = [1, 2, 3];
    let k = 1;

    let kgood_segment = find_kgood_segment(&a, k);
    let expected_segment = (0, 0);
    assert_eq!(kgood_segment, expected_segment);

    println!("Success!");
}
