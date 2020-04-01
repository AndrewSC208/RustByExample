// remove all values from list a that are in list b
fn array_diff<T: PartialEq> (a: Vec<T>, b: Vec<T>) -> Vec<T> {
	// __into_iter:__ Convert into an iterator. By implementing `IntoIterator` for a type, you define how it will be converted to an iterator.
	// __filter:__ An iterator that filters the elements of `iter` with `predicate`.
	// __collect:__ Transforms an iterator into a collection
	// reference: https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item 
	a.into_iter().filter(|x| !b.contains(x)).collect()
}

// a really small array diff method for Vec's<u8>
// adding in the Vec<u8> so that I can make my life a bit more easy
// while I learn this stuff
fn array_diff_2(a: Vec<u8>, b: Vec<u8>) {
	// iterate over all of items in b
	// remove each item from a that is the value of b
	for n in b {
		println!("{}", n);
		//for i in 0..a.len() - 1 {
		//	println!("of a {}", i);
		//	if a[i] == n {
		//		// remove item from vector in position i
		//		// @note -> I think that I need to actually create a new vec, and return only the vec
		//		// 	    with the values that I want it to contain.
		//		//	    ref(https://stackoverflow.com/questions/57749069/thread-main-panicked-at-index-out-of-bounds-the-len-is-2-but-the-index-is-2)
		//		println!("pos: {}", i);	
		//		a.swap_remove(i);
		//	}
		//}
	}	
}

fn main() {
	let a = vec![1, 2, 2, 4, 5, 5, 5, 6, 7, 8, 9];
	// remove all occurances of items in b from a
	let b = vec![2];

	// this is a my failed attempt, but I learned alot of about vectors.
	// array_diff_2(a, b);

	let diff = array_diff(a, b);
	println!("{:?}", diff)
}
