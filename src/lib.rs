#[cfg(test)]
mod tests {

    mod day_1 {
        use day1;

        #[test]
        fn small_a() {
            let value = day1::calibration("./day1/input_small.txt");

            assert_eq!(value, 142);
        }

        #[test]
        fn small_b() {
            let value = day1::calibration("./day1/input_small_b.txt");

            assert_eq!(value, 281);
        }

        #[test]
        fn input() {
            let a = day1::calibration("./day1/input_ab.txt");

            println!("{:?}", a);
        }
    }
    mod day_2 {
        use day2;

        #[test]
        fn small_a() {
            let value = day2::cube_conundrum("./day2/input_small_a.txt");

            // assert_eq!(value, 8); // part a
            assert_eq!(value, 2286); // part b
        }
        #[test]
        fn input() {
            let value = day2::cube_conundrum("./day2/input_ab.txt");

            println!("Sum of game ids -> {value}");

            assert!(value > 0);
        }
    }
}
