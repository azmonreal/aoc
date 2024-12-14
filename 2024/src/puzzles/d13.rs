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
                a[0].parse::<i64>().unwrap(),
                a[1].parse::<i64>().unwrap(),
                b[0].parse::<i64>().unwrap(),
                b[1].parse::<i64>().unwrap(),
                prize[0].parse::<i64>().unwrap(),
                prize[1].parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let solve = |ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64| -> Option<(i64, i64)> {
        let a = (px * by - py * bx) / (ax * by - ay * bx);
        let b = (px - ax * a) / bx;

        if ax * a + bx * b == px && ay * a + by * b == py {
            return Some((a, b));
        }
        None
    };

    let tokens = machines
        .iter()
        .filter_map(|(ax, ay, bx, by, px, py)| {
            if let Some((a, b)) = solve(*ax, *ay, *bx, *by, *px, *py) {
                return Some(a * 3 + b);
            }
            None
        })
        .sum::<i64>();

    let tokensl = machines
        .iter()
        .filter_map(|(ax, ay, bx, by, mut px, mut py)| {
            px += 10000000000000;
            py += 10000000000000;

            if let Some((a, b)) = solve(*ax, *ay, *bx, *by, px, py) {
                return Some(a * 3 + b);
            }
            None
        })
        .sum::<i64>();

    (tokens.to_string(), tokensl.to_string())
}
