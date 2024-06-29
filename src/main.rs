use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
struct Pos{ x: u8, y: u8 }

impl Pos {
    fn new(x: u8, y: u8) -> Pos {
        Pos { x, y }
    }

    fn from_1d(num: u8, dim: u8) -> Pos {
        Pos { x: num / dim, y: num % dim }
    }

    fn to_1d(&self, dim: u8) -> u8 {
        self.x * dim + self.y
    }
}

enum EnemyType {
    Spider,
    Skeleton
}

struct Enemy {
    enemy_type: EnemyType,
    health: u8,
    range: u8,
    movement: u8,
    attack: u8,
    defense: u8
}

struct Player {
    health: u8,
    range: u8,
    movement: u8,
    attack: u8,
    defense: u8,
    range_mod: u8,
    movement_mod: u8,
    attack_mod: u8,
    defense_mod: u8,
}

/// Level origin on top left, positions (X,Y) where X and Y increase as horizontal (left to right)
/// and vertical (top to bottom) distance to the origin increases.
struct Level<const DIM: u8> {
    enemies: Vec<Enemy>,
    enemy_pos: Vec<Pos>,
    player: Player,
    player_pos: Pos,
    obstacles: Vec<Pos>
}

/// Given a level a starting position and a destination and a maximum distance, check if the
/// destination is reacheable from the start within the distance. The starting position wont be
/// checked for obstacles.
fn reachable_in<const DIM: u8>(level: &Level<DIM>, starting_pos: Pos, destination_pos: Pos, max_distance: u8) -> bool {
    let mut visited = vec![false; (DIM * DIM).into()];
    let mut check_neighbors: VecDeque<(Pos, u8)> = VecDeque::new();
    visited[starting_pos.to_1d(DIM) as usize] = true;
    check_neighbors.push_back((starting_pos, 0));

    while !check_neighbors.is_empty() {
        // unwrap is safe because of while condition above.
        let root = check_neighbors.pop_front().unwrap();
        if root.1 <= max_distance {
            if root.0 == destination_pos {
                return true;
            }
            let root_pos_1d = root.0.to_1d(DIM);
            let neighbors = [1, DIM].iter()
                .map(|x| vec![root_pos_1d.checked_sub(*x), root_pos_1d.checked_add(*x)])
                .flatten().flatten()
                .filter(|x| { 
                    // Check x not visited, inside the grid and that it didn't wrap from neighboring column e.g. on a 5x5 grid converted to 1d, 5 is not next to 4.
                    if *x < DIM * DIM && !visited[*x as usize] && (root_pos_1d % DIM != 0 || *x % DIM != DIM - 1) && (root_pos_1d % DIM != DIM - 1 || *x % DIM != 0) {
                        visited[*x as usize] = true;
                        true
                    } else {
                        false
                    }})
                .map(|x| Pos::from_1d(x, DIM))
                .filter(|x| !level.enemy_pos.contains(x) && !level.obstacles.contains(x) && level.player_pos != *x);

            for neighbor in neighbors {
                check_neighbors.push_back((neighbor, root.1 + 1));
            }
        }
    }
    false
}

fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_player() -> Player {
        Player {
            health: 0,
            range: 0,
            movement: 0,
            attack: 0,
            defense: 0,
            range_mod: 0,
            movement_mod: 0,
            attack_mod: 0,
            defense_mod: 0
        }
    }

    #[test]
    fn same_pos() {
        let level = Level::<5> {
            enemies: Vec::new(),
            enemy_pos: Vec::new(),
            player: default_player(),
            player_pos: Pos{ x: 5, y: 5 },
            obstacles: Vec::new()
        };

        assert!(reachable_in(&level, Pos::new(0,0), Pos::new(0, 0), 0));
    }

    #[test]
    fn unreachable_pos_unobstructed() {
        let level = Level::<5> {
            enemies: Vec::new(),
            enemy_pos: Vec::new(),
            player: default_player(),
            player_pos: Pos{ x: 5, y: 5 },
            obstacles: Vec::new()
        };

        assert!(!reachable_in(&level, Pos::new(0,0), Pos::new(4, 4), 3));
        assert!(reachable_in(&level, Pos::new(0,0), Pos::new(4, 4), 8));
    }

    #[test]
    /// |S| | | | |
    /// | | | | | |
    /// | | | |▒| |
    /// | | |▒|F| |
    /// | | | | | |P
    fn reachable_in_with_obstruction() {
        let level = Level::<5> {
            enemies: Vec::new(),
            enemy_pos: Vec::new(),
            player: default_player(),
            player_pos: Pos{ x: 5, y: 4 },
            obstacles: vec![Pos::new(3,2), Pos::new(2,3)]
        };

        assert!(!reachable_in(&level, Pos::new(0,0), Pos::new(3, 3), 7));
        assert!(reachable_in(&level, Pos::new(0,0), Pos::new(3, 3), 8));
    }

    #[test]
    /// |S| | | |
    /// | | | | |
    /// | | | | |
    /// | | |E|F|P
    fn reachable_in_single_obstruction() {
        let level = Level::<4> {
            enemies: Vec::new(),
            enemy_pos: vec![Pos::new(2,3)],
            player: default_player(),
            player_pos: Pos{ x: 4, y: 3 },
            obstacles: Vec::new()
        };

        assert!(reachable_in(&level, Pos::new(0,0), Pos::new(3, 3), 6));
    }

    #[test]
    /// |S| | | |
    /// | | | | |
    /// | | | |E|
    /// | | |▒|F|P
    fn blocked_pos_corner() {
        let level = Level::<4> {
            enemies: Vec::new(),
            enemy_pos: vec![Pos::new(3,2)],
            player: default_player(),
            player_pos: Pos{ x: 4, y: 3 },
            obstacles: vec![Pos::new(2,3)]
        };

        assert!(!reachable_in(&level, Pos::new(0,0), Pos::new(3, 3), 100));
    }

    #[test]
    /// |S| | | | | |
    /// | | | | | | |
    /// | | | |E| | |
    /// | | |▒|F|E| |
    /// | | | |▒| | |
    /// | | | | | | |
    fn blocked_pos_surrounded() {
        let level = Level::<6> {
            enemies: Vec::new(),
            enemy_pos: vec![Pos::new(3,2), Pos::new(4,3)],
            player: default_player(),
            player_pos: Pos{ x: 4, y: 3 },
            obstacles: vec![Pos::new(2,3), Pos::new(3,4)]
        };

        assert!(!reachable_in(&level, Pos::new(0,0), Pos::new(3, 3), 100));
    }
}

