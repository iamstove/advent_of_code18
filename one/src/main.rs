fn main() -> Result<(),std::io::Error>
{
	use std::io::{BufReader, BufRead};
	use std::fs::File;
	let path = "input.txt";
	let mut current: i32 = 0;
	let mut change: i32 = 5;

	//open and read file
	let input = File::open(path)?;
	let reader = BufReader::new(input);

	//loop over file
	for line in reader.lines() {
		//println!("{}", line?);
		let s = line.unwrap();
		change = s.parse::<i32>().unwrap();
		current = shift(current,change);
	}

/*	loop{
		let mut s = String::new();
		print!("Enter drift: ");
		let _=io::stdout().flush();
		//get user input
		io::stdin().read_line(&mut s).unwrap();
		
		let x = s.trim().parse::<i32>();

		match x {
			Ok(v) => {
				change = x.unwrap();
				current = shift(current, change)
			},
			Err(e) => {
				//println!("Input = {}, Error = {}", s, e);
				break
			}
		}
		let _=io::stdout().flush();
	}*/

    println!("current = {}", current);

    Ok(())
}

fn shift(current: i32, change: i32) -> i32
{
	let retval: i32;

	retval = current + change;

	return retval;
}