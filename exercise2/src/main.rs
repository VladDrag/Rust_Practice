/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@students.42wolfsburg.de +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/29 10:33:32 by vdragomi          #+#    #+#             */
/*   Updated: 2022/01/30 16:52:00 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
    This exercise aims to take command line arguments and
    write them inside a file if they end with a vocal or
    print them if they end with something else
*/

use std::env;

mod support;

fn main() {
    let args: Vec<String> = env::args().collect(); //we collect the CL arguments inside a Vector
    let len = args.len();							//we get the number of arguments
    let mut i = 1;

	//we create a file "foo.txt"
    let file = std::fs::File::create("foo.txt").expect("File could not be created!");
    while i < len {
        if support::vocal_checker(&args[i]) == 0 { //we check if the last character of the argument is a vocal
            support::file_write(&file, &args[i])   //we write the argument inside our file if the last arg is a vocal
        } else {
            if support::vocal_checker(&args[i]) == 2 {
                println!("{}", &args[i]); //we print the argument if the last character is not a vocal and if all the vocals have been checked
            } else {
                continue;
            }
        }
        i += 1;
    }
}
