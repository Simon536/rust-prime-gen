use clap::Parser;
use std::thread::spawn;

fn primesieve(max: usize) -> Vec<bool> {
    let g_factor = (max as f32).sqrt();

    // Create boolean vector
    // Creating the full size vector is more efficient than creating a small one and incrementally adding to it
    let mut mask = vec![true; max];

    // Manually mark 0 and 1 as non-prime
    mask[0] = false;
    mask[1] = false;

    for n in 2..=(g_factor as usize) {
        for m in ((n * 2)..max).step_by(n) {
            mask[m] = false;
        }
    }

    mask
}

fn primesieve_extended(min: usize, max: usize) -> usize {
    let g_factor = (max as f32).sqrt() as usize;

    let sieve = primesieve(g_factor + 1);

    let mut mask = vec![true; max - min];

    for (n, _) in sieve.iter().enumerate().take(g_factor + 1).skip(2) {
        if sieve[n] {
            let mut first = min + n - 1;
            first -= first % n;

            for m in (first..max).step_by(n) {
                if m == n {
                    continue;
                }
                mask[m - min] = false;
            }
        }
    }

    mask.iter().filter(|x| **x).count()
}

fn list_primes(max: usize) {
    let out = primesieve(max);

    for (n, val) in out.iter().enumerate() {
        if *val {
            println!("{n}");
        }
    }

    // Print a count of the number of primes found
    println!(
        "\n{} primes smaller than {}",
        out.iter().filter(|x| **x).count(),
        max
    );
}

fn count_primes(max: usize) -> usize {
    let count = if max < 1_000_000 {
        primesieve(max).iter().filter(|x| **x).count()
    } else {
        let t1 = spawn(move || primesieve_extended(2, max / 3));
        let t2 = spawn(move || primesieve_extended(max / 3, (2 * max) / 3));
        let t3 = spawn(move || primesieve_extended((2 * max) / 3, max));

        t1.join().unwrap() + t2.join().unwrap() + t3.join().unwrap()
    };

    count
}

/// Simple program to calculate prime numbers using a sieve
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number to stop search at
    #[arg(short, long, default_value_t = 1000)]
    search_end: usize,

    /// List all primes found
    #[arg(short, long)]
    list: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_count() {
        // There are 78,498 primes smaller than 1,000,000
        assert_eq!(78498, primesieve(1000000).iter().filter(|x| **x).count());
    }

    #[test]
    fn test_prime_list() {
        assert_eq!(
            vec![
                false, false, true, true, false, true, false, true, false, false, false, true,
                false, true, false
            ],
            primesieve(15)
        );
    }

    #[test]
    fn sieve_range_count() {
        assert_eq!(primesieve_extended(10, 50), 11);
    }
}

fn main() {
    let args = Args::parse();

    if args.search_end > 1_000_000_000_000 {
        panic!(
            "{} is too large. Max allowed value is 1,000,000,000,000.",
            args.search_end
        );
    }

    if args.list && args.search_end > 1_000_000 {
        panic!(
            "{} is too large. Cannot list primes greater than 1,000,000. Choose a smaller number or remove the list flag.",
            args.search_end
        );
    }

    if args.list {
        list_primes(args.search_end);
    } else {
        let c = count_primes(args.search_end);

        print!("There are {} primes smaller than {}", c, args.search_end);
    }
}
