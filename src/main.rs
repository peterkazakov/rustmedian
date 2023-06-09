#![feature(test)]

use std::collections::BinaryHeap;
use rand::prelude::*;

//Rossetta code implementation
fn median(mut xs: Vec<i32>) -> f64 {
    xs.sort_by(|x,y| x.partial_cmp(y).unwrap() );
    let n = xs.len();
    if n % 2 == 0 {
        (xs[n/2] + xs[n/2 - 1]) as f64 / 2.0
    } else {
        xs[n/2] as f64
    }
}

// BinaryHeap is based on priority queue implementation; to get min, we use negative ordering for higher_queue
// This will cause problem for value edge case -2^31, but since we are limited in scope, we do not cover this
fn median_priority(xs: Vec<i32>) -> f64 {
    let mut lower_queue = BinaryHeap::new();
    let mut higher_queue = BinaryHeap::new();
    for elem in xs.iter() {
        match *elem {
            v if lower_queue.is_empty() && higher_queue.is_empty() => { lower_queue.push(v); }
            v if v < *lower_queue.peek().unwrap() => { lower_queue.push(v); }
            v if higher_queue.is_empty() => { higher_queue.push(-v); }
            v if v > *higher_queue.peek().unwrap() => { higher_queue.push(-v); }
            v if lower_queue.len() > higher_queue.len() => { higher_queue.push(-v); }
            v => { lower_queue.push(v); }
        }
        //balance
        if lower_queue.len() > higher_queue.len()+1 {
            let item = lower_queue.pop().unwrap();
            higher_queue.push(-item);
        } else if higher_queue.len() > lower_queue.len() +1 {
            let item = higher_queue.pop().unwrap();
            lower_queue.push(-item);
        };
    }

    if xs.len() % 2 == 0 { (lower_queue.pop().unwrap() - higher_queue.pop().unwrap()) as f64 / 2.0 }
    else if lower_queue.len() > higher_queue.len() { lower_queue.pop().unwrap() as f64 }
    else { -higher_queue.pop().unwrap() as f64 }
}

fn main() {
    let nums = vec![4,5,7,1111,-23,4];
    println!("{:?}", median(nums.clone()));
    println!("{:?}", median_priority(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(median(vec![1]), 1.0);
        assert_eq!(median_priority(vec![1]), 1.0);
    }
    #[test]
    fn test_double() {
        assert_eq!(median(vec![1, 8]), 4.5);
        assert_eq!(median_priority(vec![1, 8]), 4.5);
    }
    #[test]
    fn test_many() {
        assert_eq!(median(vec![1, 8, -1, -8, 444, -444, 16, -16]), 0.0);
        assert_eq!(median_priority(vec![1, 8, -1, -8, 444, -444, 16, -16]), 0.0);
    }
    #[test]
    fn test_odd() {
        assert_eq!(median(vec![4,5,7,1111,-23,4]), 4.5);
        assert_eq!(median_priority(vec![4,5,7,1111,-23,4]), 4.5);
    }

    #[test]
    fn test_shuffle_20() {
        let mut rng = rand::thread_rng();

        let mut nums: Vec<i32> = (1..21).collect();
        nums.shuffle(&mut rng);

        assert_eq!(median(nums.clone()), 10.5);
        assert_eq!(median_priority(nums.clone()), 10.5);
    }

    #[test]
    fn test_random_100() {
        let arr = [(); 90].map(|_| thread_rng().gen_range(0..100)).to_vec();
        assert_eq!(median(arr.clone()), median_priority(arr.clone()));
    }


    extern crate test;

    use test::Bencher;

    fn random_vector() -> Vec<i32> {
        [(); 20000].map(|_| thread_rng().gen_range(0..100)).to_vec()
    }

    #[bench]
    fn bench_standard(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers: Vec<i32> = random_vector();
            median(numbers);
        });
    }

    #[bench]
    fn bench_queues(b: &mut Bencher) {
        b.iter(|| {
            let mut numbers: Vec<i32> = random_vector();
            median_priority(numbers);
        });
    }
}