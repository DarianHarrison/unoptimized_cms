#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// note for now only supportng u64
use std::hash::{Hash, Hasher};
use fasthash::{MumHasher, Murmur3HasherExt, MetroHasher, XXHasher, T1haHasher, SeaHasher, SpookyHasher, FarmHasher, CityHasher}; // preferred non-cryptographic hash functions
use std::collections::HashMap;

fn hx<T: Hash>(hf: &usize, x: &T) -> u64 {

    match hf { // match according to number of hash functions
        0 => {
            let mut s: MumHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        1 => {
            let mut s: Murmur3HasherExt = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        2 => {
            let mut s: MetroHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        3 => {
            let mut s: XXHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        4 => {
            let mut s: T1haHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        5 => {
            let mut s: SeaHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        6 => {
            let mut s: SpookyHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        7 => {
            let mut s: FarmHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        8 => {
            let mut s: CityHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        },
        _ => { // should error handle this
            let mut s: MumHasher = Default::default();
            x.hash(&mut s);
            s.finish()
        }, 
    }
}

fn p_estimation(std_error: f64) -> f64 { // estimate number of hash funcitons
    let p: f64 = (1.0/std_error).ln();
    p
}

fn m_recommendation ( n: usize, oe: usize )-> f64 { 
    // n = number of elements expected to be indexed 10,000,000
    // oe = fixed overestimate allowed error
    let e: f64 = oe as f64 / n as f64;
    let m: f64 = 2.71828 / e;
    m
}

#[cfg(test)]
mod frequency_tests {

    use super::*; // bring all of the test module’s parent’s items into scope with use super::*

    #[test]
    fn test_p_estimation() {
        let std_error: f64 = 0.01; // desired error ( largest supported accuracy for now ~ 99.9% accuracy with 0.000125)
        let p: f64 = p_estimation(std_error).ceil(); // at least p hash functions to satisfy error
        assert_eq!(p, 5.0);
    }

    #[test]
    fn test_size_estimation () { 
        let p: f64 = 5.0;
        let m_recommendation: f64 = m_recommendation( 10000000, 10 );
        let integer_bit_size: f64 = 32.0; // u32 bit size
        let conversion = 0.000000125; // bits in a MB
        let mb_memory: f64 = (m_recommendation * p * integer_bit_size * conversion).ceil();
        assert_eq!(mb_memory, 55.0); // would need around 55 MB of memory to maintain a 10 Million counter array 
    }

    #[test]
    fn test_countminsketch() {
        let stream: Vec<_> = vec![4,4,4,4,2,3,5,4,6,4,3,3,4,2,3,3,3,2]; // dataset length
        let n: usize = stream.len();
        let m: usize = 4; // number of counters
        let p: usize = 3; // number of hash functions
        let mut cms: Vec<Vec<u64>> = vec![vec![0; m]; p]; // initialize data structure

        // cms
        for (i, x) in stream.iter().enumerate() { // consume the sequence
            for j in 0..p { // for each hash function
                let h_value = hx(&j, &x);
                let h: usize = h_value as usize % m ;
                cms[j][h]+=1;
            }
        }
        assert_eq!(cms[0], vec![6, 3, 1, 8]);
    }

    #[test]
    fn test_frequency() {
        let stream: Vec<_> = vec![4,4,4,4,2,3,5,4,6,4,3,3,4,2,3,3,3,2]; // dataset length
        let n: usize = stream.len();
        let m: usize = 4; // number of counters
        let p: usize = 3; // number of hash functions
        let q: _ = 3; // number to estimate/query frequency
        let mut cms: Vec<Vec<u64>> = vec![vec![0; m]; p]; // initialize data structure

        // cms
        for (i, x) in stream.iter().enumerate() { // consume the sequence
            for j in 0..p { // for each hash function
                let h_value = hx(&j, &x);
                let h: usize = h_value as usize % m ;
                cms[j][h]+=1;
            }
        }

        // find frequency of a given input
        let mut counts: Vec<usize> = vec![];
        for j in 0..p { // for each hash function
            let h_value = hx(&j, &q); // 
            let h: usize = h_value as usize % m; // hash value position
            let c: usize = cms[j][h] as usize; // count
            counts.push(c)
        }
        let f_hat: usize = *counts.iter().min().unwrap();        
        assert_eq!(f_hat, 6);
    }

    #[test]
    fn test_streaming_top_k_heavy_hitters() {
        let stream: Vec<_> = vec![4,4,4,4,2,3,5,4,6,4,3,3,4,2,3,3,3,2]; // dataset length
        let n: usize = stream.len();
        let m: usize = 4; // number of counters
        let p: usize = 3; // number of hash functions

        let mut cms: Vec<Vec<u64>> = vec![vec![0; m]; p]; // initialize data structure

        // maintaining a heap for e-Heavy hitters problem with e=1/2k requires O(log(1/e)) additional work per element
        let k: usize = 3; // number of heavy hitters while processing stream
        //let mut x_star: Vec<(usize,usize)> = vec![]; // heap to store up to k heavy hitters 
        let mut x_star = HashMap::new();
        
        for (i, x) in stream.iter().enumerate() { // consume the sequence

            for j in 0..p { // for each hash function
                let h_value = hx(&j, &x);
                let h: usize = h_value as usize % m ;
                cms[j][h]+=1;
            }

            // find frequency of a given input
            let q: _ = x; // number to estimate frequency
            let mut counts: Vec<usize> = vec![];

            for j in 0..p { // for each hash function
                let h_value = hx(&j, &q); // 
                let h: usize = h_value as usize % m; // hash value position
                let c: usize = cms[j][h] as usize; // count
                counts.push(c)
            }
            let f_hat: f64 = *counts.iter().min().unwrap() as f64; // frequency estimation
            let f_star: f64 = i as f64 / k as f64 ; // threshold
            
            if &f_hat >= &f_star {

                x_star.insert(*x, f_hat as usize);
                x_star.retain(|_, f_hat| *f_hat as f64 > *x as f64);
            }
        }
        assert_eq!(true, x_star.contains_key(&4));
    }


}