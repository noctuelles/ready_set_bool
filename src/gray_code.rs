/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   gray_code.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/05 18:22:54 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/05 19:41:06 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/* https://www.youtube.com/watch?v=xuw8HSEP_eI */
pub fn gray_code(n: u32) -> u32 {
    let mut i = 63;
    let mut r = 0;
    let mut p = 0;
    let mut a = 0;

    while i < 64 {
        if i == 63 {
            a = 0;
        } else {
            a = n & (1 << i);
        }
        r |= a ^ (p << 1);
        p = a;
        i = i + 1;
    }
    r
}

mod tests {
    use super::gray_code;

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(8), 12)
    }
}
