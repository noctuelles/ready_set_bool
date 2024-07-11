mod adder;
mod binary_tree;
mod bool_eval;
mod eval_formula;
mod gray_code;
mod multiplier;

fn main() {
    println!("This project being purely an exercice, take a look and run the tests instead !");
}

mod tests {
    mod adder {

        #[test]
        fn test_adder() {
            use crate::adder::adder;

            assert_eq!(adder(13, 11), 13 + 11);
            assert_eq!(adder(1, 0), 1 + 0);
            assert_eq!(adder(2, 3), 2 + 3);
            assert_eq!(adder(192, 758), 192 + 758);
            assert_eq!(adder(58372, 27192), 58372 + 27192);
            assert_eq!(adder(3871528, 1975728), 3871528 + 1975728);
            assert_eq!(adder(2381, 1238), 2381 + 1238);
            assert_eq!(adder(1238, 2381), 1238 + 2381);
            assert_eq!(adder(0, 0), 0 + 0);
            assert_eq!(adder(0, 1), 0 + 1);
            assert_eq!(adder(1, 1), 1 + 1);
            assert_eq!(adder(1, 2), 1 + 2);
            assert_eq!(adder(2, 2), 2 + 2);

            assert_eq!(adder(u32::MIN, u32::MAX), u32::MAX);
            assert_eq!(adder(u32::MAX, u32::MIN), u32::MAX);
            assert_eq!(
                adder(u32::MAX / 2, u32::MAX / 2),
                u32::MAX / 2 + u32::MAX / 2
            );
            assert_eq!(adder(u32::MAX / 2, u32::MAX / 2 + 1), u32::MAX);
            assert_eq!(adder(u32::MAX / 2 + 1, u32::MAX / 2), u32::MAX);
        }

        #[test]
        #[should_panic]
        fn test_adder_overflow() {
            use crate::adder::adder;

            assert_eq!(adder(u32::MAX, 1), 0);
            assert_eq!(adder(1, u32::MAX), 0);
            assert_eq!(adder(u32::MAX, u32::MAX), 0);
        }
    }
    mod multiplier {
        #[test]
        fn test_multiplier() {
            use crate::multiplier::*;

            assert_eq!(multiplier(0, 0), 0 * 0);
            assert_eq!(multiplier(0, 1), 0 * 1);
            assert_eq!(multiplier(1, 0), 1 * 0);
            assert_eq!(multiplier(1, 1), 1 * 1);
            assert_eq!(multiplier(1, 2), 1 * 2);
            assert_eq!(multiplier(2, 1), 2 * 1);
            assert_eq!(multiplier(2, 2), 2 * 2);
            assert_eq!(multiplier(2, 3), 2 * 3);
            assert_eq!(multiplier(3, 2), 3 * 2);
            assert_eq!(multiplier(13, 2), 13 * 2);
            assert_eq!(multiplier(254, 198), 254 * 198);
            assert_eq!(multiplier(1234, 5678), 1234 * 5678);
            assert_eq!(multiplier(12345, 78912), 12345 * 78912);
            assert_eq!(multiplier(u32::MAX, 1), u32::MAX * 1);
            assert_eq!(multiplier(1, u32::MAX), 1 * u32::MAX);
            assert_eq!(multiplier(u32::MAX / 2, 2), (u32::MAX / 2) * 2);
        }

        #[test]
        #[should_panic]
        fn test_multiplier_overflow() {
            use crate::multiplier::*;

            assert_eq!(multiplier(u32::MAX, 2), 0);
            assert_eq!(multiplier(u32::MAX / 2, 3), 0);
            assert_eq!(multiplier(2847182, 9999999), 0);
        }
    }
    mod gray_code {

        #[test]
        fn test_gray_code() {
            use crate::gray_code::*;

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
    // mod eval_formula {
    //     #[test]
    //     fn simple_valid() {
    //         use crate::eval_formula::*;

    //         assert_eq!(false, eval_formula("10&"));
    //         assert_eq!(true, eval_formula("10|"));
    //         assert_eq!(true, eval_formula("11>"));
    //         assert_eq!(false, eval_formula("10="));
    //         assert_eq!(true, eval_formula("1011||="));
    //     }

    //     #[test]
    //     fn intermediate_valid() {
    //         use crate::eval_formula::*;

    //         /* F(X,Y,Z) = (X & Y) | (!X & Z) : If X then Y else Z */

    //         assert_eq!(false, eval_formula("00&0!0&|")); // x = 0, y = 0, z = 0
    //         assert_eq!(false, eval_formula("10&1!0&|")); // x = 1, y = 0, z = 0
    //         assert_eq!(false, eval_formula("01&0!0&|")); // x = 0, y = 1, z = 0
    //         assert_eq!(true, eval_formula("00&0!1&|")); // x = 0, y = 0, z = 1
    //         assert_eq!(true, eval_formula("11&1!0&|")); // x = 1, y = 1, z = 0
    //         assert_eq!(true, eval_formula("01&0!1&|")); // x = 0, y = 1, z = 1
    //         assert_eq!(false, eval_formula("10&1!1&|")); // x = 1, y = 0, z = 1
    //         assert_eq!(true, eval_formula("11&1!1&|")); // x = 1, y = 1, z = 1
    //     }

    //     #[test]
    //     fn invalid() {
    //         use crate::eval_formula::*;

    //         assert_eq!(false, eval_formula("11011"));
    //         assert_eq!(false, eval_formula("1&"));
    //         assert_eq!(false, eval_formula("1|"));
    //         assert_eq!(false, eval_formula("1>"));
    //         assert_eq!(false, eval_formula("1="));
    //         assert_eq!(false, eval_formula("0^"));
    //         assert_eq!(false, eval_formula("!"));
    //         assert_eq!(false, eval_formula("1011||=0"));
    //         assert_eq!(false, eval_formula("1011||=&"));
    //         assert_eq!(false, eval_formula("11&|"));
    //     }
    // }
}
