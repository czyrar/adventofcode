pub fn part2(input: &str) -> usize {
    let length = input.find("\n").unwrap();
    let flat = input.replace("\n", "");
    let nlines = flat.len() / length;
    let mut bytes: Vec<&u8> = flat.as_bytes().into_iter().collect();

    let mut removed = 0;
    loop {
        let mut removed_this_iter = 0;
        for i in 0..(flat.len()) {
            if *bytes[i] == 0x2e {
                continue;
            }

            let modulus = i % length;
            let upper = i >= length;
            let lower = i < (nlines - 1) * length;
            let right = modulus < length - 1;
            let left = modulus > 0;

            let mut count = 0;
            if upper {
                if left {
                    if *bytes[i - length - 1] == 0x40 {
                        count += 1;
                    }
                }
                if *bytes[i - length] == 0x40 {
                    count += 1;
                }
            }
            if right {
                if upper {
                    if *bytes[i - length + 1] == 0x40 {
                        count += 1;
                    }
                }
                if *bytes[i + 1] == 0x40 {
                    count += 1;
                }
            }
            if lower {
                if right {
                    if *bytes[i + length + 1] == 0x40 {
                        count += 1;
                    }
                }
                if *bytes[i + length] == 0x40 {
                    count += 1;
                }
            }
            if left {
                if lower {
                    if *bytes[i + length - 1] == 0x40 {
                        count += 1;
                    }
                }
                if *bytes[i - 1] == 0x40 {
                    count += 1;
                }
            }
            if count < 4 {
                bytes[i] = &0x2e;
                removed_this_iter += 1;
            }
        }
        if removed_this_iter == 0 {
            break;
        }
        removed += removed_this_iter;
    }
    removed
}
