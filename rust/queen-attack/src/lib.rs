use std::collections::HashSet;
use std::slice::from_ref;
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

fn is_in_chess_board(rank: i32, file: i32) -> bool {
    (0..8).contains(&rank) && (0..8).contains(&file)
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if is_in_chess_board(rank, file) {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let self_attacked_area = Queen::calc_queen_attacked_area(self);
        let other_attacked_area = Queen::calc_queen_attacked_area(other);
        self_attacked_area.contains(&other.0) && other_attacked_area.contains(&self.0)
    }

    fn calc_queen_attacked_area(queen: &Queen) -> HashSet<ChessPosition> {
        let mut attacked_area: HashSet<ChessPosition> = HashSet::new();
        attacked_area.insert(queen.0.clone());
        let directions = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for direction in &directions {
            let positions: HashSet<ChessPosition> = from_ref(direction)
                .iter()
                .cycle()
                .zip(1..=8)
                .map(|((dx, dy), cnt)| {
                    ChessPosition::new(queen.0.rank + dx * cnt, queen.0.file + dy * cnt)
                })
                .take_while(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect();
            dbg!(queen, &positions);
            attacked_area = attacked_area.union(&positions).cloned().collect();
        }
        attacked_area
    }
}
