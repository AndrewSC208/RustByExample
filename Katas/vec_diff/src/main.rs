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
fn array_diff_2(a: Vec<u8>, b: Vec<u8>) {
	for n in b {
		println!("{}", n)
		for i in b {
			
		}		
	}	
}

fn main() {
	let mut a = vec![1, 2, 2, 4, 5, 5, 5, 6, 7, 8, 9];
	let mut b = vec![2];

	array_diff_2(a, b);
}
