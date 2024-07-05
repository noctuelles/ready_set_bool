/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   multiplier.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: plouvel <plouvel@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/07/05 17:36:29 by plouvel           #+#    #+#             */
/*   Updated: 2024/07/05 18:13:11 by plouvel          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut i = 0;
    let mut r = 0;

    while i < 32 {
        if b & (1 << i) != 0 {
            r = adder(r, a << i);
        }
        i = i + 1;
    }
    r
}

mod tests {
    #[test]
    fn test_multiplier() {
        assert_eq!(super::multiplier(0, 0), 0 * 0);
        assert_eq!(super::multiplier(0, 1), 0 * 1);
        assert_eq!(super::multiplier(1, 0), 1 * 0);
        assert_eq!(super::multiplier(1, 1), 1 * 1);
        assert_eq!(super::multiplier(1, 2), 1 * 2);
        assert_eq!(super::multiplier(2, 1), 2 * 1);
        assert_eq!(super::multiplier(2, 2), 2 * 2);
        assert_eq!(super::multiplier(2, 3), 2 * 3);
        assert_eq!(super::multiplier(3, 2), 3 * 2);
        assert_eq!(super::multiplier(13, 2), 13 * 2);
        assert_eq!(super::multiplier(254, 198), 254 * 198);
        assert_eq!(super::multiplier(1234, 5678), 1234 * 5678);
        assert_eq!(super::multiplier(12345, 78912), 12345 * 78912);
        assert_eq!(super::multiplier(u32::MAX, 1), u32::MAX * 1);
        assert_eq!(super::multiplier(1, u32::MAX), 1 * u32::MAX);
        assert_eq!(super::multiplier(u32::MAX / 2, 2), (u32::MAX / 2) * 2);
    }

    #[test]
    #[should_panic]
    fn test_multiplier_overflow() {
        assert_eq!(super::multiplier(u32::MAX, 2), 0);
        assert_eq!(super::multiplier(u32::MAX / 2, 3), 0);
        assert_eq!(super::multiplier(2847182, 9999999), 0);
    }
}
