/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   input.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@students.42wolfsburg.de +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/01/28 21:27:27 by vdragomi          #+#    #+#             */
/*   Updated: 2022/01/28 21:45:54 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;

pub fn input_get() -> String {
    println!("Please insert your input: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Imput was not correctly passed!");
    return String::from(input.trim());
}
