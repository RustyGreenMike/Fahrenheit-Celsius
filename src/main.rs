// Converting temps between fahrenheit and celcius with input
use std::io;

fn main() {
	
	// Gets user to say which they want to convert: c or f32
	println!("Type c for Celcius to Fahrenheit or type f for Fahrenheit to Celcius, and then press Enter.");
	let mut f_or_c = String::new();
	
	io::stdin()
		.read_line(&mut f_or_c)
		.expect("Failed to read line");
	
	// Gets user to specify the number of degrees they are looking to convert
	println!("Now, enter in the number of degrees with a decimal place e.g. 32.4 or 100.0");
	let mut deg_text = String::new();
	
	io::stdin()
		.read_line(&mut deg_text)
		.expect("Failed to read line");
		
	let deg_f: f32 = deg_text.trim().parse().unwrap();
	let deg_c: f32 = deg_text.trim().parse().unwrap();

	let f_or_c: &str = f_or_c.trim();
	
	if f_or_c == "f".to_string() {
		number_of_degrees_f(deg_f);
	} else {
		number_of_degrees_c(deg_c);
	}
}

// Function that returns that answer for f
fn number_of_degrees_f(num: f32) -> f32 {
	println!("{} degrees Fahrenheit is {} degrees Celcius!", num, f_to_c(num));
	return f_to_c(num)
}

// Function that returns the answer for c
fn number_of_degrees_c(num: f32) -> f32 {
	println!("{} degrees Celcius is {} degrees Fahrenheit!", num, c_to_f(num));
	return c_to_f(num)
}
	
// Function for Fahrenheit Calc
fn f_to_c(f: f32) -> f32 {
	(f - 32.0) / 1.8
}

// Function for Celcius Calc
fn c_to_f(c: f32) -> f32{
	(c * 1.8) + 32.0
}




	

	
