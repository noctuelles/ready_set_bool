/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   gray_code.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/05 18:22:54 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/09 22:03:32 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/* https://www.youtube.com/watch?v=xuw8HSEP_eI */
pub fn gray_code(n: u32) -> u32 {
    let mut r = 0;
    let mut i = 0;

    while i < 31 {
        let a = n & (1 << i);
        let n = (n & (1 << (i + 1))) >> 1;
        r |= a ^ n;
        i = i + 1;
    }
    r |= n & (1 << 31);
    r
}

mod tests {
    use super::gray_code;

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(0b0111), 0b0100);
        assert_eq!(gray_code(0b1000), 0b1100);
        assert_eq!(gray_code(0b1111), 0b1000);
        assert_eq!(gray_code(0b1010), 0b1111);
        assert_eq!(gray_code(0b10010011100011), 0b11011010010010);
        assert_eq!(
            gray_code(0b1111011010111111101110011),
            0b1000110111100000011001010
        );
        assert_eq!(
            gray_code(0b11010010110011001110101001001001),
            0b10111011101010101001111101101101
        );
    }
}
