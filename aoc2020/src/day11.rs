use nom::lib::std::fmt::Formatter;
use core::fmt;

#[derive(Clone, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

type Layout = Vec<Vec<Seat>>;

pub struct State {
    seats: Layout,
    height: usize,
    width: usize,
    changes: usize,
}

#[derive(Copy, Clone)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Depth {
    Inf,
    Next,
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Seat::Floor => write!(f, "."),
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
        }
    }
}

impl Seat {
    fn from(c: char) -> Seat {
        match c {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!("Invalid letter: {}", c)
        }
    }
}

impl State {
    fn new(seats: Layout) -> State {
        let height = seats.len();
        let width = seats[0].len();

        State {
            seats,
            height,
            width,
            changes: 0,
        }
    }

    fn next_seat(&self, pos: Position, depth: Depth) -> Seat {
        let width = self.width as isize;
        let height = self.height as isize;
        let in_bounds = |x, y| x >= 0 && x < width && y >= 0 && y < height;
        let dirs = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];

        let count = dirs
            .iter()
            .fold(0, |acc, (xdir, ydir)| {
                let mut p = Position { x: pos.x + xdir, y: pos.y + ydir };
                while depth == Depth::Inf && in_bounds(p.x, p.y) &&
                    self.seats[p.y as usize][p.x as usize] == Seat::Floor {
                    p = Position { x: p.x + xdir, y: p.y + ydir };
                }

                if in_bounds(p.x, p.y) &&
                    self.seats[p.y as usize][p.x as usize] == Seat::Occupied {
                    acc + 1
                } else {
                    acc
                }
            });

        let cond = match depth {
            Depth::Inf => 5,
            Depth::Next => 4
        };
        match &self.seats[pos.y as usize][pos.x as usize] {
            Seat::Empty if count == 0 => Seat::Occupied,
            Seat::Occupied if count >= cond => Seat::Empty,
            seat => seat.clone()
        }
    }

    fn update(&self, depth: Depth) -> State {
        let mut seats: Layout = self.seats.to_vec();
        let changes =
            (0..self.height).fold(0, |acc, y|
                (0..self.width).fold(0, |acc, x| {
                    let p = Position { x: x as isize, y: y as isize };
                    let seat = self.next_seat(p, depth);
                    let change = if seat != seats[y][x] { 1 } else { 0 };
                    seats[y][x] = seat;
                    acc + change
                }) + acc,
            );

        State {
            seats,
            changes,
            ..*self
        }
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter().fold(0, |acc, x|
            acc + x.iter().fold(0, |acc, y|
                acc + if *y == Seat::Occupied { 1 } else { 0 }))
    }
}

#[aoc_generator(day11)]
pub fn gen(input: &str) -> State {
    let seats: Layout = input
        .lines()
        .map(|line| line
            .trim()
            .chars()
            .map(Seat::from)
            .collect()
        )
        .collect();

    State::new(seats)
}

fn rec(state: &State, depth: Depth) -> usize {
    let state = state.update(depth);
    if state.changes == 0 {
        state.count_occupied()
    } else {
        rec(&state, depth)
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(state: &State) -> usize {
    rec(state, Depth::Next)
}

#[aoc(day11, part2)]
pub fn solve_part2(state: &State) -> usize {
    rec(state, Depth::Inf)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    }

    #[test]
    fn test_gen() {
        let s = gen(get_input());
        for v in s.seats {
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&gen(get_input())), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&gen(get_input())), 26);
    }
}