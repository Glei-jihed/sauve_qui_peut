use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

/// Position dans le laby
pub type Position = (usize, usize);


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Retourne la direction après avoir tourner à gauche (pour pas être désorienté)
    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West  => Direction::South,
            Direction::South => Direction::East,
            Direction::East  => Direction::North,
        }
    }

    /// pareil mais après un tour à droite.
    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North,
        }
    }

    /// Donne le décalage ligne/colonne correspondant à la direction.
    pub fn as_offset(self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East  => (0, 1),
            Direction::West  => (0, -1),
        }
    }
}


fn can_move(grid: &Vec<Vec<bool>>, pos: Position) -> bool {
    let (i, j) = pos;
    if i < grid.len() && j < grid[0].len() {
        grid[i][j]
    } else {
        false
    }
}

/// On sors du labyrinthe avec l'algo de Pledge.
/// * `grid` : grille du labyrinthe (true = passage, false = mur)
/// * `start` : position de départ
/// * `exit` : position de sortie
/// * `goal` : direction souhaitée (ex. la direction approximative vers la sortie)
/// Retourne un vecteur de positions formant le chemin s'il est trouvé.
pub fn solve_maze_pledge(
    grid: &Vec<Vec<bool>>,
    start: Position,
    exit: Position,
    goal: Direction,
) -> Option<Vec<Position>> {
    // Variables d'état de l'algorithme.
    let mut current = start;
    let mut current_direction = goal;
    let mut pledge_counter: i32 = 0;
    let mut in_pledge = false;
    let mut path = vec![start];

    // Limite le nombre d'itérations pour éviter les boucles infini
    let max_steps = grid.len() * grid[0].len() * 20;
    
    for _ in 0..max_steps {
        if current == exit {
            return Some(path);
        }
        
        if !in_pledge {
            // Essayer d'avancer dans la direction "goal" (la sortie)
            let (di, dj) = goal.as_offset();
            let new_i = current.0 as i32 + di;
            let new_j = current.1 as i32 + dj;
            if new_i >= 0 && new_j >= 0 {
                let new_pos = (new_i as usize, new_j as usize);
                if can_move(grid, new_pos) {
                    current = new_pos;
                    path.push(current);
                    continue;
                }
            }
            // S'il y a un obstacle, activer le mode Pledge.
            in_pledge = true;
            // On initie la poursuite du mur par un petit virage à droite.
            current_direction = goal.turn_right();
            pledge_counter = 0;
        }
        
        // En mode Pledge, on suit le mur avec la règle de la main droite.
        // 1. Vérifier si l'on peut tourner à droite.
        let right_dir = current_direction.turn_right();
        let (di, dj) = right_dir.as_offset();
        let pos_right = (
            current.0 as i32 + di,
            current.1 as i32 + dj,
        );
        let can_right = pos_right.0 >= 0
            && pos_right.1 >= 0
            && can_move(grid, (pos_right.0 as usize, pos_right.1 as usize));

        if can_right {
            current_direction = right_dir;
            pledge_counter += 1;
            let (di, dj) = current_direction.as_offset();
            current = ((current.0 as i32 + di) as usize, (current.1 as i32 + dj) as usize);
            path.push(current);
        } else {
            // Sinon, si on peut avancer dans la direction actuelle.
            let (di, dj) = current_direction.as_offset();
            let pos_forward = (
                current.0 as i32 + di,
                current.1 as i32 + dj,
            );
            let can_forward = pos_forward.0 >= 0
                && pos_forward.1 >= 0
                && can_move(grid, (pos_forward.0 as usize, pos_forward.1 as usize));
            if can_forward {
                current = ((current.0 as i32 + di) as usize, (current.1 as i32 + dj) as usize);
                path.push(current);
            } else {
                // Si rien ne va, tourner à gauche.
                current_direction = current_direction.turn_left();
                pledge_counter -= 1;
                // On ne bouge pas pour ce tour, juste la direction change.
            }
        }
        
        // Si le compteur de Pledge est revenu à zéro et que le chemin dans la direction goal est libre,
        // on quitte le mode Pledge.
        if in_pledge && pledge_counter == 0 {
            let (di, dj) = goal.as_offset();
            let pos_goal = (
                current.0 as i32 + di,
                current.1 as i32 + dj,
            );
            if pos_goal.0 >= 0 && pos_goal.1 >= 0 && can_move(grid, (pos_goal.0 as usize, pos_goal.1 as usize)) {
                in_pledge = false;
                current_direction = goal;
            }
        }
    }
    
    None // Aucune solution trouvée dans la limite du nombre d'étapes.
}

/// Génère un fichier  contenant le chemin vers la sortie
/// pour un labyrinthe d'exemple.
pub fn generate_solution_file_pledge() {
    // Exemple de labyrinthe 5x5 (les bords sont des murs).
    let grid = vec![
        vec![false, false, false, false, false],
        vec![false, true,  true,  true,  false],
        vec![false, true,  false, true,  false],
        vec![false, true,  true,  true,  false],
        vec![false, false, false, false, false],
    ];
    let start = (1, 1);
    let exit = (3, 3);
    let goal = Direction::East; 
    
    match solve_maze_pledge(&grid, start, exit, goal) {
        Some(path) => {
            let mut file = File::create("maze_solution_pledge.txt")
                .expect("Impossible de créer maze_solution_pledge.txt");
            writeln!(file, "Chemin solution (algorithme Pledge):")
                .expect("Erreur lors de l'écriture");
            for pos in path {
                writeln!(file, "{:?}", pos).expect("Erreur lors de l'écriture");
            }
            println!("La solution a été écrite dans maze_solution_pledge.txt");
        }
        None => {
            println!("Aucune solution trouvée avec l'algorithme Pledge pour ce labyrinthe.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_solve_maze_pledge() {
        let grid = vec![
            vec![false, false, false, false, false],
            vec![false, true,  true,  true,  false],
            vec![false, true,  false, true,  false],
            vec![false, true,  true,  true,  false],
            vec![false, false, false, false, false],
        ];
        let start = (1, 1);
        let exit = (3, 3);
        let goal = Direction::East;
        let path = solve_maze_pledge(&grid, start, exit, goal);
        assert!(path.is_some());
    }
}
