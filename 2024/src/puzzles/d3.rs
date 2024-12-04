use regex::Regex;

#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ));
    assert_eq!(p1, "161");
    assert_eq!(p2, "48");
}

pub fn solve(data: String) -> (String, String) {
    let re_dos = Regex::new(r"do\(\)").unwrap();
    let mut dos = re_dos
        .captures_iter(&data)
        .map(|x| x.get(0).unwrap().start())
        .collect::<Vec<_>>();

    dos.insert(0, 0);
    dos = dos.into_iter().rev().collect();

    let re_donts = Regex::new(r"don't\(\)").unwrap();
    let mut donts = re_donts
        .captures_iter(&data)
        .map(|x| x.get(0).unwrap().start())
        .collect::<Vec<_>>();
    donts = donts.into_iter().rev().collect();

    let re_mults = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mults = re_mults
        .captures_iter(&data)
        .map(|x| {
            x.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * x.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .collect::<Vec<_>>();
    let do_mults = re_mults
        .captures_iter(&data)
        .map(|x| {
            let pos = x.get(0).unwrap().start();

            if dos.iter().find(|y| **y < pos) > donts.iter().find(|y| **y < pos) {
                x.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    * x.get(2).unwrap().as_str().parse::<i32>().unwrap()
            } else {
                0
            }
        })
        .collect::<Vec<_>>();

    (
        mults.iter().sum::<i32>().to_string(),
        do_mults.iter().sum::<i32>().to_string(),
    )
}
