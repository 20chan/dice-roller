fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let cnt: usize = args[1].parse().expect("please give me number");
    println!(
        "Now rolling {} {}..",
        cnt,
        if cnt == 1 { "dice" } else { "dices" }
    );

    let mut result = 0;
    for i in 0..7 {
        std::thread::sleep(std::time::Duration::from_millis(i * 150));
        result = roll(cnt, true);
    }

    println!("Finally.. result is {} !", result);
}

fn roll(cnt: usize, grid: bool) -> usize {
    let mut is = Vec::new();
    let mut total = 0;
    for _ in 0..cnt {
        let i = (rand::random::<usize>() + 6) % 6 + 1;
        is.push(i);
        total += i;
    }
    println!("{}", make_dices(is, grid));
    total
}

fn make_dices(is: Vec<usize>, grid: bool) -> String {
    is.iter()
        .map(|&i| make_dice(i, grid))
        .fold(
            vec![
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            |v, s| {
                v.iter()
                    .zip(s)
                    .map(|(a, b)| format!("{}   {}", a, b))
                    .collect::<Vec<_>>()
            },
        ).join("\n")
}

fn make_dice(i: usize, grid: bool) -> Vec<String> {
    format!(
        "+---+---+---+\n{}\n+---+---+---+",
        dice_str(i)
            .iter()
            .map(|a| format!(
                "|{}|",
                a.iter()
                    .map(|c| if *c { " o " } else { "   " })
                    .collect::<Vec<_>>()
                    .join(if grid { "|" } else { " " })
            )).collect::<Vec<_>>()
            .join(if grid {
                "\n+---+---+---+\n"
            } else {
                "\n+           +\n"
            })
    ).split('\n')
    .map(|s| s.to_string())
    .collect()
}

fn dice_str(i: usize) -> [[bool; 3]; 3] {
    match i {
        1 => [
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ],
        2 => [
            [true, false, false],
            [false, false, false],
            [false, false, true],
        ],
        3 => [
            [false, false, true],
            [false, true, false],
            [true, false, false],
        ],
        4 => [
            [true, false, true],
            [false, false, false],
            [true, false, true],
        ],
        5 => [
            [true, false, true],
            [false, true, false],
            [true, false, true],
        ],
        6 => [
            [true, false, true],
            [true, false, true],
            [true, false, true],
        ],
        _ => panic!("ang"),
    }
}
