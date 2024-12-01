#[test]
fn test_p1() {
    let (p1, p2) = solve(String::from(
        "3   4
4   3
2   5
1   3
3   9
3   3",
    ));
    assert_eq!(p1, "11");
    assert_eq!(p2, "31");
}

pub fn solve(data: String) -> (String, String) {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = data
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap());
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();
            (a, b)
        })
        .unzip();

    a.sort();
    b.sort();

    let dif = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .collect::<Vec<i32>>();

    let count = a
        .iter()
        .map(|x| x * b.iter().filter(|y| x == *y).count() as i32)
        .collect::<Vec<i32>>();

    (
        dif.iter().sum::<i32>().to_string(),
        count.iter().sum::<i32>().to_string(),
    )
}
