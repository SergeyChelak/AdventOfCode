use std::fmt::Write;

pub trait KnotHashable {
    fn knot_hash(&self) -> String;
}

impl KnotHashable for String {
    fn knot_hash(&self) -> String {
        let hasher = KnotHash::new();
        hasher.hash(self)
    }
}

impl KnotHashable for str {
    fn knot_hash(&self) -> String {
        let hasher = KnotHash::new();
        hasher.hash(self)
    }
}

struct KnotHash {
    buffer_size: usize,
    rounds: usize,
}

impl KnotHash {
    fn new() -> Self {
        Self {
            buffer_size: 256,
            rounds: 64,
        }
    }

    fn hash(&self, s: &str) -> String {
        let mut input = s.bytes().collect::<Vec<u8>>();
        let mut offset = vec![17u8, 31, 73, 47, 23];
        input.append(&mut offset);
        tie_knot(self.buffer_size, self.rounds, &input)
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0, |acc, val| acc ^ val))
            .fold(String::new(), |mut output, b| {
                _ = write!(output, "{b:02x}");
                output
            })
    }
}

pub fn tie_knot(size: usize, rounds: usize, input: &[u8]) -> Vec<u8> {
    let mut arr = vec![0u8; size]
        .iter_mut()
        .enumerate()
        .map(|(idx, _)| idx as u8)
        .collect::<Vec<u8>>();
    let mut pos = 0usize;
    let mut skip_size = 0usize;
    for _ in 0..rounds {
        for len in input {
            let len = *len as usize;
            for i in 0..len / 2 {
                let a = (pos + i) % size;
                let b = (pos + len - i - 1) % size;
                arr.swap(a, b);
            }
            pos += len + skip_size;
            skip_size += 1;
        }
    }
    arr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knot_hash_example1() {
        let arr = tie_knot(5, 1, &[3, 4, 1, 5]);
        assert_eq!(arr, [3, 4, 2, 1, 0]);
    }

    #[test]
    fn knot_hash_example2() {
        assert_eq!("".knot_hash(), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!("AoC 2017".knot_hash(), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!("1,2,3".knot_hash(), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!("1,2,4".knot_hash(), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
