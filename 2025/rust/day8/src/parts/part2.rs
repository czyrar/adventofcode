use std::collections::HashMap;

pub fn part2(input: &str) -> usize {
    let coords: Vec<_> = input
        .lines()
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut dists = HashMap::new();
    for (i, c1) in coords.iter().enumerate() {
        for (j, c2) in coords[(i + 1)..].iter().enumerate() {
            let dx = c1[0] - c2[0];
            let dy = c1[1] - c2[1];
            let dz = c1[2] - c2[2];
            dists.insert((i, i + 1 + j), dx * dx + dy * dy + dz * dz);
        }
    }

    // At the beggining we have n groups of one element
    let mut groups = HashMap::new();
    for i in 0..coords.len() {
        groups.insert(i, i);
    }

    let mut min_so_far = 0;
    loop {
        let (closest, min) = dists
            .iter()
            .filter(|&(_, d)| *d > min_so_far)
            .min_by_key(|&(_, d)| d)
            .unwrap();
        min_so_far = *min;
        let g0 = *groups.get(&closest.0).unwrap();
        let g1 = *groups.get(&closest.1).unwrap();
        if g0 == g1 {
            continue;
        }
        let mut equal = 0;
        for (_, v) in groups.iter_mut() {
            if *v == g0 {
                equal += 1;
            }
            if *v == g1 {
                *v = g0;
                equal += 1;
            }
        }
        if equal == coords.len() {
            return (coords[closest.0][0] * coords[closest.1][0]) as usize;
        }
    }
}
