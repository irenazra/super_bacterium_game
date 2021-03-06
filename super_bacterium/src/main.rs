use rand::{thread_rng, Rng};
use std::io;
use std::collections::HashSet;
use std::io::{Write};
use std::str::FromStr;
use std::fs;


//Keeps track of the game state and simulates space
#[derive (Clone)]
struct Map {
    pub board: Vec<Vec<i32>>,
    pub records : Vec<Vec<i32>>,
    pub damage: usize,
    pub energy: usize,
    pub row: i32,
    pub col: i32
}

impl Map {

    //Prints the game board
    //This is the main function that simulates the space of the game
    fn print_board(&self) {

        let width = self.board[0].len();
        let row_line = "-".repeat(width * 7);


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

    //Enables an object on the board to be moves somewhere else
    fn move_object(& mut self, new_row:i32, new_col:i32, row:i32, col:i32) {
        let object = self.board[row as usize][col as usize];

        self.board[new_row as usize][new_col as usize] = object;
        self.board[row as usize][col as usize] = 0 ;

    }

    // Determines whether the given position is valid
    fn is_valid_position(&self, row:i32, col:i32) -> bool{
        (row >= 0) & (row < self.board.len() as i32) & (col >= 0) & (col < self.board[0].len() as i32)
    }

    //Randomly place specified amount and kind of elements on the board
    fn place_elements(& mut self, enemy_number : usize, food_number: usize,largest_row :usize , largest_col :usize ) {
        self.board[0][0] = 1;
        self.board[largest_row][largest_col] = 4;

        //Keep track of the filled positions to avoid hits
        let mut filled_positions = HashSet::new();
        filled_positions.insert((0,0));
        filled_positions.insert((largest_row,largest_col));
    
        //Randomly generate positions and place the specified amount of enemies
        let mut enemy_counter = enemy_number;
        while enemy_counter > 0 {
            let random_row : usize = thread_rng().gen_range(0, self.board[0].len());
            let random_col : usize = thread_rng().gen_range(0, self.board[0].len());
    
            if !filled_positions.contains(&(random_row,random_col)) {
                filled_positions.insert((random_row,random_col));
                self.board[random_row][random_col] = 2;
                enemy_counter -= 1;
            }
        }
    
        //Randomly generate positions and place the specified amount of enemies
        let mut food_counter = food_number;
        while food_counter > 0 {
            let r_row : usize = thread_rng().gen_range(0, self.board[0].len());
            let r_col : usize = thread_rng().gen_range(0, self.board[0].len());
    
            if !filled_positions.contains(&(r_row,r_col)) {
                filled_positions.insert((r_row,r_col));
                self.board[r_row][r_col] = 3;
                food_counter -= 1;
            }
        }
    }

    //Record the latest move to the records vector
    fn record_state(& mut self, row: i32, col : i32, the_move : i32, previous_object_num: i32) {
        //At position row,col the_move move was done
        let new_vec = vec![row,col,the_move,previous_object_num];
        self.records.push(new_vec);
    }

    //Undo the latest move in the records vector
    fn rewind_one_move (& mut self) {
        //Because we are rewinding, we need to give the energy back to the player
        self.energy += 1;

        //Information about the last move
        let latest_move = self.records.pop().unwrap();
        let row = latest_move[0];
        let col = latest_move[1];
        let the_move = latest_move[2];
        let prev =latest_move[3];

        //If we encountered an enemy in the last move
        if prev == 2 {
            self.damage -= 1;
        }

        //If we found food in the last move
        if prev == 3 {
            self.energy -= 1;
        }

        //Calculate the previous location of the player
        let mut current_row = row;
        let mut current_col = col;
        match the_move {
            0 => {current_row -= 1;},
            1 => {current_row += 1;},
            2 => {current_col += 1;},
            3 => {current_col -= 1;},
            _ => println!("{}", "Something is wrong!"),
        }

        //Place the player and the previous object at that location at their old positions
        self.board[row as usize][col as usize] = 1;
        self.board[current_row as usize][current_col as usize] = prev;
        self.col = col;
        self.row = row;
    }
}

//Context free grammar which generates messages when the user moves to an empty space
fn cfg_empty_print(){
    // Rules of the Grammar
    // S -> Helper, Positive Helper, Helper Positive
    // Helper -> "This space was empty.", "Empty section!", "Nothing here!", "There is nothing here but me!"
    // Positive -> "Yay!","Go you!", "Yes!", "Perfect!"

    let helper = vec!["This space was empty.", "Empty section!", "Nothing here!", "There is nothing here but me!"];
    let positive = vec!["Yay!","Go you!", "Yes!", "Perfect!"];

    let s : usize = thread_rng().gen_range(0, 3);
    let h: usize =  thread_rng().gen_range(0, 4);
    let p: usize =  thread_rng().gen_range(0, 4);

    let help = helper[h];
    let pos = positive[p];

    if s == 0 {
        print!("{}", help);
    } else if  s == 1{
        let result = format!("{} {}",pos, help);
        print!("{}", result);
    } else {
        let result = format!("{} {}",help, pos);
        print!("{}", result);
    }
}

//Context free grammar which generates messages when the user moves to a space with food
fn cfg_food_print(){
    // Rules of the Grammar
    // S -> Helper, Positive Helper, Helper Positive
    // Helper -> "FOOD!","Found food!", "Delicious!","Bon Appétit"
    // Positive -> "Yay!","Go you!", "Yes!", "Perfect!"

    let helper = vec!["FOOD!","Found food!", "Delicious!","Bon Appétit"];
    let positive = vec!["Yay!","Go you!", "Yes!", "Perfect!"];

    let s : usize = thread_rng().gen_range(0, 3);
    let h: usize =  thread_rng().gen_range(0, 4);
    let p: usize =  thread_rng().gen_range(0, 4);

    let help = helper[h];
    let pos = positive[p];

    if s == 0 {
        print!("{}", help);
    } else if  s == 1{
        let result = format!("{} {}",pos, help);
        print!("{}", result);
    } else {
        let result = format!("{} {}",help, pos);
        print!("{}", result);
    }
}


//Context free grammar which generates messages when the user moves to a space with an enemy
fn cfg_enemy_print(){
    // Rules of the Grammar
    // S -> Helper, Negative Helper, Helper Negative
    // Helper -> "Enemy!", "This section was already occupied by something unfriendly!", "We battled an enemy!", "Other bacteria!","DISINFECTANT!"
    // Negative -> "Well...", "Oh no!", "Oooops!", "No!"

    let helper = vec!["Enemy!", "This section was already occupied by something unfriendly!", "We battled an enemy!", "Other bacteria!","DISINFECTANT!"];
    let negative = vec!["Well...", "Oh no!", "Oooops!", "No!"];

    let s : usize = thread_rng().gen_range(0, 3);
    let h: usize =  thread_rng().gen_range(0, 5);
    let n: usize =  thread_rng().gen_range(0, 4);

    let help = helper[h];
    let neg = negative[n];

    if s == 0 {
        print!("{}", help);
    } else if  s == 1{
        let result = format!("{} {}",neg, help);
        print!("{}", result);
    } else {
        let result = format!("{} {}",help, neg);
        print!("{}", result);
    }
}


fn main() {

    println!("{}","");
    println!("{}", "GAME RULES");
    println!("{}","");
    print!("You are an ambitious bacterium, and there is nothing in the world that you want more than spreading everywhere!");
    print!("To reach other petridishes, try to reach the exit as you travel through different spaces in your starting petridish. Be careful! You can run out of energy or acquire way too much damage!");
    print!("You can pick up food as you travel the different segments, but be aware of other bacteria and disinfectant spills!");
    println!("{}","");



    //----------------------------------------------
    //    BEGIN: CREATING A MAP THROUGH A FILE
    //----------------------------------------------
    
    // let contents = fs::read_to_string("test.txt")
    //     .expect("Error! I cannot read the file :(");

    // // Create an iterator 
    // let mut iterator = contents.split_whitespace();

    // // Extract information from the file
    // let num_rows:i32 = FromStr::from_str(&iterator.next().unwrap()).unwrap();
    // let num_cols:i32 = FromStr::from_str(&iterator.next().unwrap()).unwrap();

    // let mut initial_board = vec![vec![0 as i32;num_cols as usize];num_rows as usize];

    // let player_row:i32 = FromStr::from_str(&iterator.next().unwrap()).unwrap();
    // let player_col:i32 = FromStr::from_str(&iterator.next().unwrap()).unwrap();

    // let damage_num: usize= FromStr::from_str(&iterator.next().unwrap()).unwrap();
    // let energy_num: usize = FromStr::from_str(&iterator.next().unwrap()).unwrap();


    // // Place objects where they are supposed to be on the board
    // let mut object_pos_list = vec![];
    // loop {
    //     let next= &iterator.next();

    //     if next.is_some(){
    //         let next_as_usize : usize = FromStr::from_str(next.unwrap()).unwrap();
    //         object_pos_list.push(next_as_usize);
            
    //         if object_pos_list.len() == 3 {
    //             let object_kind = object_pos_list.pop().unwrap();
    //             let object_col = object_pos_list.pop().unwrap();
    //             let object_row = object_pos_list.pop().unwrap();
    
    //             initial_board[object_row][object_col] = object_kind as i32;
    //             object_pos_list.clear();
    //         }
    //     } else {
    //         break;
    //     }
    // }

    // // Create the board and the map
    // initial_board[player_row as usize][player_col as usize] = 1;
    // let mut map = Map{board:initial_board, records: vec![], damage:damage_num,energy: energy_num, row:player_row, col:player_col};

    //----------------------------------------------
    //    END: CREATING A MAP THROUGH A FILE
    //----------------------------------------------


  

    //----------------------------------------------------------------------
    //  BEGIN: CREATING A MAP THROUGH RANDOM BOARD INITIALIZATION STARTS
    //-----------------------------------------------------------------------

    let mut map = Map{board: vec![vec![0;8];8], records: vec![], damage: 0, energy:10, row:0 as i32, col:0 as i32};
    map.place_elements(20,20,7,7);

    //----------------------------------------------------------------------
    //  END: CREATING A MAP THROUGH RANDOM BOARD INITIALIZATION STARTS
    //-----------------------------------------------------------------------




    let mut input = String::new();

    println!("{}","");
    println!("{}", "STATS");
    println!("Energy : {}", map.energy);
    println!("Damage :  {}", map.damage);
    println!("{}","");




    while (map.energy > 0) & (map.damage < 5) {

        map.energy -= 1;
        input.clear();

        map.print_board();

        println!("Where should we move? Up, down, left, right?");
        print!("> ");
        // Flush so that the prompt is definitely printed
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        //Turn all input into uppercase to make sure multiple different ways to writing answers will be accepted
        let input = input.to_uppercase();

        let mut new_row:i32 = map.row;
        let mut new_col:i32 = map.col;

        let the_move;
        match input.as_str() {
            "UP" => {new_row = new_row - 1;
                    the_move = 0 },
            "DOWN" => {new_row = new_row + 1;
                    the_move = 1 },
            "RIGHT" => {new_col = new_col + 1;
                    the_move =  2},
            "LEFT" => {new_col = new_col - 1;
                    the_move = 3},

            "UNDO" => {println!("{}", "Taking a step back!");
                        map.rewind_one_move();
                        map.energy += 1;
                        println!("{}","");
                        println!("{}", "STATS");
                        println!("Energy : {}", map.energy);
                        println!("Damage :  {}", map.damage);
                        println!("{}","");
                        continue;},
            "SAVE" => {println!("{}", "Trying to save!");
                        break;},

            _ => panic!("Something went wrong!"),
        }

        if map.is_valid_position(new_row,new_col) {

            let former_occupant = map.board[new_row as usize][new_col as usize];
            map.record_state(map.row,map.col,the_move,former_occupant);
            map.move_object(new_row,new_col,map.row, map.col);

            map.row = new_row;
            map.col = new_col;

            // 0 is empty 
            // 2 is enemy
            // 3 is food
            // 4 exit
            match former_occupant {
                0 => {println!("{}","");
                     cfg_empty_print();
                     println!("{}","");}
                2 => { println!("{}","");
                    cfg_enemy_print();
                    println!("{}","");
                    map.damage += 1;}
                3 => {println!("{}","");
                     cfg_food_print();
                     println!("{}","");
                    map.energy += 1;}
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
        println!("Energy : {}", map.energy);
        println!("Damage :  {}", map.damage);
        println!("{}","");

        if map.energy == 0 {
            println!("{}", "You starved: YOU LOST");
        } 

        if map.damage == 5 {
            println!("{}", "You received way too much damage: YOU LOST");
        }



    }



}
