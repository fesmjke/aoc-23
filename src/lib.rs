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
}
