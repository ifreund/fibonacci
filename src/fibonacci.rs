use std::collections::HashMap;

pub struct Fibonacci {
    fib_cache: HashMap<u64, u64>,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {
            fib_cache: HashMap::new(),
        }
    }

    pub fn calc(&mut self, n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                if let Some(i) = self.fib_cache.get(&n) {
                    *i
                } else {
                    let res = self.calc(n - 1) + self.calc(n - 2);
                    self.fib_cache.insert(n, res);
                    res
                }
            }
        }
    }

    pub fn calc_no_cache(&self, n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => self.calc_no_cache(n - 1) + self.calc_no_cache(n - 2),
        }
    }
}
