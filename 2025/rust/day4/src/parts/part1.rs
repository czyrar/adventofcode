pub fn part1(input: &str) -> usize {
    let length = input.find("\n").unwrap();
    let flat = input.replace("\n", "");
    let nlines = flat.len() / length;
    let bytes = flat.as_bytes();
    flat.char_indices()
        .filter(|&(i, _)| {
            if bytes[i] == 0x2e {
                return false;
            }

            let modulus = i % length;
            let upper = i >= length;
            let lower = i < (nlines - 1) * length;
            let right = modulus < length - 1;
            let left = modulus > 0;

            let mut count = 0;
            if upper {
                if left {
                    if bytes[i - length - 1] == 0x40 {
                        count += 1;
                    }
                }
                if bytes[i - length] == 0x40 {
                    count += 1;
                }
            }
            if right {
                if upper {
                    if bytes[i - length + 1] == 0x40 {
                        count += 1;
                    }
                }
                if bytes[i + 1] == 0x40 {
                    count += 1;
                }
            }
            if lower {
                if right {
                    if bytes[i + length + 1] == 0x40 {
                        count += 1;
                    }
                }
                if bytes[i + length] == 0x40 {
                    count += 1;
                }
            }
            if left {
                if lower {
                    if bytes[i + length - 1] == 0x40 {
                        count += 1;
                    }
                }
                if bytes[i - 1] == 0x40 {
                    count += 1;
                }
            }
            count < 4
        })
        .count()
}
