use std::collections::HashMap;

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

    let rules: HashMap<i32, Vec<i32>> = input.0.lines().fold(HashMap::new(), |mut acc, r| {
        let (lh, rh) = r
            .split_once("|")
            .map(|(lh, rh)| (lh.parse::<i32>().unwrap(), rh.parse::<i32>().unwrap()))
            .unwrap();
        match acc.get_mut(&lh) {
            Some(v) => {
                v.push(rh);
            }
            None => {
                acc.insert(lh, vec![rh]);
            }
        }

        acc
    });

    // let mut rules: Vec<Vec<i32>> = input
    //     .0
    //     .lines()
    //     .map(|l| l.split("|").map(|e| e.parse().unwrap()).collect())
    //     .collect();
    //
    // rules.sort();

    let mut updates: Vec<Vec<i32>> = input
        .1
        .lines()
        .map(|l| l.split(",").map(|e| e.parse().unwrap()).collect())
        .collect();

    let middles: (i32, i32) = updates.iter_mut().fold((0, 0), |acc, u| {
        let len = u.len();
        let incorrect = rules
            .iter()
            .filter(|r| {
                let mut pos0 = u.iter().position(|&e| e == *r.0).unwrap_or(0);

                r.1.iter()
                    .filter(|&&rh| {
                        let pos1 = u.iter().position(|&e| e == rh).unwrap_or(len);
                        if pos0 > pos1 {
                            let a = u[pos0];
                            u.remove(pos0);
                            u.insert(pos1, a);
                            pos0 = pos1;
                            return true;
                        }
                        false
                    })
                    .count()
                    != 0
            })
            .count()
            == 0;
        let mid = u[len / 2];
        (
            acc.0 + mid * incorrect as i32,
            acc.1 + mid * !incorrect as i32,
        )
    });

    // let middles: (i32, i32) = updates.iter_mut().fold((0, 0), |acc, u| {
    //     let len = u.len();
    //     let incorrect = rules
    //         .iter()
    //         .filter(|r| {
    //             let pos0 = u.iter().position(|e| *e == r[0]).unwrap_or(0);
    //             let pos1 = u.iter().position(|e| *e == r[1]).unwrap_or(len);
    //             if pos0 > pos1 {
    //                 let a = u[pos0];
    //                 u.remove(pos0);
    //                 u.insert(pos1, a);
    //                 return true;
    //             }
    //             false
    //         })
    //         .count()
    //         == 0;
    //     let mid = u[len / 2];
    //     (
    //         acc.0 + mid * incorrect as i32,
    //         acc.1 + mid * !incorrect as i32,
    //     )
    // });

    // let correct: Vec<&Vec<i32>> = updates
    //     .iter()
    //     .filter(|u| {
    //         let len = u.len();
    //         rules.iter().all(|r| {
    //             u.iter().position(|e| *e == r[0]).unwrap_or(0)
    //                 <= u.iter().position(|e| *e == r[1]).unwrap_or(len)
    //         })
    //     })
    //     .collect();
    //
    // let incorrect: Vec<Vec<i32>> = updates
    //     .iter()
    //     .filter_map(|u| {
    //         if !correct.contains(&u) {
    //             let len = u.len();
    //             let mut uc = u.clone();
    //             rules.iter().for_each(|r| {
    //                 let pos0 = uc.iter().position(|e| *e == r[0]).unwrap_or(0);
    //                 let pos1 = uc.iter().position(|e| *e == r[1]).unwrap_or(len);
    //                 if pos0 > pos1 {
    //                     let a = uc[pos0];
    //                     uc.remove(pos0);
    //                     uc.insert(pos1, a);
    //                 }
    //             });
    //             return Some(uc);
    //         }
    //         None
    //     })
    //     .collect();

    (middles.0.to_string(), middles.1.to_string())
}
