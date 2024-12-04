#[test]
fn tes() {
    let (p1, p2) = solve(String::from(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    ));
    assert_eq!(p1, "2");
    assert_eq!(p2, "4");
}

fn is_report_safe(report: &[i32]) -> bool {
    let safe_difs = report
        .windows(2)
        .map(|w| (w[0] - w[1]).abs())
        .all(|d| d <= 3);

    safe_difs && (report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1]))
}

pub fn solve(data: String) -> (String, String) {
    let reports = data
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let len = reports.len();

    let notsafe = reports
        .into_iter()
        .filter(|levels| !is_report_safe(levels))
        .collect::<Vec<Vec<i32>>>();

    let safe_count = len - notsafe.len();

    // NOTE: brute force is fast enough (face palm)
    //
    // let safeish = notsafe
    //     .into_iter()
    //     .filter(|levels| {
    //         let difs = levels
    //             .windows(2)
    //             .map(|w| (w[1] - w[0]))
    //             .collect::<Vec<i32>>();
    //
    //         let ascending = levels.windows(2).filter(|w| w[0] < w[1]).count() > (difs.len() / 2);
    //
    //         let errors = difs
    //             .iter()
    //             .enumerate()
    //             .filter_map(|(i, d)| {
    //                 if *d == 0 || *d < -3 || *d > 3 || *d > 0 && !ascending || *d < 0 && ascending {
    //                     Some((i, *d))
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .collect::<Vec<(usize, i32)>>();
    //
    //         errors.len() == 1
    //             && ({
    //                 let mut n = levels.clone();
    //                 n.remove(errors[0].0);
    //                 is_report_safe(&n)
    //             } || {
    //                 let mut n = levels.clone();
    //                 n.remove(errors[0].0 + 1);
    //                 is_report_safe(&n)
    //             })
    //             || errors.len() == 2 && {
    //                 let mut n = levels.clone();
    //                 n.remove(errors[1].0);
    //                 is_report_safe(&n)
    //             }
    //     })
    //     .collect::<Vec<Vec<i32>>>();

    let safeish = notsafe
        .into_iter()
        .filter(|levels| {
            for i in 0..levels.len() {
                let mut n = levels.clone();
                n.remove(i);
                if is_report_safe(&n) {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<Vec<i32>>>();

    (
        safe_count.to_string(),
        (safe_count + safeish.len()).to_string(),
    )
}
