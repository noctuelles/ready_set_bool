/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   adder.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/05 17:36:36 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/11 17:27:25 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn adder(a: u32, b: u32) -> u32 {
    let mut c = 0;
    let mut r = 0;
    let mut i = 0;

    while i < 32 {
        let a = a & (1 << i);
        let b = b & (1 << i);
        r |= a ^ b ^ c;
        c = ((a & b) | (a & c) | (b & c)) << 1; /* If there's at least two variable that is true, set the carry for the next bit-wise XOR. */
        i = i + 1;
    }
    r
}
