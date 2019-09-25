mod types {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Color {
        White,
        Black,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Figure {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum File {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Rank {
        R1,
        R2,
        R3,
        R4,
        R5,
        R6,
        R7,
        R8,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum HalfMove {
        HalfMove(MoveDetails),
        KingsideCastling,
        QueensideCastling,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct MoveDetails {
        pub figure: Figure,
        pub departure: Option<Departure>,
        pub capture: bool,
        pub arrival: Square,
        pub check: bool,
        pub en_passant: bool,
        pub draw_offer: bool,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Square {
        pub file: File,
        pub rank: Rank,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Departure {
        File(File),
        Rank(Rank),
        Square(Square),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Move {
        pub white: HalfMove,
        pub black: HalfMove,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Result {
        Checkmate(Color),
        Win(Color),
        Draw,
        Unknown,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Game {
        pub moves: Vec<Move>,
        pub last_half_move: Option<HalfMove>,
        pub result: Result,
    }

    pub struct MoveBuilder(MoveDetails);

    pub fn move_figure_to(figure: Figure, file: File, rank: Rank) -> MoveBuilder {
        MoveBuilder(MoveDetails {
            figure,
            arrival: Square { file, rank },
            departure: None,
            capture: false,
            check: false,
            en_passant: false,
            draw_offer: false,
        })
    }

    impl MoveBuilder {
        pub fn with_departure_file(mut self, file: File) -> Self {
            self.0.departure = Some(Departure::File(file));
            self
        }

        pub fn with_departure_rank(mut self, rank: Rank) -> Self {
            self.0.departure = Some(Departure::Rank(rank));
            self
        }

        pub fn with_departure_square(mut self, file: File, rank: Rank) -> Self {
            self.0.departure = Some(Departure::Square(Square { file, rank }));
            self
        }

        pub fn capture(mut self) -> Self {
            self.0.capture = true;
            self
        }

        pub fn en_passant(mut self) -> Self {
            self.0.en_passant = true;
            self
        }

        pub fn check(mut self) -> Self {
            self.0.check = true;
            self
        }

        pub fn draw_offer(mut self) -> Self {
            self.0.draw_offer = true;
            self
        }

        pub fn build(self) -> HalfMove {
            HalfMove::HalfMove(self.0)
        }
    }
}

mod render {
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
}

mod parse {
    use super::types::*;
    use nom::{
        branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*,
        sequence::*, IResult,
    };

    fn parse_figure(i: &str) -> IResult<&str, Figure> {
        alt((
            value(Figure::King, char('K')),
            value(Figure::Queen, char('Q')),
            value(Figure::Rook, char('R')),
            value(Figure::Bishop, char('B')),
            value(Figure::Knight, char('N')),
            |i| Ok((i, Figure::Pawn)),
        ))(i)
    }

    fn parse_file(i: &str) -> IResult<&str, File> {
        alt((
            value(File::A, char('a')),
            value(File::B, char('b')),
            value(File::C, char('c')),
            value(File::D, char('d')),
            value(File::E, char('e')),
            value(File::F, char('f')),
            value(File::G, char('g')),
            value(File::H, char('h')),
        ))(i)
    }

    fn parse_rank(i: &str) -> IResult<&str, Rank> {
        alt((
            value(Rank::R1, char('1')),
            value(Rank::R2, char('2')),
            value(Rank::R3, char('3')),
            value(Rank::R4, char('4')),
            value(Rank::R5, char('5')),
            value(Rank::R6, char('6')),
            value(Rank::R7, char('7')),
            value(Rank::R8, char('8')),
        ))(i)
    }

    fn parse_square(i: &str) -> IResult<&str, Square> {
        let (input, file) = parse_file(i)?;
        let (input, rank) = parse_rank(input)?;
        Ok((input, Square { file, rank }))
    }

    fn parse_departure(i: &str) -> IResult<&str, Departure> {
        alt((
            map(parse_square, Departure::Square),
            map(parse_file, Departure::File),
            map(parse_rank, Departure::Rank),
        ))(i)
    }

    fn parse_opt_char(ch: char, i: &str) -> IResult<&str, bool> {
        map(opt(char(ch)), |opt_ch: Option<char>| opt_ch.is_some())(i)
    }

    fn parse_en_passant(i: &str) -> IResult<&str, bool> {
        map(
            opt(preceded(space1, tag("e.p."))),
            |opt_ep: Option<&str>| opt_ep.is_some(),
        )(i)
    }

    fn parse_draw_offer(i: &str) -> IResult<&str, bool> {
        map(
            opt(preceded(space1, tag("(=)"))),
            |opt_offer: Option<&str>| opt_offer.is_some(),
        )(i)
    }

    fn parse_half_move_with_departure(i: &str) -> IResult<&str, MoveDetails> {
        let (input, figure) = parse_figure(i)?;
        let (input, departure) = map(parse_departure, Some)(input)?;
        let (input, capture) = parse_opt_char('x', input)?;
        let (input, arrival) = parse_square(input)?;
        let (input, check) = parse_opt_char('+', input)?;
        let (input, en_passant) = parse_en_passant(input)?;
        let (input, draw_offer) = parse_draw_offer(input)?;
        Ok((
            input,
            MoveDetails {
                figure,
                departure,
                capture,
                arrival,
                check,
                en_passant,
                draw_offer,
            },
        ))
    }

    fn parse_half_move_without_departure(i: &str) -> IResult<&str, MoveDetails> {
        let (input, figure) = parse_figure(i)?;
        let (input, capture) = parse_opt_char('x', input)?;
        let (input, arrival) = parse_square(input)?;
        let (input, check) = parse_opt_char('+', input)?;
        let (input, en_passant) = parse_en_passant(input)?;
        let (input, draw_offer) = parse_draw_offer(input)?;
        Ok((
            input,
            MoveDetails {
                figure,
                departure: None,
                capture,
                arrival,
                check,
                en_passant,
                draw_offer,
            },
        ))
    }

    fn parse_half_move(i: &str) -> IResult<&str, HalfMove> {
        alt((
            map(parse_half_move_with_departure, |d: MoveDetails| {
                HalfMove::HalfMove(d)
            }),
            map(parse_half_move_without_departure, |d: MoveDetails| {
                HalfMove::HalfMove(d)
            }),
            value(HalfMove::QueensideCastling, tag("0-0-0")),
            value(HalfMove::KingsideCastling, tag("0-0")),
        ))(i)
    }

    // world record is 269 moves but that was using old FIDE rules so 256 should be enough
    fn parse_move_number(i: &str) -> IResult<&str, u8> {
        terminated(map_res(digit1, |s: &str| s.parse::<u8>()), char('.'))(i)
    }

    fn parse_numbered_full_move(i: &str) -> IResult<&str, Move> {
        let (input, _) = parse_move_number(i)?;
        let (input, white) = preceded(space1, parse_half_move)(input)?;
        let (input, black) = preceded(space1, parse_half_move)(input)?;
        Ok((input, Move { white, black }))
    }

    fn parse_game_result(draw_offer: bool, i: &str) -> IResult<&str, Result> {
        if draw_offer {
            // in case of a draw we assume no result as draw offer being accepted
            alt((
                preceded(
                    space1,
                    alt((
                        value(Result::Win(Color::White), tag("1-0")),
                        value(Result::Win(Color::Black), tag("0-1")),
                        value(Result::Draw, tag("1/2-1/2")),
                    )),
                ),
                |i| Ok((i, Result::Draw)),
            ))(i)
        } else {
            preceded(
                space1,
                alt((
                    value(Result::Win(Color::White), tag("1-0")),
                    value(Result::Win(Color::Black), tag("0-1")),
                    value(Result::Draw, tag("1/2-1/2")),
                )),
            )(i)
        }
    }

    fn parse_black_checkmate(i: &str) -> IResult<&str, Result> {
        // checkmate mark after a full move
        let (input, _) = char('#')(i)?;
        // optional game result
        let (input, _) = opt(preceded(space1, tag("0-1")))(input)?;
        Ok((input, Result::Checkmate(Color::Black)))
    }

    fn parse_black_end_game(draw_offer: bool, i: &str) -> IResult<&str, Result> {
        alt((parse_black_checkmate, |i| parse_game_result(draw_offer, i)))(i)
    }

    fn parse_white_checkmate(i: &str) -> IResult<&str, Result> {
        // checkmate mark after a half move
        let (input, _) = char('#')(i)?;
        // optional game result
        let (input, _) = opt(preceded(space1, tag("1-0")))(input)?;
        Ok((input, Result::Checkmate(Color::White)))
    }

    fn parse_white_end_game(i: &str) -> IResult<&str, (HalfMove, Result)> {
        let (input, _) = parse_move_number(i)?;
        let (input, white) = preceded(space1, parse_half_move)(input)?;
        let draw_offer = match white {
            HalfMove::HalfMove(ref details) => details.draw_offer,
            _ => false,
        };
        map(
            alt((parse_white_checkmate, move |i| {
                parse_game_result(draw_offer, i)
            })),
            move |result| (white.clone(), result),
        )(input)
    }

    pub fn parse_game(i: &str) -> IResult<&str, Game> {
        let (input, moves) = separated_list(space1, parse_numbered_full_move)(i)?;
        // as a simplification we assume no draw offer after castling
        let black_draw_offer = match moves.last() {
            Some(Move {
                black: HalfMove::HalfMove(ref details),
                ..
            }) => details.draw_offer,
            _ => false,
        };
        let (input, (last_half_move, result)) = alt((
            map(
                |i| parse_black_end_game(black_draw_offer, i),
                |result| (None, result),
            ),
            // white player does only a half move
            map(
                preceded(space1, parse_white_end_game),
                |(white_move, result)| (Some(white_move), result),
            ),
        ))(input)?;
        Ok((
            input,
            Game {
                moves,
                last_half_move,
                result,
            },
        ))
    }

    #[cfg(test)]
    mod tests {
        use super::super::render::*;
        use super::*;

        #[test]
        fn test_parse_half_moves() {
            assert_eq!(
                parse_half_move("e4"),
                Ok(("", move_figure_to(Figure::Pawn, File::E, Rank::R4).build()))
            );
            assert_eq!(
                parse_half_move("dxe5"),
                Ok((
                    "",
                    move_figure_to(Figure::Pawn, File::E, Rank::R5)
                        .with_departure_file(File::D)
                        .capture()
                        .build()
                ))
            );
            assert_eq!(
                parse_half_move("N5f3"),
                Ok((
                    "",
                    move_figure_to(Figure::Knight, File::F, Rank::R3)
                        .with_departure_rank(Rank::R5)
                        .build()
                ))
            );
            assert_eq!(
                parse_half_move("Bb2e5"),
                Ok((
                    "",
                    move_figure_to(Figure::Bishop, File::E, Rank::R5)
                        .with_departure_square(File::B, Rank::R2)
                        .build()
                ))
            );
            assert_eq!(
                parse_half_move("Bxb5+"),
                Ok((
                    "",
                    move_figure_to(Figure::Bishop, File::B, Rank::R5)
                        .capture()
                        .check()
                        .build()
                ))
            );
            assert_eq!(parse_half_move("0-0"), Ok(("", HalfMove::KingsideCastling)));
        }

        // 3 game notations from the FIDE handbook
        #[test]
        fn test_parse_game1() {
            let game = "1. e4 0-0 2. Nf3 Nf6 3. d4 exd4 4. e5 Ne4 5. Qxd4 d5 \
                        6. exd6 e.p. Nxd6 7. Bg5 Nc6 8. Qe3+ Be7 9. Nbd2 0-0 \
                        10. 0-0-0 Re8 11. Kb1 (=) 1/2-1/2";
            let (_, parsed) = parse_game(game).unwrap();
            assert_eq!(render_game(parsed), game);
        }

        #[test]
        fn test_parse_game2() {
            let game = "1. e4 e5 2. Nf3 Nf6 3. d4 ed4 4. e5 Ne4 5. Qd4 d5 \
                        6. ed6 Nd6 7. Bg5 Nc6 8. Qe3 Be7 9. Nbd2 0-0 10. 0-0-0 Re8 \
                        11. Kb1 (=) 1/2-1/2";
            let (_, parsed) = parse_game(game).unwrap();
            assert_eq!(render_game(parsed), game);
        }

        #[test]
        fn test_parse_game3() {
            let game = "1. e2e4 e7e5 2. Ng1f3 Ng8f6 3. d2d4 e5xd4 4. e4e5 Nf6e4 \
                        5. Qd1xd4 d7d5 6. e5xd6 e.p. Ne4xd6 7. Bc1g5 Nb8c6 \
                        8. Qd4d3 Bf8e7 9. Nb1d2 0-0 10. 0-0-0 Rf8e8 11. Kb1 (=)";
            let (_, parsed) = parse_game(game).unwrap();
            assert_eq!(
                render_game(parsed),
                // game result is optional when parsing but not so in render
                [game, "1/2-1/2"].join(" ")
            );
        }

        #[test]
        fn test_parse_fischer_spassky() {
            // 1992.11.04, Belgrade, 29th round
            let game = "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 4. Ba4 Nf6 5. 0-0 Be7 6. Re1 b5 7. Bb3 d6 \
                        8. c3 0-0 9. h3 Nb8 10. d4 Nbd7 11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 \
                        14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5 Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 \
                        20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6 23. Ne5 Rae8 24. Bxf7+ Rxf7 \
                        25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5 hxg5 29. b3 Ke6 \
                        30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5 35. Ra7 g6 \
                        36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6 Nf2 \
                        42. g4 Bd3 43. Re6 1/2-1/2";
            let (_, parsed) = parse_game(game).unwrap();
            assert_eq!(render_game(parsed), game);
        }

        #[test]
        fn test_parse_morphy_game_1858() {
            // This sample chess game was played between Paul Morphy and his two opponents,
            // the Duke of Brunswick and Count Isouard, in 1858 during a performance of
            // The Barber of Seville at the Paris Opera.
            let game =
                "1. e4 e5 2. Nf3 d6 3. d4 Bg4 4. d4xe5 Bxf3 5. Qxf3 d6xe5 6. Bc4 Nf6 \
                 7. Qb3 Qe7 8. Nc3 c6 9. Bg5 b5 10. Nxb5 c6xb5 11. Bxb5+ Nd7 \
                 12. 0-0-0 Rd8 13. Rxd7 Rxd7 14. Rd1 Qe6 15. Bxd7+ Nxd7 16. Qb8+ Nxb8 17. Rd8#";
            let (_, parsed) = parse_game(game).unwrap();
            assert_eq!(
                render_game(parsed),
                // game result is optional when parsing but not so in render
                [game, "1-0"].join(" ")
            );
        }
    }
}

pub use self::parse::parse_game;
pub use self::render::render_game;
pub use self::types::*;
