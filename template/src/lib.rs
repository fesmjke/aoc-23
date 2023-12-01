#[macro_export]
macro_rules! read_file {
    ($p:expr) => {
        {
            let file = File::open($p).expect("Cannot read file!");
            let mut reader = BufReader::new(file);

            reader.lines()
        }
    };
}