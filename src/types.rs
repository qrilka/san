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
