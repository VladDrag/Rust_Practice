/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@students.42wolfsburg.de +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/28 21:20:32 by vdragomi          #+#    #+#             */
/*   Updated: 2022/01/30 18:44:43 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*program that aims to:
1) read input from user
2) create a new text file
3) write the input from the user inside the text file
4) BONUS: using a reference to print input at the end, thus testing the burrowing mechanism.
     For testing, remove the reff on row 27 and observe the result!
*/

mod input;
use std::fs;

fn main() {
    let input = input::input_get();
    fs::write("foo.txt", &input).expect("Input could not be writte inside the new file!");
}
