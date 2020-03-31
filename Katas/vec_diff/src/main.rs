// remove all values from list a that are in list b
//fn array_diff<T: PartialEq> (a: Vec<T>, b: Vec<T>) {
	// loop over b
//	for n in b {
//		println!("{:#?}", n)
//	}
		// loop over a and if item in a matches b then remove	
//}

// a really small array diff method for Vec's<u8>
// adding in the Vec<u8> so that I can make my life a bit more easy
// while I learn this stuff
fn array_diff_2(mut a: Vec<u8>, b: Vec<u8>) {
	// iterate over all of items in b
	// remove each item from a that is the value of b
	for n in b {
		println!("{}", n);
		for i in 0..a.len() - 1 {
			println!("of a {}", i);
			if a[i] == n {
				// remove item from vector in position i
				// @note -> I think that I need to actually create a new vec, and return only the vec
				// 	    with the values that I want it to contain.
				//	    ref(https://stackoverflow.com/questions/57749069/thread-main-panicked-at-index-out-of-bounds-the-len-is-2-but-the-index-is-2)
				println!("pos: {}", i);	
				a.swap_remove(i);
			}
		}		
	}	
}

fn main() {
	let mut a = vec![1, 2, 2, 4, 5, 5, 5, 6, 7, 8, 9];
	// remove all occurances of items in b from a
	let mut b = vec![2];

	array_diff_2(a, b);
}
