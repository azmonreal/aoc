use std::collections::HashMap;

#[test]
fn test() {
    let (p1, p2) = solve(String::from("125 17"));
    assert_eq!(p1, "55312");
    assert_eq!(p2, "65601038650482");
}

pub fn solve(data: String) -> (String, String) {
    let original = data
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut stones = original.clone();

    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
            } else {
                let str = stones[i].to_string();
                if str.len() % 2 == 0 {
                    let old = stones.remove(i).to_string();
                    let (a, b) = old.split_at(old.len() / 2);
                    stones.insert(i, a.parse::<u64>().unwrap());
                    stones.insert(i + 1, b.parse::<u64>().unwrap());
                    i += 1;
                } else {
                    stones[i] *= 2024;
                }
            }
            i += 1;
        }
    }

    let mut cache: HashMap<u64, HashMap<usize, u64>> = HashMap::new();

    fn blink(
        val: u64,
        remaining: usize,
        cache: &mut HashMap<u64, HashMap<usize, u64>>,
        _max: usize,
    ) -> u64 {
        if remaining == 0 {
            return 1;
        }

        if let Some(v) = cache.get(&val) {
            if let Some(l) = v.get(&remaining) {
                return *l;
            }
        }

        let len;
        if val == 0 {
            len = blink(1, remaining - 1, cache, _max);
        } else {
            let str = val.to_string();
            if str.len() % 2 == 0 {
                let (a, b) = str.split_at(str.len() / 2);
                len = blink(a.parse::<u64>().unwrap(), remaining - 1, cache, _max)
                    + blink(b.parse::<u64>().unwrap(), remaining - 1, cache, _max);
            } else {
                len = blink(val * 2024, remaining - 1, cache, _max);
            }
        }

        cache.entry(val).or_default().insert(remaining, len);

        len
    }

    let len = original
        .iter()
        .fold(0, |acc, x| acc + blink(*x, 75, &mut cache, 75));

    (stones.len().to_string(), len.to_string())
}
