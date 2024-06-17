use std::fmt;
use rand::Rng;

struct Board {
    dim: usize,
    size: usize,
    tiles: Vec<char>,
}

impl Board {
    fn new(dim: usize) -> Self {
        Self {
            dim,
            size: dim*dim,
            tiles: Vec::with_capacity(dim*dim),
        }
    }

    fn fill_random(&mut self) {
        self.tiles = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            let num: u32 = rand::thread_rng().gen_range(65..=90);
            let character = char::from_u32(num);
            match character {
                Some(c) => self.tiles.push(c),
                None => unreachable!(),
            }
        }
    }
    
    fn get_valid_moves(&self, idx_seq: Vec<usize>) -> Vec<usize> {
        let curr = *idx_seq.iter().last().unwrap();
        let mut valid_moves = Vec::new();
        for i in -1..=1 {
            if i == -1 && curr % self.dim == 0 {
                continue;
            }
            else if i == 1 && (curr + 1) % self.dim == 0 {
                continue;
            }
            for j in -1..=1 {
                if j == -1 && curr < self.dim {
                    continue
                }
                else if j == 1 && curr > (self.size - self.dim) {
                    continue
                }
                let valid_move = usize::try_from(curr as i32 + i + j * self.dim as i32);
                match valid_move {
                    Ok(m) => valid_moves.push(m),
                    Err(_) => unreachable!(),
                }
            }
        }
        valid_moves.retain(|x| !idx_seq.contains(x));
        valid_moves.sort();
        return valid_moves;
    }

    fn get_word_from_seq(&self, idx_seq: Vec<usize>) -> String {
        let mut result = String::new();
        for &i in idx_seq.iter() {
            result.push(self.tiles[i]);
        }
        return result;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for (i, &e) in self.tiles.iter().enumerate() {
            output.push(e);
            output.push_str("   ");
            if (i + 1) % self.dim == 0 && i + 1 < self.size {
                output.push_str("\n\n");
            }
        }
        write!(f, "{}", output)
    }
}   

fn main() {
    let mut b = Board::new(4);
    b.fill_random();
    println!("{b}");

    let w = b.get_word_from_seq(vec![0, 4, 5, 10]);
    println!("Sequence [0, 4, 5, 10] => {w}");

    for i in 0..b.size {
        println!("Valid moves from {} => {:?}", i, b.get_valid_moves(vec![i]));
    }
}
