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
                let departure = match d.departure {
                    None => "".to_string(),
                    Some(ref dep) => format!("{}", dep),
                };
                write!(
                    f,
                    "{}{}{}{}{}",
                    d.figure,
                    departure,
                    if d.capture { "x" } else { "" },
                    d.arrival,
                    if d.check { "+" } else { "" }
                )
                .unwrap();
                if d.en_passant {
                    write!(f, " e.p.").unwrap()
                };
                if d.draw_offer {
                    write!(f, " (=)")
                } else {
                    Ok(())
                }
            }
            HalfMove::KingsideCastling => write!(f, "0-0"),
            HalfMove::QueensideCastling => write!(f, "0-0-0"),
        }
    }
}

fn render_move(n: usize, m: &Move) -> String {
    format!("{}. {} {}", n, m.white, m.black)
}

fn score_by_color(color: Color) -> String {
    match color {
        Color::White => "1-0".to_string(),
        Color::Black => "0-1".to_string(),
    }
}

pub fn render_game(game: Game) -> String {
    let mut v: Vec<String> = game
        .moves
        .iter()
        .enumerate()
        .map(|(i, chess_move)| render_move(i + 1, chess_move))
        .collect();
    if let Some(half_move) = game.last_half_move {
        let n = game.moves.len() + 1;
        v.push(format!("{}. {}", n, half_move))
    }
    match game.result {
        Result::Draw => v.push("1/2-1/2".to_string()),
        Result::Checkmate(color) => {
            // a checkmate marker
            if let Some(last) = v.last_mut() {
                last.push('#')
            };
            v.push(score_by_color(color))
        }
        Result::Win(color) => v.push(score_by_color(color)),
        Result::Unknown => {}
    };
    v.join(" ")
}
