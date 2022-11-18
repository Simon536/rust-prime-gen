use clap::Parser;

fn primesieve(max: usize) -> Vec<bool> {
    println!("Searching for primes less than {max}");

    let g_factor = (max as f32).sqrt();

    println!("Greatest factor is {g_factor}");

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
}

fn main() {
    let args = Args::parse();

    if args.search_end > 1_000_000_000_000 {
        panic!(
            "{} is too large. Max allowed value is 1,000,000,000,000.",
            args.search_end
        );
    }

    let out = primesieve(args.search_end);

    if args.list {
        for (n, val) in out.iter().enumerate() {
            if *val {
                println!("{n}");
            }
        }
    }

    // Print a count of the number of primes found
    println!(
        "\n{} primes smaller than {}",
        out.iter().filter(|x| **x).count(),
        args.search_end
    );
}
