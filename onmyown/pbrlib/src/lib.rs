use std::ops::Range;

/// Return true iff two ranges overlap.
///
///     assert_eq!(ranges::overlap(0..7, 3..10, true);
///
/// If either range is empty, they don't count
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end &&
        r1.start < r2.end && r2.start < r1.end
}

///
pub fn fact(n: u64) -> u64 {
    match n {
		0 => 1,
		1 => 1,
		_ => n * fact(n - 1)
	}
}

///
pub fn fibo(n: u64) -> u64 {

	/*
	let rv = if n == 0 {
        	0
	} else if n == 1 {
		1
	} else { fib(n - 1) + fib(n - 2) };
	*/

	match n {
		0 => 0,
		1 => 1,
		_ => fibo(n - 1) + fibo(n - 2)
	}

	// this breaks the whole stuff... let b = 21;
}


/// This doc, instead, hides a line of code sample
///
///     # use code::not::tested::Here;
///     apply_sunlight(4);
///
pub fn apply_sunlight(time: u32) {
    println!("applying sunlight {}", time)
}


pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        // Track if any elements were swapped in this pass
        let mut swapped = false;

        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                // Swap elements
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // If no elements were swapped, the array is already sorted
        if !swapped {
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fibo() {
        assert_eq!(fibo(5), 5);
    }

    #[test]
    fn test_fact() {
        assert_eq!(fact(5), 120);
    }

    #[test]
    fn test_bubble_sort() {
        let mut numbers = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut numbers);
        assert_eq!(numbers, [11, 12, 22, 25, 34, 64, 90]);
        // println!("After sorting: {:?}", numbers);
    }
}
