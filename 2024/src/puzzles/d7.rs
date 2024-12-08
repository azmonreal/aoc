#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    ));
    assert_eq!(p1, "3749");
    assert_eq!(p2, "11387");
}

fn to_base_n(val: i64, base: i64, len: usize) -> Vec<i64> {
    let mut val = val;
    let mut res = vec![];
    while val > 0 {
        res.push(val % base);
        val /= base;
    }
    let padding = len - res.len();
    res.extend(vec![0; padding]);
    res
}

pub fn solve(data: String) -> (String, String) {
    let eqs = data
        .lines()
        .map(|l| {
            let (res, vals) = l.split_once(":").unwrap();
            let res = res.parse::<i64>().unwrap();
            let vals = vals
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (res, vals)
        })
        .collect::<Vec<_>>();

    let max_len = eqs.iter().map(|(_, vals)| vals.len()).max().unwrap();

    let ops = (0..(1 << max_len))
        .map(|b: u64| format!("{:064b}", b).chars().rev().collect::<String>())
        .collect::<Vec<_>>();
    let sum_2op = eqs
        .iter()
        .filter_map(|(res, vals)| {
            if ops[..2_usize.pow(vals.len() as u32)].iter().any(|op| {
                vals.iter().zip(op.chars()).fold(0, |acc, (v, o)| match o {
                    '0' => acc + v,
                    '1' => acc * v,
                    _ => acc,
                }) == *res
            }) {
                return Some(res);
            }
            None
        })
        .sum::<i64>();

    // PERF: too slow!!!
    let ops = (0..(3_i64.pow(max_len as u32)))
        .map(|b| {
            to_base_n(b, 3, max_len)
                .iter()
                .map(|v| v.to_string())
                .rev()
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let sum_3op = eqs
        .iter()
        .filter_map(|(res, vals)| {
            if ops.iter().any(|op| {
                vals.iter().zip(op.chars()).fold(0, |acc, (v, o)| match o {
                    '0' => acc + v,
                    '1' => acc * v,
                    '2' => (acc.to_string() + &v.to_string()).parse::<i64>().unwrap(),
                    _ => acc,
                }) == *res
            }) {
                return Some(res);
            }
            None
        })
        .sum::<i64>();

    (sum_2op.to_string(), sum_3op.to_string())
}
