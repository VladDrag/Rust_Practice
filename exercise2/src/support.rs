/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   support.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@students.42wolfsburg.de +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/30 16:27:40 by vdragomi          #+#    #+#             */
/*   Updated: 2022/01/30 18:44:02 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;

pub fn vocal_checker(word: &String) -> i32 {
    let vocals = ['a', 'e', 'i', 'o', 'u'];

    for vocal in vocals {
        if word
            .chars()
            .nth(word.len() - 1) //checks the last element of the 'word' String
            .expect("The string element cannot be written as a char.")
            == vocal
        {
            return 0;
        } else {
            if vocal == 'u' { //we will write the 'word' inside our file if the last char is not a vocal AND if all vocals have been checked
                return 2;
            } else {
                continue;
            }
        }
    }
    return 1;
}

pub fn file_write(mut file: &std::fs::File, args: &String) {
    file.write_all(args.as_bytes())
        .expect("The string was not written");
    file.write_all(b"\n").expect("The string was not written");
}
