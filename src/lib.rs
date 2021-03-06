use std::cmp::Ordering;

fn double_pivot_quicksort<T:Ord> (arr: &mut [T], left: usize, right: usize) {

	let len = right - left;
	if len < 27 {
		insertion_sort(arr, left, right);
		return;
	}

	unsafe {
		// pivots
		let pivot1 : *mut T = &mut arr[left];
		let pivot2 : *mut T = &mut arr[right];
		
		// swap pivots if p1 > p2
		if (&*pivot1).cmp(&*pivot2) == Ordering::Greater {
			arr.swap(left, right);
		}

		// partition indexes
		let mut less = left + 1;
		let mut greater = right - 1;

		// sorting
		let mut k = less;
		while k <= greater {
			match arr[k].cmp(&*pivot1) {
				Ordering::Less | Ordering::Equal => {
					arr.swap(k, less);
					less = less + 1;
				},
				_ => {
					if arr[k].cmp(&*pivot2) == Ordering::Greater || arr[k].cmp(&*pivot2) == Ordering::Equal {
						while k < greater && arr[greater].cmp(&*pivot2) == Ordering::Greater {
							greater = greater - 1;
						}
						arr.swap(k, greater);
						greater = greater - 1;

						if arr[k].cmp(&*pivot1) == Ordering::Less || arr[k].cmp(&*pivot1) == Ordering::Equal {
							arr.swap(k, less);
							less = less + 1;
						}
					}
				}
			}
			k = k + 1;
		}
		arr.swap(less - 1, left);
		arr.swap(greater + 1, right);

		
		if less > left + 2 {
			double_pivot_quicksort(arr, left, less - 2);
		}

		if greater + 2 < right {
			double_pivot_quicksort(arr, greater + 2, right);
		}

		if less < greater && (&*pivot1).cmp(&*pivot2) == Ordering::Less {
			double_pivot_quicksort(arr, less, greater);
		}
	}
}

/// An insertion sort for small slices
#[inline]
fn insertion_sort<T>(arr: &mut [T], left: usize, right: usize) where T: Ord {
	for i in (left + 1)..(right + 1) {
		let mut j = i;
		while j > left && arr[j].cmp(&arr[j - 1]) == Ordering::Less {
			arr.swap(j, j - 1);
			j = j - 1;
		}
	}
}

/// An in-place quicksort for ordered items.
#[inline]
pub fn quicksort<T>(arr: &mut [T]) where T: Ord {
    let len = arr.len();
    if len > 1 {
    	double_pivot_quicksort(arr, 0, len - 1);
    }
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
pub mod tests {
    use rand::{self, Rng};
    use super::quicksort;

    #[test]
    pub fn random() {
        let mut rng = rand::thread_rng();
        for _ in 0u32 .. 50000u32 {
            let len: usize = rng.gen();
            let mut v: Vec<isize> = rng.gen_iter::<isize>().take((len % 64) + 1).collect();
            quicksort(&mut v);
            for i in 0 .. v.len() - 1 {
                assert!(v[i] <= v[i + 1])
            }
        }
    }

	#[test]
	pub fn bench_quicksort1() {
		let mut rng = rand::thread_rng();
		let len: usize = 20000000;
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();
	    quicksort(&mut v);
	}

	#[test]
	pub fn bench_quicksort2() {
		let len: usize = 7000;
	    let mut v: Vec<usize> = (0..len).collect();
	    quicksort(&mut v);
	}

	#[test]
	pub fn bench_sort() {
		let mut rng = rand::thread_rng();
		let len: usize = 20000000;
	    let mut v: Vec<isize> = rng.gen_iter::<isize>().take(len).collect();
	    v.sort();
	}


	#[test]
	pub fn bench_sort2() {
		let len: usize = 7000;
	    let mut v: Vec<usize> = (0..len).collect();
	    vq.sort();
	}

}