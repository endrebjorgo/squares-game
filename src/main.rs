use std::fmt;
use rand::Rng;

struct Board {
    dim: u32,
    size: u32,
    tiles: Vec<char>,
}

impl Board {
    fn new(dim: u32) -> Self  {
        let s = dim*dim;
        Self {
            dim,
            size: s,
            tiles: Vec::with_capacity(s as usize),
        }
    }

    fn fill_random(&mut self) {
        self.tiles = Vec::with_capacity(self.size as usize);
        for _ in 0..self.size {
            let num: u32 = rand::thread_rng().gen_range(65..=90);
            let character = char::from_u32(num);
            match character {
                Some(c) => self.tiles.push(c),
                None => unreachable!(),
            }
        }
    }
    
    fn get_valid_moves(&self, idx_seq: &Vec<u32>) -> Vec<u32> {
        if idx_seq.is_empty() {
            return (0..self.size).collect();
        }
        let curr_idx = *idx_seq.iter().last().unwrap();
        let (x, y) = (curr_idx % self.dim, curr_idx / self.dim);
        let mut valid_moves = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_x = x as i32 + dx as i32;
                let new_y = y as i32 + dy as i32;

                let x_in_range = 0 <= new_x && new_x < self.dim as i32;
                let y_in_range = 0 <= new_y && new_y < self.dim as i32;

                if x_in_range && y_in_range {
                    let new_idx = (new_x + new_y * self.dim as i32) as u32;
                    valid_moves.push(new_idx);
                }
            }
        }
        valid_moves.retain(|x| !idx_seq.contains(x));
        valid_moves.sort();
        return valid_moves;
    }

    fn get_word_from_seq(&self, idx_seq: &Vec<u32>) -> String {
        let mut result = String::new();
        for &i in idx_seq.iter() {
            result.push(self.tiles[i as usize]);
        }
        return result;
    }

    fn get_all_seqs(&self, idx_seq: &Vec<u32>) -> Vec<Vec<u32>> {
        let mut all_seqs = Vec::new();

        let valid_moves = self.get_valid_moves(idx_seq);

        
        if valid_moves.is_empty() {
            return Vec::<Vec<u32>>::new();
        }
        for &m in valid_moves.iter() {
            let mut seq = idx_seq.clone();
            seq.push(m);

            let mut next_seqs = self.get_all_seqs(&seq);
            all_seqs.push(seq);
            all_seqs.append(&mut next_seqs);
        }

        return all_seqs;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for (i, &e) in self.tiles.iter().enumerate() {
            output.push(e);
            output.push_str("   ");
            if (i as u32 + 1) % self.dim == 0 && (i as u32 + 1) < self.size {
                output.push_str("\n\n");
            }
        }
        write!(f, "{}", output)
    }
}   

fn main() {
    let mut b = Board::new(3);
    b.fill_random();
    println!("{b}");

    let w = b.get_word_from_seq(&vec![0, 1]);
    println!("Sequence [0, 4, 5, 10] => {w}");

    for i in 0..b.size {
        println!("Valid moves from {} => {:?}", i, b.get_valid_moves(&vec![i]));
    }

    let all_seqs = b.get_all_seqs(&Vec::new());
    println!("{:?}", all_seqs);
    println!("{:?}", all_seqs.len());
}
