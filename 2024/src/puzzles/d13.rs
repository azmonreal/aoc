use regex::Regex;

#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    ));
    assert_eq!(p1, "480");
    assert_eq!(p2, "");
}

fn gcd(a: usize, b: usize) -> usize {
    // https://en.wikipedia.org/wiki/Euclidean_algorithm
    let mut max = a.max(b);
    let mut min = a.min(b);

    while min != max {
        let d = max - min;
        (max, min) = (min.max(d), min.min(d));
    }
    min
}

pub fn solve(data: String) -> (String, String) {
    let machines = data
        .split("\n\n")
        .map(|machine| {
            let lines = machine.lines().collect::<Vec<_>>();
            let a = &Regex::new(r"X\+(\d+), Y\+(\d+)")
                .unwrap()
                .captures(lines[0])
                .unwrap();
            let b = &Regex::new(r"X\+(\d+), Y\+(\d+)")
                .unwrap()
                .captures(lines[1])
                .unwrap();
            let prize = &Regex::new(r"X=(\d+), Y\=(\d+)")
                .unwrap()
                .captures(lines[2])
                .unwrap();

            (
                a[1].parse::<usize>().unwrap(),
                a[2].parse::<usize>().unwrap(),
                b[1].parse::<usize>().unwrap(),
                b[2].parse::<usize>().unwrap(),
                prize[1].parse::<usize>().unwrap(),
                prize[2].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let reachable = machines
        .iter()
        .filter(|(ax, ay, bx, by, px, py)| px % gcd(*ax, *bx) == 0 && py % gcd(*ay, *by) == 0)
        .collect::<Vec<_>>();

    let tokens = reachable
        .iter()
        .filter_map(|(ax, ay, bx, by, px, py)| {
            for i in 1..=100 {
                for j in 1..=100 {
                    if ax * i + bx * j == *px && ay * i + by * j == *py {
                        return Some(i * 3 + j);
                    }
                }
            }
            None
        })
        .sum::<usize>();

    (tokens.to_string(), String::new())
}
