#[test]
fn test() {
    let (p1, p2) = solve(String::from("2333133121414131402"));
    assert_eq!(p1, "1928");
    assert_eq!(p2, "2858");
}

pub fn solve(data: String) -> (String, String) {
    let disk_map = data
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut blocks = disk_map
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, &c)| {
            if i % 2 == 0 {
                acc.append(&mut vec![(i / 2) as i64; c as usize]);
            } else {
                acc.append(&mut vec![-1; c as usize]);
            }
            acc
        });

    // HACK: why is this needed
    let binding = blocks.clone();
    let not_free = binding
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, &b)| b != -1)
        .collect::<Vec<_>>();
    let free = blocks
        .clone()
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b == -1 { Some(i) } else { None })
        .collect::<Vec<_>>();

    // NOTE: can probably just update the index for the block instead of fully swapping
    free.iter().zip(not_free).for_each(|(&f, (nf, b))| {
        if f < nf {
            blocks[f] = *b;
            blocks[nf] = -1;
        }
    });

    let checksum = blocks
        .iter()
        .enumerate()
        .filter(|(_, &b)| b != -1)
        .fold(0, |acc, (i, &block)| acc + i as i64 * block);

    let mut space_map = disk_map
        .chunks(2)
        .enumerate()
        .map(|(i, spaces)| (i, spaces[0], spaces.get(1).cloned().unwrap_or(0)))
        .collect::<Vec<_>>();

    for i in (0..space_map.len()).rev() {
        let pos = space_map.iter().position(|&(id, _, _)| id == i).unwrap();

        let mut file = space_map.remove(pos);

        if let Some(fit) = space_map.iter().position(|&(_, _, free)| free >= file.1) {
            if fit < pos {
                space_map[pos - 1].2 += file.1 + file.2;
                file.2 = space_map[fit].2 - file.1;
                space_map[fit].2 = 0;

                space_map.insert(fit + 1, file);
            } else {
                space_map.insert(pos, file);
            }
        } else {
            space_map.insert(pos, file);
        }
    }
    let map_cecksum = space_map
        .iter()
        .fold((0, 0), |(pos, sum), &(id, size, free)| {
            (
                pos + size + free,
                sum + id as u64 * ((pos * 2 + size - 1) as f64 * (size as f64 / 2_f64)) as u64,
            )
        })
        .1;

    (checksum.to_string(), map_cecksum.to_string())
}
