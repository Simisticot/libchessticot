use std::char;
use std::fmt::Debug;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;

pub struct Square(pub u8);

impl Square {
    pub fn rank(&self) -> u8 {
        self.0.div_euclid(8) + 1
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }
}

#[derive(PartialEq)]
pub struct BitBoard(pub u64);

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let integer = self.0;
        let bits = format!("{integer:064b}");
        let ranks = bits
            .chars()
            .collect::<Vec<char>>()
            .chunks(8)
            .map(|rank| rank.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "\n{ranks}")
    }
}

impl BitBoard {
    pub fn from_square_number(square_number: u8) -> BitBoard {
        BitBoard(1 << square_number)
    }

    pub fn occupied(self, square_number: u8) -> bool {
        !((self & BitBoard::from_square_number(square_number)) == BitBoard(0))
    }

    pub fn nth_rank(n: u8) -> BitBoard {
        assert!(n < 8);
        let mut board = BitBoard(0);
        let first = n * 8;
        let last = 7 + n * 8;
        for square in first..=last {
            board = board | BitBoard::from_square_number(square);
        }
        board
    }

    pub fn nth_file(n: u8) -> BitBoard {
        assert!(n < 8);
        let mut board = BitBoard(0);
        for rank in 0..=7 {
            board = board | BitBoard::from_square_number(rank * 8 + n);
        }
        board
    }

    pub fn significant_for_rook_in(square: Square) -> BitBoard {
        let mut edges = BitBoard(0);
        if square.rank() != 0 {
            edges = edges | BitBoard::nth_rank(0);
        }
        if square.rank() != 7 {
            edges = edges | BitBoard::nth_rank(7);
        }
        if square.file() != 0 {
            edges = edges | BitBoard::nth_file(0);
        }
        if square.file() != 7 {
            edges = edges | BitBoard::nth_file(7);
        }
        dbg!(&edges);

        (BitBoard::nth_rank(square.rank()) ^ BitBoard::nth_file(square.file())) & !edges
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn e4_is_4th_rank() {
        assert_eq!(Square(28).rank(), 4);
    }

    #[test]
    fn e1_is_first_rank() {
        assert_eq!(Square(4).rank(), 1);
    }

    #[test]
    fn e4_is_4th_file() {
        assert_eq!(Square(28).file(), 4);
    }

    #[test]
    fn f4_is_3rd_file() {
        assert_eq!(Square(27).file(), 3);
    }
}
