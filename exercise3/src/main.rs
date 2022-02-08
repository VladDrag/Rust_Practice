/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@students.42wolfsburg.de +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/02/06 12:20:33 by vdragomi          #+#    #+#             */
/*   Updated: 2022/02/06 19:57:20 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/* In this exercise, we're going to get a CL imput and attribute it to a created structure.
The input will represent:
- a character, as a flag, representing if it is a filled rectangle, or an empty one; it might be 'r' for empty rectangles or 'R' for full rects;
- length of a rectangle, an integer larger than 0 and up until 30;
- height of a rectangle, an integer larger than 0 and up until 30;
- color of a rectangle, a char representing the background of a rectangle;
 */

 use std::env;
 use std::process::exit;

#[derive(Debug)]
 struct Rect {
	flag: char,
	length: u8,
	height: u8,
	color: char
 }
 
 impl Rect {
	 fn new(flag:char, length: u8, height: u8, color: char) -> Rect
	 {
		Rect{
			flag: flag,
			length: length,
			height: height,
			color: color
		}
	 }
 }

 fn main() {
    let args:Vec<String> = env::args().collect();
	if args.len() != 5
	{
		println!("Please insert the 4 measurements required.");
	}

	// let chars:Vec<_> = args[1].chars().collect();
	// if chars.len() != 1 || !(chars[0] != 'r' || chars[0] != 'R')
	// {
	// 	println!("Please insert the correct character. The first argument should be either 'r' or 'R'");
	// 	exit(1);
	// }
	let flag = args[1].parse::<char>().expect("Please input a valid character. It should be either 'r' or 'R'");
	if flag != 'r' && flag != 'R'
	{
		println!("Please input a valid character. It should be either 'r' or 'R'");
		exit(1);
	}
	let length:u8 = args[2].parse::<u8>().expect("Please input a valid value, from 0 to 30");
	let height:u8 = args[3].parse::<u8>().expect("Please input a valid value, from 0 to 30");
	if length > 30 || length == 0 ||  height > 30 || height == 0
	{
		println!("Please input a valid value, from 0 to 30");
		exit(1);
	}
	let color = args[4].parse::<char>().expect("Please input a valid character.");
	let rectangle = Rect::new(flag, length, height, color);
	// println!("{:?}", &rectangle);
	
	if rectangle.flag == 'R'{
		for _i in 0..=rectangle.height {
			for _j in 0..=rectangle.length { print!("{}", rectangle.color);}
			print!("\n");
		}
	}
	else {
		for i in 0..=rectangle.height {
			if i == 0  || i == rectangle.height{
			for _j in 0..=rectangle.length { print!("{}", rectangle.color);}
			print!("\n");}
			else
			{
				for j in 0..=rectangle.length { 
					if j == 0 || j == rectangle.length { print!("{}", rectangle.color);}
					else
					{
						print!(" ");
					}
				}
				print!("\n");
			}
		}
	}



 }
