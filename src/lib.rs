#[macro_export]
macro_rules! run_day {
    ( $x:ident, $input:expr ) => {{
        println!("\n{}:", stringify!($x));
        let start = std::time::Instant::now();
        let p1 = $x::run_part1($input);
        let duration = start.elapsed();
        println!("Part 1: {} ({:?})", p1, duration);

        let start = std::time::Instant::now();
        let p2 = $x::run_part2($input);
        let duration = start.elapsed();
        println!("Part 2: {} ({:?})", p2, duration);
    }};
}

#[macro_export]
macro_rules! run_day_test {
    ( $x:ident, $input:expr, $res:expr ) => {{
        let results: Vec<i64> = std::fs::read_to_string($res)
            .unwrap()
            .lines()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        if results.is_empty() {
            println!("No results in file.");
            return;
        }

        println!("\nTesting {}:", stringify!($x));
        let start = std::time::Instant::now();
        let p1 = $x::run_part1($input);
        let duration = start.elapsed();

        let good = if results[0] == p1 {
            "PASSED".to_string()
        } else {
            format!("FAILED, expected {}", results[0])
        };
        println!("Part 1: {} - {} ({:?})", p1, good, duration);

        if results.len() == 2 {
            let start = std::time::Instant::now();
            let p2 = $x::run_part2($input);
            let duration = start.elapsed();

            let good = if results[1] == p2 {
                "PASSED".to_string()
            } else {
                format!("FAILED, expected {}", results[1])
            };
            println!("Part 2: {} - {} ({:?})", p2, good, duration);
        }
    }};
}

#[macro_export]
macro_rules! aoc_days{
    ($($day:ident),+ $(,)?) => {
        $(mod $day;)*
        pub fn match_day(d: &str) {
            let d = format!("{}{}", "aoc", d);

            match &*d {
                $(stringify!($day) => run_day!($day, &format!("data/inputs/{}", stringify!($day))),)*
                _ => eprintln!("Day not valid"),
            }
        }

        pub fn match_day_test(d: &str){
            let d = format!("{}{}", "aoc", d);

            match &*d {
                $(stringify!($day) => run_day_test!($day, &format!("data/tests/{}", stringify!($day)),
                                        &format!("data/tests/{}_results", stringify!($day))),)*
                _ => eprintln!("Day not valid"),
            }
        }
    };
}
