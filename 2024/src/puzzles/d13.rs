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
            let a = lines[0].split_once(":").unwrap().1.split_once(",").unwrap();
            let ax = &a.0.trim()[2..];
            let ay = &a.1.trim()[2..];
            let b = lines[1].split_once(":").unwrap().1.split_once(",").unwrap();
            let bx = &b.0.trim()[2..];
            let by = &b.1.trim()[2..];
            let prize = lines[2].split_once(":").unwrap().1.split_once(",").unwrap();
            let px = &prize.0.trim()[2..];
            let py = &prize.1.trim()[2..];

            (
                ax.parse::<i64>().unwrap(),
                ay.parse::<i64>().unwrap(),
                bx.parse::<i64>().unwrap(),
                by.parse::<i64>().unwrap(),
                px.parse::<i64>().unwrap(),
                py.parse::<i64>().unwrap(),
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
