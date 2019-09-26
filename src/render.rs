use super::types::*;
use std::fmt;

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Figure::King => write!(f, "K"),
            Figure::Queen => write!(f, "Q"),
            Figure::Rook => write!(f, "R"),
            Figure::Bishop => write!(f, "B"),
            Figure::Knight => write!(f, "N"),
            Figure::Pawn => Ok(()),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::R1 => write!(f, "1"),
            Rank::R2 => write!(f, "2"),
            Rank::R3 => write!(f, "3"),
            Rank::R4 => write!(f, "4"),
            Rank::R5 => write!(f, "5"),
            Rank::R6 => write!(f, "6"),
            Rank::R7 => write!(f, "7"),
            Rank::R8 => write!(f, "8"),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl fmt::Display for Departure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Departure::File(file) => write!(f, "{}", file),
            Departure::Rank(r) => write!(f, "{}", r),
            Departure::Square(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for HalfMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HalfMove::HalfMove(d) => {
                write!(f, "{}", d.figure)?;
                match d.departure {
                    None => (),
                    Some(ref dep) => {
                        write!(f, "{}", dep)?;
                    }
                };
                if d.capture {
                    write!(f, "x")?;
                }
                write!(f, "{}", d.arrival)?;
                if d.check {
                    write!(f, "+")?;
                }
                if d.en_passant {
                    write!(f, " e.p.")?;
                }
                if d.draw_offer {
                    write!(f, " (=)")?;
                }

                Ok(())
            }
            HalfMove::KingsideCastling => write!(f, "0-0"),
            HalfMove::QueensideCastling => write!(f, "0-0-0"),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.white, self.black)
    }
}

fn render_score_by_color(color: &Color) -> String {
    match color {
        Color::White => "1-0".to_string(),
        Color::Black => "0-1".to_string(),
    }
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Result::Draw => write!(f, " 1/2-1/2"),
            Result::Checkmate(color) => {
                // with a checkmate marker
                write!(f, "# {}", render_score_by_color(color))
            }
            Result::Win(color) => write!(f, " {}", render_score_by_color(color)),
            Result::Unknown => Ok(()),
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, chess_move) in self.moves.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}. {}", i + 1, chess_move)?;
        }
        if let Some(half_move) = &self.last_half_move {
            let n = self.moves.len() + 1;
            write!(f, " {}. {}", n, half_move)?;
        }
        write!(f, "{}", self.result)?;
        Ok(())
    }
}
