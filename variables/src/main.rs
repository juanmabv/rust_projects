fn main() {
    let mut x = 5;
	println!("The value of x is {x}");
	x = 6;
	println!("The value of x is {x}");
	
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
	println!("Three hours in seconds are {THREE_HOURS_IN_SECONDS} seconds");
	
	let y = 7;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
	
	let spaces = "     ";
	println!("spaces text: ({spaces})");
    let spaces = spaces.len();
	println!("spaces numeric: ({spaces})");
	
	// example of comment
}
