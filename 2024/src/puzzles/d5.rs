#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    ));
    assert_eq!(p1, "143");
    assert_eq!(p2, "123");
}

pub fn solve(data: String) -> (String, String) {
    let input = data.split_once("\n\n").unwrap();

    let mut rules: Vec<Vec<i32>> = input
        .0
        .lines()
        .map(|l| l.split("|").map(|e| e.parse().unwrap()).collect())
        .collect();

    rules.sort();

    let updates: Vec<Vec<i32>> = input
        .1
        .lines()
        .map(|l| l.split(",").map(|e| e.parse().unwrap()).collect())
        .collect();

    let correct: Vec<&Vec<i32>> = updates
        .iter()
        .filter(|u| {
            let len = u.len();
            rules.iter().all(|r| {
                u.iter().position(|e| *e == r[0]).unwrap_or(0)
                    <= u.iter().position(|e| *e == r[1]).unwrap_or(len)
            })
        })
        .collect();

    let incorrect: Vec<Vec<i32>> = updates
        .iter()
        .filter_map(|u| {
            if !correct.contains(&u) {
                let len = u.len();
                let mut uc = u.clone();
                rules.iter().for_each(|r| {
                    let pos0 = uc.iter().position(|e| *e == r[0]).unwrap_or(0);
                    let pos1 = uc.iter().position(|e| *e == r[1]).unwrap_or(len);
                    if pos0 > pos1 {
                        let a = uc[pos0];
                        uc.remove(pos0);
                        uc.insert(pos1, a);
                    }
                });
                return Some(uc);
            }
            None
        })
        .collect();

    (
        correct
            .iter()
            .map(|c| c[c.len() / 2])
            .sum::<i32>()
            .to_string(),
        incorrect
            .iter()
            .map(|c| c[c.len() / 2])
            .sum::<i32>()
            .to_string(),
    )
}
