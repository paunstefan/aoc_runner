#[macro_export]
macro_rules! run_day {
    ( $x:ident, $input:expr ) => {{
        println!("\n{}:", stringify!($x));
        let start = std::time::Instant::now();
        let p1 = $x::run_part1($input);
        let duration = start.elapsed();
        println!("Part 1: {} ({:?})", p1, duration);

        let start = std::time::Instant::now();
        let p1 = $x::run_part2($input);
        let duration = start.elapsed();
        println!("Part 2: {} ({:?})", p1, duration);
    }};
}

#[macro_export]
macro_rules! aoc_days{
    ($($day:ident),+ $(,)?) => {
        $(mod $day;)*
        pub fn match_day(d: &str) {
            let d = format!("{}{}", "aoc", d);
            match &*d {
                $(stringify!($day) => run_day!($day, &format!("data/{}", stringify!($day))),)*
                _ => eprintln!("Day not valid"),
            }
        }
    };
}
