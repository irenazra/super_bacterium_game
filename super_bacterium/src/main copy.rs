use rand::{thread_rng, Rng};
use std::io;
use std::io::Write;
use std::collections::HashSet;


#[derive (Clone)]
struct Map {
    pub board: Vec<Vec<i32>>
}

struct Game_State {
    pub previous_board_list:Vec<Vec<Vec<i32>>>,
    pub previous_player_state: Vec<Player_State>
}

// 0 is empty 
// 1 is the player
// 2 is enemy
// 3 is food
// 4 exit
impl Map {
    fn print_board(&self) {
        let width = self.board[0].len();
        let row_line = "-".repeat((width * 7));

        self.clone().board.into_iter().for_each(|it| {
            let mut row_string:String = "|".to_string();
            it.into_iter().for_each(|val| {
                let message = match val {
                    0 => "      |",
                    1 => " ME   |",
                    2 => "Danger|",
                    3 => "Food  |",
                    4 => "Exit  |",
                    _ => "Error |" //Error
                };
                row_string.push_str(&(message));
            });

            println!("{}", row_line);
            println!("{}", row_string);
            
            
        });
        println!("{}", row_line);
        
    }

    fn move_object(& mut self, new_row:i32, new_col:i32, row:i32, col:i32) {
        let object = self.board[row as usize][col as usize];

        self.board[new_row as usize][new_col as usize] = object;
        self.board[row as usize][col as usize] = 0 ;

    }

    fn is_valid_position(&self, row:i32, col:i32) -> bool{
        (row >= 0) & (row < self.board.len() as i32) & (col >= 0) & (col < self.board[0].len() as i32)
    }

    fn place_elements(& mut self, enemy_number : usize, food_number: usize,largest_row :usize , largest_col :usize ) {
        self.board[0][0] = 1;
        self.board[largest_row][largest_col] = 4;
        let mut filled_positions = HashSet::new();
        filled_positions.insert((0,0));
        filled_positions.insert((largest_row,largest_col));
    
        let mut enemy_counter = enemy_number;
    
        while enemy_counter > 0 {
            println!("{}", "Enemy while loop");
            let random_row : usize = thread_rng().gen_range(0, self.board[0].len());
            let random_col : usize = thread_rng().gen_range(0, self.board[0].len());
    
            if !filled_positions.contains(&(random_row,random_col)) {
                filled_positions.insert((random_row,random_col));
                self.board[random_row][random_col] = 2;
                enemy_counter -= 1;
            }
    
        }
    
        let mut food_counter = food_number;
    
        while food_counter > 0 {
            println!("{}", "Food while loop");
            let r_row : usize = thread_rng().gen_range(0, self.board[0].len());
            let r_col : usize = thread_rng().gen_range(0, self.board[0].len());
    
            if !filled_positions.contains(&(r_row,r_col)) {
                filled_positions.insert((r_row,r_col));
                self.board[r_row][r_col] = 3;
                food_counter -= 1;
            }
    
        }
    
    }

    fn update_board(& mut self, b :Vec<Vec<i32>> ) {
        self.board = b;
    }



    //fn record_state()

    //fn read_latest_state()

}

#[derive (Clone)]
struct Player_State {
    pub DNA_damage: usize,
    pub energy: usize,
    //pub size: usize,
    pub row: i32,
    pub col: i32
}


fn main() {
    let mut player = Player_State{DNA_damage:0, energy: 10, row:0, col:0};
    let mut map = Map{board: vec![(vec![0;5]);5]};


    println!("{}", "Before");
    map.place_elements(5,5,4,4);
    println!("{}", "After");


    let mut game_state = Game_State{previous_board_list:vec![map.board.clone()],previous_player_state:vec![player.clone()]};


    let mut input = String::new();

    println!("{}", "STATS");
    println!("Energy : {}", player.energy);
    println!("Damage :  {}", player.DNA_damage);




    while (player.energy > 0) & (player.DNA_damage < 5) {

        player.energy -= 1;
        input.clear();

        map.print_board();

        println!("Where should we move? Up, down, left, right?");
        print!("> ");
        // Flush so that the prompt is definitely printed
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        //Turn all input into uppercase to make sure multiple different ways to writing answers will be accepted
        let mut input = input.to_uppercase();

        let mut new_row:i32 = player.row;
        let mut new_col:i32 = player.col;

        
        match input.as_str() {
            "UP" => new_row = new_row - 1,
            "DOWN" => new_row = new_row + 1,
            "RIGHT" => new_col = new_col + 1,
            "LEFT" => new_col = new_col - 1,

            "UNDO" => {println!("{}", "Trying to rewind!");
 
                        },

            _ => panic!("Something went wrong!"),
        }

        if map.is_valid_position(new_row,new_col) {
            let former_occupant = map.board[new_row as usize][new_col as usize];
            map.move_object(new_row,new_col,player.row, player.col,);

            player.row = new_row;
            player.col = new_col;

            // 0 is empty 
            // 2 is enemy
            // 3 is food
            // 4 exit
            match former_occupant {
                0 => { println!("{}", "This space was empty :)");}
                2 => { println!("{}", "Oh no! There was enemy there!");
                        player.DNA_damage += 1;}
                3 => {println!("{}", "Yay, we found food!");
                        player.energy += 1;}
                4 => {println!("{}", "YOU WON!");
                        map.print_board();
                        break;}
                _ => { println!("{}", "Something went wrong :)");}

            }

        } else {
            println!("{}", "This is not a valid move!");
        }

        println!("{}", "");
        println!("{}", "STATS");
        println!("Energy : {}", player.energy);
        println!("Damage :  {}", player.DNA_damage);

        if (player.energy == 0) {
            println!("{}", "You starved: YOU LOST");
        } 

        if (player.DNA_damage == 5) {
            println!("{}", "You received way too much damage: YOU LOST");
        }





        


    }





}
