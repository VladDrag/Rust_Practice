// use std::any::Any;

fn print_type_of<T>(_: &T, vartype: String) -> bool {
	let var = format!("{}", std::any::type_name::<T>());
	vartype.eq(&var)
}

// let apple: String = String::from("apple");
// let banana: String = String::from("banana");
// println!("{}", apple.eq(&banana));


fn main() {
    // let s = "Hello";
    let i:u8 = 42; //standardul este i32

    // println!("{}", print_type_of(&s)); // &str
    // println!("{}", print_type_of(&i)); // i32
    // println!("{}", print_type_of(&main)); // playground::main
    // println!("{}", print_type_of(&print_type_of::<i32>)); // playground::print_type_of<i32>
    // println!("{}", print_type_of(&{ || "Hi!" })); // playground::main::{{closure}}
	// println!("{}", assert_eq!(s, &str));
	let ys: [i32; 500] = [0; 500];
	println!("{:#?}", ys);
	if print_type_of(&i, String::from("u8")) == true { println!("kekmao?");}
	else {
		println!("Lololoo!");
	}
	// println!("{}", print_type_of(&s, String::from("&str")));
}