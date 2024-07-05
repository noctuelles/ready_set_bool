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

mod tests {
    #[test]
    fn test_adder() {
        assert_eq!(super::adder(13, 11), 13 + 11);
        assert_eq!(super::adder(1, 0), 1 + 0);
        assert_eq!(super::adder(2, 3), 2 + 3);
        assert_eq!(super::adder(192, 758), 192 + 758);
        assert_eq!(super::adder(58372, 27192), 58372 + 27192);
        assert_eq!(super::adder(3871528, 1975728), 3871528 + 1975728);
        assert_eq!(super::adder(2381, 1238), 2381 + 1238);
        assert_eq!(super::adder(1238, 2381), 1238 + 2381);
        assert_eq!(super::adder(0, 0), 0 + 0);
        assert_eq!(super::adder(0, 1), 0 + 1);
        assert_eq!(super::adder(1, 1), 1 + 1);
        assert_eq!(super::adder(1, 2), 1 + 2);
        assert_eq!(super::adder(2, 2), 2 + 2);

        assert_eq!(super::adder(u32::MIN, u32::MAX), u32::MAX);
        assert_eq!(super::adder(u32::MAX, u32::MIN), u32::MAX);
        assert_eq!(
            super::adder(u32::MAX / 2, u32::MAX / 2),
            u32::MAX / 2 + u32::MAX / 2
        );
        assert_eq!(super::adder(u32::MAX / 2, u32::MAX / 2 + 1), u32::MAX);
        assert_eq!(super::adder(u32::MAX / 2 + 1, u32::MAX / 2), u32::MAX);
    }

    #[test]
    #[should_panic]
    fn test_adder_overflow() {
        assert_eq!(super::adder(u32::MAX, 1), 0);
        assert_eq!(super::adder(1, u32::MAX), 0);
        assert_eq!(super::adder(u32::MAX, u32::MAX), 0);
    }
}
