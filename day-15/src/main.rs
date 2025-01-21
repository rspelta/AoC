use std::fs::File;
use std::io::{self, BufRead};

fn load_map_from_file(file_path: &str, n: usize) -> io::Result<Vec<Vec<char>>> {
    // Apri il file in modalità lettura
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Leggi solo le prime `n` righe e costruisci un array bidimensionale
    let char_map: Vec<Vec<char>> = reader
        .lines() // Itera sulle righe del file
        .take(n) // Prendi solo le prime `n` righe
        .filter_map(|line| line.ok()) // Ignora eventuali errori di lettura
        .map(|line| line.chars().collect()) // Converte ogni riga in un vettore di caratteri
        .collect();

    Ok(char_map)
}

fn extract_directions_from_file(file_path: &str) -> io::Result<Vec<char>> {
    // Apri il file in modalità lettura
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Simboli da estrarre
    let valid_symbols = ['<', '^', '>', 'v'];

    // Leggi il file riga per riga e filtra solo i simboli validi
    let directions: Vec<char> = reader
        .lines() // Itera sulle righe del file
        .filter_map(|line| line.ok()) // Ignora eventuali errori di lettura
        .flat_map(|line| line.chars().collect::<Vec<char>>()) // Converte ogni riga in un Vec<char>
        .filter(|&c| valid_symbols.contains(&c)) // Filtra i simboli validi
        .collect(); // Colleziona i simboli validi in un vettore

    Ok(directions)
}

fn find_at_coordinates(char_map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut coordinates = Vec::new();

    // Itera su ogni riga e colonna dell'array bidimensionale
    for (y, row) in char_map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '@' {
                coordinates.push((y, x)); // Aggiungi le coordinate se trovi "@"
            }
        }
    }

    coordinates
}

fn move_robot_on_map(
    char_map: &mut Vec<Vec<char>>,
    robot_pos: &mut (usize, usize),
    moves: &str,
) {
    // Definiamo i movimenti possibili con (direzione, delta_y, delta_x)
    let directions = vec![('^', -1, 0), ('v', 1, 0), ('<', 0, -1), ('>', 0, 1)];

    for mv in moves.chars() {
        if let Some(&(_, dy, dx)) = directions.iter().find(|&&(d, _, _)| d == mv) {
            let new_y = (robot_pos.0 as isize + dy) as usize;
            let new_x = (robot_pos.1 as isize + dx) as usize;

            // Controlla se la nuova posizione è dentro i limiti della mappa
            if new_y >= char_map.len() || new_x >= char_map[0].len() {
                continue;
            }

            // Se c'è un muro, il robot non si muove
            if char_map[new_y][new_x] == '#' {
                continue;
            }

            // Se il robot incontra un blocco 'O', controlla se può spingerlo a catena
            if char_map[new_y][new_x] == 'O' {
                let mut block_positions = vec![(new_y, new_x)];

                // Identifica tutti i blocchi consecutivi nella direzione del movimento
                while let Some(&(last_y, last_x)) = block_positions.last        /* Stampa la mappa dopo ogni mossa (opzionale)
                println!("Mossa '{}':", mv);
                for row in char_map.iter() {
                    println!("{}", row.iter().collect::<String>());
                }
                println!();*/() {
                    let next_y = (last_y as isize + dy) as usize;
                    let next_x = (last_x as isize + dx) as usize;

                    // Se troviamo un altro blocco, aggiungilo alla catena
                    if next_y < char_map.len()
                        && next_x < char_map[0].len()
                        && char_map[next_y][next_x] == 'O'
                    {
                        block_positions.push((next_y, next_x));
                    } else {
                        break;
                    }
                }

                // Controlla se c'è spazio per spostare tutti i blocchi
                let next_y = (block_positions.last().unwrap().0 as isize + dy) as usize;
                let next_x = (block_positions.last().unwrap().1 as isize + dx) as usize;

                if next_y < char_map.len()
                    && next_x < char_map[0].len()
                    && char_map[next_y][next_x] == '.'
                {
                    // Sposta i blocchi a catena
                    for &(y, x) in block_positions.iter().rev() {
                        let next_y = (y as isize + dy) as usize;
                        let next_x = (x as isize + dx) as usize;
                        char_map[next_y][next_x] = 'O';
                        char_map[y][x] = '.';
                    }

                    // Sposta il robot nella posizione del primo blocco
                    char_map[new_y][new_x] = '@';
                    char_map[robot_pos.0][robot_pos.1] = '.';
                    *robot_pos = (new_y, new_x);
                }
            } else {
                // Sposta il robot su una cella vuota
                char_map[new_y][new_x] = '@';
                char_map[robot_pos.0][robot_pos.1] = '.';
                *robot_pos = (new_y, new_x);
            }
        }
    }
}

fn sum_o_coordinates(char_map: &[Vec<char>]) -> u32 {
    let mut sum = 0;

    for (y, row) in char_map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                // Calcola la coordinata usando la formula e aggiungila alla somma
                sum += 100 * (y as u32) + (x as u32);
            }
        }
    }

    sum
}

fn main() -> io::Result<()> {
    // Percorso del file contenente la mappa
    let file_path = "./src/input.txt";

    // Numero di righe da leggere
    let n = 50;

    // Carica la mappa
    let mut char_map = load_map_from_file(file_path, n)?;

    // Estrai i simboli di direzione dal file
    let directions = extract_directions_from_file(file_path)?;

    // Stampa l'intera mappa
    for row in &char_map {
        println!("{}", row.iter().collect::<String>());
    }
    println!("Simboli di direzione trovati: {:?}", directions);

    let mut coordinates = find_at_coordinates(&char_map);
    println!("Coordinate trovate: {:?}", coordinates);

    let moves = directions.iter().collect::<String>();
    move_robot_on_map(&mut char_map, &mut coordinates[0], &moves);

    println!("END:");
    for row in char_map.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!("sum: {}", sum_o_coordinates(&char_map));

    Ok(())
}