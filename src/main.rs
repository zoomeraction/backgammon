use rand::Rng;
use core::num;
use std::{fs::read};
use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::time::Instant;

fn write_to_file(game_move: &(i32, i32, i32)) {
    let data = format!("{}, {}, {}\n", game_move.0, game_move.1, game_move.2);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("game_history.txt")
        .expect("Unable to open file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
fn write_to_file_dice(dice: &(i32,i32)){
    let data = format!("{}, {}\n", dice.0, dice.1);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("dice_history.txt")
        .expect("Unable to open file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
// Display of the board
pub fn display_board(board: &Vec<(i32, i32)>, hit_stones_1: &i32, hit_stones_2: &i32, keep_count_of_1_stones: &mut i32, keep_count_of_2_stones: &mut i32) {
    const lower_nums: &str = "11 10 9  8  7  6  | 5  4  3  2  1  0";
    const upper_nums: &str = "12 13 14 15 16 17 | 18 19 20 21 22 23";
    let mut highest_lower: i32 = 0;
    let mut highest_upper: i32 = 0;
    for i in 0..12{
        if(board[i].1 > highest_lower){
            highest_lower = board[i].1;
        }
    }
    for i in 12..24{
        if(board[i].1 > highest_upper){
            highest_upper = board[i].1;
        }
    }
    println!("{upper_nums}");
    for i in 1..=highest_upper{
        let mut row_string: String = String::from("");
        for j in 12..24{
            if(board[j].1 - i >= 0){
                if(board[j].0 == 1){
                    if(j == 17){
                        row_string.push_str("*    ");
                    }else{
                        row_string.push_str("*  ");
                    }
                } else {
                    if(j == 17){
                        row_string.push_str("#    ");
                    }else{
                        row_string.push_str("#  ");
                    }
                }
            }else{
                if(j == 17){
                    row_string.push_str("     ");
                } else {
                    row_string.push_str("   ")
                }
            }
        }
        println!("{row_string}");
    }
    // println!("-------------------------------------");
    // println!("-------------------------------------"); 
    println!("");
    let mut lower_row_strings: Vec<String> = vec![];
    for i in 1..=highest_lower{
        let mut row_string: String = String::from("");
        for j in (0..12).rev(){
            if(board[j].1 - i >= 0){
                if board[j].0 == 1{
                    if j == 11 || j == 10{
                        row_string.push_str("*  ");
                    }else if j == 6 {
                        row_string.push_str("*    ")
                    }else {
                        row_string.push_str("*  ");
                    }
                } else {
                    if j == 11 || j == 10 {
                        row_string.push_str("#  ");
                    }else if j == 6{
                        row_string.push_str("#    ");
                    }else {
                        row_string.push_str("#  ");
                    }
                }
            }else{
                if j == 6{
                    row_string.push_str("     ");
                } else {
                    if j == 11 || j == 10{
                        row_string.push_str("   ");
                    } else {
                        row_string.push_str("   ")
                    }
                    
                }
            }
        }
        lower_row_strings.push(row_string);
    }
    for i in (0..lower_row_strings.len()).rev(){
        println!("{}",lower_row_strings[i]);
    }

    println!("{lower_nums}");
    println!("hit_stones_1: {}", hit_stones_1);
    println!("hit_stones_2: {}", hit_stones_2);
    println!("out_stones_1: {}", 15 - *keep_count_of_1_stones);
    println!("out_stones_2: {}", 15 - *keep_count_of_2_stones);
    if 15 - *keep_count_of_1_stones > 15 {
        println!("SSSSSS 111111 > 15");
        read_input();
    }
    if 15 - *keep_count_of_2_stones > 15 {
        println!("SSSSSS 2222222 > 15");
        read_input();
    }
    let mut s1 = 0;
    let mut s2 = 0;
    for i in 0..=23{
        if board[i].0 == 1 && board[i].1 >= 1{
            s1 += board[i].1;
        }
        if board[i].0 == 2 && board[i].1 >= 1{
            s2 += board[i].1;
        }
    }
    println!("s1: {}, s2: {}", s1, s2);
    
}

fn update_board(board: &mut Vec<(i32, i32)>, game_move: &(i32, i32, i32), hit_stones_1: &mut i32, hit_stones_2: &mut i32, keep_count_of_1_stones: &mut i32, keep_count_of_2_stones: &mut i32){
    // going from a place to an empty place
    write_to_file(game_move);
    if game_move.1 == -2 {
        board[game_move.2 as usize].1 -= 1;
        if board[game_move.2 as usize].1 == 0 {
            board[game_move.2 as usize].0 = 0 
        }
        if game_move.0 == 1{
            *keep_count_of_1_stones -= 1;
        } else {
            *keep_count_of_2_stones -= 1;
        }
    } else{
        if board[game_move.1 as usize].0 == 0{
            board[game_move.1 as usize].0 = game_move.0;
            board[game_move.1 as usize].1 = 1;
            if game_move.2 == -1{
                if game_move.0 == 1{
                    *hit_stones_1 -= 1;
                } else {
                    *hit_stones_2 -= 1;
                }
            } else {
                board[game_move.2 as usize].1 -= 1;
                if board[game_move.2 as usize].1 == 0{
                    board[game_move.2 as usize].0 = 0;
                }
            }
        // going from a place to a place you are and you are not
        } else{
            // your place
            if(board[game_move.1 as usize].0 == game_move.0){
                board[game_move.1 as usize].1 += 1;
                if game_move.2 == -1{
                    if game_move.0 == 1{
                        *hit_stones_1 -= 1;
                    } else {
                        *hit_stones_2 -= 1;
                    }
                } else {
                    board[game_move.2 as usize].1 -= 1;
                    if board[game_move.2 as usize].1 == 0{
                        board[game_move.2 as usize].0 = 0;
                    }
                }
            // not place
            } else {
                if board[game_move.1 as usize].0 == 1{
                    *hit_stones_1 += 1;
                } else {
                    *hit_stones_2 += 1;
                }
                if game_move.2 == -1{
                    if game_move.0 == 1 {
                        *hit_stones_1 -= 1;
                    } else {
                        *hit_stones_2 -= 1;
                    }
                }else{
                    board[game_move.2 as usize].1 -= 1;
                    if board[game_move.2 as usize].1 == 0{
                        board[game_move.2 as usize].0 = 0;
                    }
                }
                board[game_move.1 as usize].0 = game_move.0;
                board[game_move.1 as usize].1 = 1;   
            }
        }
    }
    // This is a shitty fix for whatever is
    // println!("hitstones1: {}, hitstones2: {}", *hit_stones_1, *hit_stones_2);
    // if *hit_stones_1 < 0 {
    //     *hit_stones_1 = 0;
    // }
    // if *hit_stones_2 < 0 {
    //     *hit_stones_1 = 0;
    // }
    // read_input();
}


fn read_input() -> i32 {
    loop {
        let mut input = String::new();
        
        println!("Please enter an integer:");

        // Read input from standard input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim the input and parse it into an i32
        match input.trim().parse::<i32>() {
            Ok(number) => return number, // Return the valid number
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

/* 
    play 
    backgammon game simulation
*/

pub fn play(number_of_games: i32, history_of_games: &mut Vec<( Vec<(i32, i32, i32)>, Vec<(i32, i32)>)>){
    let mut i = 0;
    
    let mut rng = rand::thread_rng();
    let mut win_1 = 0;
    let mut win_2 = 0;
    // Generate a random number in the range [0, 10)    
    while(i < number_of_games){
        // I need to know the number of positions available,
        // Get the legal moves and choose between them
        let mut board: Vec<(i32, i32)> = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
                                          (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
                                          (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
                                          (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];
        
        let mut history_of_a_game: Vec<(i32, i32, i32)> = vec![];
        let mut history_of_dice: Vec<(i32, i32)> = vec![];
        let mut keep_count_of_1_stones: i32 = 15;
        let mut keep_count_of_2_stones: i32 = 15;
        let mut hit_stones_1: i32 = 0;
        let mut hit_stones_2: i32 = 0;
        let mut number_of_steps: i32 = 0;
        while keep_count_of_1_stones > 0 && keep_count_of_2_stones > 0  {
            number_of_steps += 1;
            let dice1: i32 = rng.gen_range(1..=6);
            let dice2: i32 = rng.gen_range(1..=6);
            write_to_file_dice(&(dice1, dice2));
            history_of_dice.push((dice1, dice2));
            let mut possible_moves: Vec<(i32, i32, i32)> = Vec::new();
            // Find the stones with possiblitiy of the move
            if dice1 != dice2 {
                // two moves maximum to play
                // check whose turn it is first

                // START: PLAYER 11111 WITH TWO DICES NOT ALIKE
                if (history_of_dice.len() % 2) == 1 {
                    // println!("PPPPPPPPP 11111111111111");
                    // println!("keep_count_1: {},keep_count_2 {}", keep_count_of_1_stones, keep_count_of_2_stones);
                    // display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                    let mut d1_used: bool = false;
                    let mut d2_used: bool = false;
                    for i in 0..2{
                        match history_of_dice.last() {
                            Some(last_roll ) => {
                                // START: Check hit stones for player 11111
                                if hit_stones_1 > 0{
                                    // See if we have any possible moves to make and then randomly choose between them
                                    
                                    if !(board[24 - (last_roll.0 as usize) ].0 == 2 && board[24 - (last_roll.0 as usize) ].1 >= 2){
                                        possible_moves.push((1, 24 - last_roll.0, -1));
                                    }
                                    if !(board[24 - (last_roll.1 as usize) ].0 == 2 && board[24 - (last_roll.1 as usize) ].1 >= 2){
                                        possible_moves.push((1, 24 - last_roll.1, -1));
                                    
                                    }
                                    if possible_moves.len() as i32 >= hit_stones_1{
                                        // choose randomly a move from the 
                                        for i in 0..hit_stones_1{
                                            // This rand_num will be updated with calculated algorithms

                                            let rand_num = rng.gen_range(0..possible_moves.len());
                                            history_of_a_game.push(possible_moves[rand_num].clone());
                                            update_board(&mut board,  &possible_moves[rand_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            if 24 - possible_moves[rand_num].1 == last_roll.0 {
                                                d1_used = true;
                                            }else{
                                                d2_used = true;
                                            }
                                            possible_moves.remove(rand_num);
                                            
                                        }
                                        
                                        possible_moves.clear();
                                    
                                    } else {
                                        // Play the moves but there is still going to be hit_stones
                                        
                                        history_of_a_game.extend(possible_moves.clone());
                                        d1_used = true;
                                        d2_used = true;
                                        for i in 0..possible_moves.len(){
                                            update_board(&mut board, &possible_moves[i], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                        }
                                        possible_moves.clear();

                                    }
                                // END: CHECKING THE IF PLAYER 11111 has any hit stones
                                // checking if any of the dices are used
                                } else if !d1_used || !d2_used {
                                    // START: THERE IS NO HIT STONES AND PLAYER 11111 CAN MOVE FREELY BOTH DICES AVAILABLE
                                    if !d1_used && !d2_used{

                                        // randomly deciding which dice we use 
                                        let rand_num = rng.gen_range(1..=2);
                                        // START: USING THE FIRST DICE WHEN WE HAVE BOTH DICES AVAILABLE
                                        if rand_num == 1{
                                            // check if stones of player 1 can go out
                                            let mut can_player1_go_out = true;
                                            for i in 6..=23{
                                                if board[i].0 == 1 {
                                                    can_player1_go_out = false;
                                                    break;
                                                }
                                            }
                                            // if the stone can go then we need to only check the player 11111 house
                                            if can_player1_go_out {
                                                for i in 0..=5{
                                                    if(board[i].0 == 1){
                                                        
                                                        if i as i32 - (last_roll.0 ) < 0{
                                                            // going out
                                                            possible_moves.push((1, -2, i as i32));
                                                        }else{
                                                            // moving inside the house
                                                            if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
                                                                possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32))
                                                            }
                                                        }
                                                        
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0{
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }

                                            // Player 11111 can't go out, finding the right move
                                            } else{
                                                for i in 0..=23{
                                                    if(board[i].0 == 1){
                                                        // println!("DDDDD: LINE 313 {}", (last_roll.0 as usize ));
                                                        if ((i as i32) - last_roll.0 ) >= 0 {
                                                            if (!(board[((i as i32) - (last_roll.0 )) as usize].0 == 2 && board[((i as i32) - (last_roll.0 )) as usize].1 >= 2))  {
                                                                possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32));
                                                            }
                                                        }
                                                    }
                                                }
                                                if possible_moves.len() > 0{
                                                    //println!("1111111111");
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    //println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                    // println!("{}, {}", last_roll.0, last_roll.1);
                                                    // read_input();
                                                }
                                            }

                                            d1_used = true;
                                        // END: USING THE FIRST DICE WHEN WE HAVE BOTH DICES AVAILABLE 
                                        // START: USING THE SECOND DICE WHEN WE HAVE BOTH DICES AVAILABLE
                                        } else {

                                            let mut can_player1_go_out = true;
                                            for i in 6..=23{
                                                if(board[i].0 == 1){
                                                    can_player1_go_out = false;
                                                    break;
                                                }
                                            }
                                            if can_player1_go_out {
                                                // when the player go out so all the stones should be in the house hence the 0..=5 
                                                for i in 0..=5{
                                                    if(board[i].0 == 1){
                                                        if i as i32 - (last_roll.1 ) < 0{
                                                            // going out
                                                            possible_moves.push((1, -2, i as i32));
                                                        }else{
                                                            // moving inside the hourse
                                                                if !(board[i - (last_roll.1 as usize)].0 == 2 && board[i - (last_roll.1 as usize)].1 >= 2) {
                                                                    possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32))
                                                                }
                                                        }
                                                        
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0{
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }

                                            // player 11111 can't go out so we have to choose a move from the board
                                            } else{
                                                for i in 0..=23{
                                                    if(board[i].0 == 1){
                                                        // println!("DDDDD LINE 378: {}", (last_roll.1 as usize));
                                                        if ((i as i32) - last_roll.1 ) >= 0 {
                                                            if !(board[((i as i32) - (last_roll.1 )) as usize].0 == 2 && board[((i as i32) - (last_roll.1 )) as usize].1 >= 2) {
                                                                possible_moves.push((1, ((i as i32) - last_roll.1 ) as i32, i as i32));
                                                            }
                                                        }
                                                    }
                                                }
                                                if possible_moves.len() > 0{
                                                // println!("222222222");
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    // println!("-------");
                                                    // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                    // println!("WE SHOULD BE HERE");
                                                    // println!("{}, {}", last_roll.0, last_roll.1);
                                                    // read_input();
                                                }
                                            }
                                            d2_used = true;
                                        }
                                    // END: THERE IS NO HIT STONES AND PLAYER 11111 CAN MOVE FREELY BOTH DICES AVAILABLE 
                                    // START: NO HIT STONES AND PLAYER 11111 CAN MOVE WITH ONE OF THE DICES, ONE OF THE DICES HAS BEEN USED
                                    } else if (!d1_used && d2_used) || (d1_used && !d2_used){
                                        // START: FIRST DICE IS NOT USED
                                        if !d1_used {
                                            let mut can_player1_go_out = true;
                                            for i in 6..=23{
                                                if(board[i].0 == 1){
                                                    can_player1_go_out = false;
                                                    break;
                                                }
                                            }
                                            if can_player1_go_out {
                                                for i in 0..=5{
                                                    if(board[i].0 == 1){
                                                        //going out
                                                        if (i as i32) - last_roll.0 < 0{
                                                            possible_moves.push((1, -2, i as i32));
                                                        }else{
                                                            if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
                                                                possible_moves.push((1,  (i - (last_roll.0 as usize)) as i32, i as i32));
                                                            }
                                                        }
                                                        
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0 {
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }
                                            } else{
                                                for i in 0..=23{
                                                    if(board[i].0 == 1){
                                                        if ((i as i32) - last_roll.0 ) >= 0 {
                                                            // println!("DDDDD: {}", (last_roll.0 as usize));
                                                            if !(board[i - (last_roll.0 as usize)].0 == 2 && board[i - (last_roll.0 as usize)].1 >= 2) {
                                                                possible_moves.push((1,  (i as i32) - (last_roll.0) as i32, i as i32));
                                                            }
                                                        }
                                                    }
                                                }
                                                if possible_moves.len() > 0{
                                                // println!("33333333");
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                    // println!("WE SHOULD BE HERE");
                                                    // println!("{}, {}", last_roll.0, last_roll.1);
                                                    // read_input();
                                                }
                                            }
                                            d1_used = true
                                        // END: FIRST DICE IS NOT USED
                                        }
                                        // START: SECOND DICE IS NOT USED
                                        if !d2_used {
                                            let mut can_player1_go_out = true;
                                            for i in 6..=23{
                                                if(board[i].0 == 1){
                                                    can_player1_go_out = false;
                                                    break;
                                                }
                                            }
                                            if can_player1_go_out {
                                                for i in 0..=5{
                                                    if(board[i].0 == 1){
                                                        //going out
                                                        if (i as i32)- last_roll.1 < 0{
                                                            possible_moves.push((1, -2, i as i32));
                                                        }else{
                                                            if !(board[i - (last_roll.1 as usize)].0 == 2 && board[i - (last_roll.1 as usize)].1 >= 2) {
                                                                possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32))
                                                            }
                                                        }
                                                        
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0{
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }
                                            } else{
                                                for i in 0..=23{
                                                    if(board[i].0 == 1){
                                                        // println!("DDDDD: {}", (last_roll.1 as usize));
                                                        if ((i as i32) - last_roll.1 ) >= 0 {
                                                            if !(board[(i as i32 - (last_roll.1 )) as usize].0 == 2 && board[(i as i32 - (last_roll.1 )) as usize].1 >= 2) {
                                                                possible_moves.push((1,  (i - (last_roll.1 as usize)) as i32, i as i32));
                                                            }
                                                        }
                                                    }
                                                }
                                                // println!("444444444");
                                                if(possible_moves.len() > 0){
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    // println!("-------");
                                                    // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                    // println!("WE SHOULD BE HERE");
                                                    // println!("{}, {}", last_roll.0, last_roll.1);
                                                    // read_input();
                                                }
                                            }

                                    
                                            d2_used = true;
                                        }
                                        // END: SECOND DICE IS NOT USED
                                    // END: NO HIT STONES AND PLAYER 11111 CAN MOVE WITH ONE OF THE DICES, ONE OF THE DICES HAS BEEN USED
                                    } else {
                                        // It's possible to be here 
                                    }
                                }

                            },None =>{
                                // There should be a roll definitely, we'll see if there is going to be anything here
                            }
                        }
                        
                    }
                // END: PLAYER 11111 WITH TWO DICES NOT ALIKE

                // START: PLAYER 22222 TURN WITH DICES NOT ALIKE
                } else {
                    // println!("PPPPPPPPP 2222222222222");
                    // println!("keep_count_1: {},keep_count_2 {}", keep_count_of_1_stones, keep_count_of_2_stones);
                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                    let mut d1_used: bool = false;
                    let mut d2_used: bool = false;
                    for i in 0..2{
                        
                        match history_of_dice.last() {
                            Some(last_roll ) => {

                                // START: Check hit stones for player 22222
                                if hit_stones_2 > 0{

                                    // See if we have any possible moves to make and then randomly choose between them
                                    if !(board[(last_roll.0 as usize) - 1 ].0 == 1 && board[(last_roll.0 as usize) - 1 ].1 >= 2){
                                        // so basically the -1 is for showing the move is form a hit position
                                        // -2 is for going out to score
                                        possible_moves.push((2, ((last_roll.0 as usize) - 1) as i32, -1));
                                        
                                    }
                                    if !(board[(last_roll.1 as usize) - 1 ].0 == 1 && board[(last_roll.1 as usize) - 1 ].1 >= 2){
                                        possible_moves.push((2,  ((last_roll.1 as usize) - 1) as i32, -1));
                                        

                                    }
                                    if possible_moves.len() as i32 >= hit_stones_2{
                                        // choose randomly a move from the 
                                        for i in 0..hit_stones_2{
                                            // This rand_num will be updated with calculated algorithms
                                            let rand_num = rng.gen_range(0..possible_moves.len());
                                            history_of_a_game.push(possible_moves[rand_num].clone());
                                            update_board(&mut board,  &possible_moves[rand_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            if(possible_moves[rand_num].1 + 1 == last_roll.0){
                                                d1_used = true;
                                            }else{
                                                d2_used = true;
                                            }
                                            possible_moves.remove(rand_num);
                                            
                                        }
                                        
                                        possible_moves.clear();

                                    } else {
                                        // Play the moves but there is still going to be hit_stones
                                        
                                        history_of_a_game.extend(possible_moves.clone());
                                        d1_used = true;
                                        d2_used = true;
                                        for i in 0..possible_moves.len(){
                                            update_board(&mut board, &possible_moves[i], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                        }
                                        possible_moves.clear()

                                    }
                                // END: Check hit stones for player 22222
                                } else if !d1_used || !d2_used {
                                    if !d1_used && !d2_used{
                                        // randomly deciding which rand_num to use
                                        let rand_num = rng.gen_range(1..=2);
                                        if(rand_num == 1){
                                            // check if stones of player 1 can go out
                                            let mut can_player2_go_out = true;
                                            for i in 0..=17{
                                                if(board[i].0 == 2){
                                                    can_player2_go_out = false;
                                                    break;
                                                }
                                            }
                                            if can_player2_go_out {
                                                for i in 18..=23{
                                                    if(board[i].0 == 2){
                                                        //going out
                                                        if i + (last_roll.0 as usize) > 23{
                                                            possible_moves.push((2, -2, i as i32));
                                                        }else{
                                                            if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
                                                                possible_moves.push((2, (i + (last_roll.0 as usize)) as i32, i as i32));
                                                            }
                                                        }
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0{
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }
                                            } else{
                                                for i in 0..=23{
                                                    if board[i].0 == 2 {
                                                        // println!("DDDDD: {}", (last_roll.0 as usize));
                                                        if (i + (last_roll.0 as usize)) <= 23{
                                                            if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2)  {
                                                                possible_moves.push((2, (i + (last_roll.0 as usize)) as i32, i as i32))
                                                            }
                                                        }
                                                    }
                                                }
                                                if possible_moves.len() > 0{
                                                    // println!("2: 111111111");
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    // println!("-------");
                                                    // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                    // println!("WE SHOULD BE HERE");
                                                    //println!("{}, {}", last_roll.0, last_roll.1);
                                                    // read_input();
                                                }
                                            }
                                            d1_used = true;
                                        } else {
                                            let mut can_player2_go_out = true;
                                            for i in 0..=17{
                                                if(board[i].0 == 2){
                                                    can_player2_go_out = false;
                                                    break;
                                                }
                                            }
                                            if can_player2_go_out {
                                                for i in 18..=23{
                                                    if(board[i].0 == 2){
                                                        //going out
                                                        if i + (last_roll.1 as usize) > 23{
                                                            possible_moves.push((2, -2, i as i32));
                                                        }else{
                                                            if !(board[((i as i32) + last_roll.1 ) as usize].0 == 1 && board[((i as i32) + last_roll.1 )as usize].1 >= 2) {
                                                                possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
                                                            }
                                                        }
                                                        
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0{
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }
                                            } else{
                                                for i in 0..=23{
                                                    if(board[i].0 == 2){
                                                        if ((i as i32) + (last_roll.1)) <= 23{
                                                            if !(board[((i as i32) +(last_roll.1)) as usize].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2)  {
                                                                possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
                                                            }
                                                        }
                                                    }
                                                }
                                                if possible_moves.len() > 0{
                                                    // println!("2: 2222222222");
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    // println!("-------");
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                    // println!("WE SHOULD BE HERE");
                                                    // println!("{}, {}", last_roll.0, last_roll.1);
                                                    // read_input();
                                                }
                                            }
                                            d2_used = true;
                                        }

                                        
                                        
                                    } else if (!d1_used && d2_used) || (d1_used && !d2_used){
                                        if !d1_used {
                                            let mut can_player2_go_out = true;
                                            for i in 0..=17{
                                                if(board[i].0 == 2){
                                                    can_player2_go_out = false;
                                                    break;
                                                }
                                            }
                                            if can_player2_go_out {
                                                for i in 18..=23{
                                                    if(board[i].0 == 2){
                                                        //going out
                                                        if i + (last_roll.0 as usize) > 23{
                                                            possible_moves.push((2,  -2, i as i32));
                                                        }else{
                                                            if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
                                                                possible_moves.push((2,  (i + (last_roll.0 as usize)) as i32, i as i32));
                                                            }
                                                        }
                                                        
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0 {
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }
                                            } else{
                                                for i in 0..=23{
                                                    if(board[i].0 == 2){
                                                        // println!("DDDDD: {}", (last_roll.0 as usize));
                                                        if ((i as i32) + (last_roll.0)) <= 23{
                                                            if !(board[i + (last_roll.0 as usize)].0 == 1 && board[i + (last_roll.0 as usize)].1 >= 2) {
                                                                possible_moves.push((2,  (i + (last_roll.0 as usize)) as i32, i as i32));
                                                            }
                                                        }
                                                    }
                                                }
                                                if possible_moves.len() > 0{
                                                    // println!("2: 333333333333");
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    // println!("-------");
                                                    // println!("{}, {}, {}", possible_moves[ran_num].0, possible_moves[ran_num].1, possible_moves[ran_num].2);
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                    // println!("WE SHOULD BE HERE");
                                                    // println!("{}, {}", last_roll.0, last_roll.1);
                                                    // read_input();
                                                }
                                            }
                                            d1_used = true
                                        }
                                        if !d2_used {
                                            let mut can_player2_go_out = true;
                                            for i in 0..=17{
                                                if(board[i].0 == 1){
                                                    can_player2_go_out = false;
                                                    break;
                                                }
                                            }
                                            if can_player2_go_out {
                                                for i in 18..=23{
                                                    if(board[i].0 == 2){
                                                        //going out
                                                        if (i + (last_roll.1 as usize)) > 23{
                                                            possible_moves.push((2, -2, i as i32));
                                                        }else{
                                                            if !(board[i + (last_roll.1 as usize)].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2) {
                                                                possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
                                                            }
                                                        }
                                                        
                                                    }
                                                }
                                                // going to choose a random possible move and play it
                                                if possible_moves.len() > 0{
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }
                                            } else{
                                                for i in 0..=23{
                                                    if(board[i].0 == 2){
                                                        if ((i as i32) + (last_roll.1)) <= 23{
                                                            if !(board[i + (last_roll.1 as usize)].0 == 1 && board[i + (last_roll.1 as usize)].1 >= 2) {
                                                                
                                                                possible_moves.push((2,  (i + (last_roll.1 as usize)) as i32, i as i32))
                                                            }
                                                        }
                                                    }
                                                }
                                                if possible_moves.len() > 0{
                                                    let ran_num = rng.gen_range(0..possible_moves.len());
                                                    history_of_a_game.push(possible_moves[ran_num].clone());
                                                    update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                                    possible_moves.clear();
                                                }
                                            }
                                            d2_used = true;
                                        }

                                    } else {
                                        // It's possible to be here 
                                    }
                                }

                            },None =>{
                                // There should be a roll definitely, we'll see if there is going to be anything here
                            }
                        }
                        
                    }
                }
                // END: PLAYER 22222 TURN WITH DICES NOT ALIKE
            
            } 
            // This else statement is for double dices like (1,1),(2,2),(4,4)...
            else {
                // println!("DOUBLE DOUBLE DOUBLE DOUBLE");
                if history_of_dice.len() % 2 == 1 {
                    // println!("PPPPPPPPP 11111111111111");
                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                    for i in 0..4{
                        match history_of_dice.last() {
                            Some(last_roll ) => {
                                if hit_stones_1 > 0 {
                                    if !(board[24 - (last_roll.0 as usize) ].0 == 2 && board[24 - (last_roll.0 as usize) ].1 >= 2){
                                        
                                        let game_move = (1,  (24 - (last_roll.0 as usize)) as i32, -1);
                                        history_of_a_game.push(game_move);
                                        update_board(&mut board, &game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                        display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones)
                                    }
                                } else {
                                    let mut can_player_1_go_out = true;
                                    for i in 6..=23{
                                        if board[i].0 == 1{
                                            can_player_1_go_out = false;
                                            break;
                                        }
                                    }
                                    if can_player_1_go_out {
                                        for i in 0..=5{
                                            if(board[i].0 == 1){
                                                // going out
                                                if((i as i32) - last_roll.0 <= -1){
                                                    let game_move = (1, -2, i as i32);
                                                    possible_moves.push(game_move);
                                                    
                                                } else {
                                                    if !(board[i  - (last_roll.0 as usize)].0 == 2  && board[i - (last_roll.0 as usize)].1 >= 2) {
                                                        possible_moves.push((1, (i as i32) - last_roll.0, i as i32));
                                                    }
                                                }
                                            }
                                        }
                                        if possible_moves.len() > 0{
                                            let ran_num = rng.gen_range(0..possible_moves.len());
                                            history_of_a_game.push(possible_moves[ran_num].clone());
                                            update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            possible_moves.clear();
                                        }
                                    } else {
                                        for i in 0..=23{
                                            if board[i].0 == 1 {
                                                if ((i as i32)  - (last_roll.0)) > 0 {
                                                    if !(board[((i as i32)  - (last_roll.0)) as usize].0 == 2  && board[((i as i32)  - (last_roll.0)) as usize].1 >= 2) {
                                                        possible_moves.push((1, (i as i32) - last_roll.0, i as i32));
                                                    }
                                                }
                                            }
                                        }
                                        if possible_moves.len() > 0{
                                            let ran_num = rng.gen_range(0..possible_moves.len());
                                            history_of_a_game.push(possible_moves[ran_num].clone());
                                            update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            possible_moves.clear();
                                        }
                                    }
                                }
                            },None=>{

                            }
                        }
                    }
                } else {
                    //println!("PPPPPPPPP 2222222222222222");
                    display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                    for i in 0..4{
                        match history_of_dice.last() {
                            Some(last_roll) => {
                                if hit_stones_2 > 0 {
                                    if !(board[(last_roll.0 as usize) - 1 ].0 == 1 && board[(last_roll.0 as usize) - 1 ].1 >= 2){
                                        
                                        let game_move = ( 2,   ((last_roll.0 as usize) - 1) as i32, -1);
                                        history_of_a_game.push(game_move);
                                        update_board(&mut board, &game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                        display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                    }
                                } else {
                                    let mut can_player_2_go_out = true;
                                    for i in 0..=17{
                                        if board[i].0 == 2 {
                                            can_player_2_go_out = false;
                                            break;
                                        }
                                    }
                                    if can_player_2_go_out {
                                        for i in 18..=23{
                                            if board[i].0 == 2 {
                                                // going out
                                                if((i as i32) + last_roll.0 > 23){
                                                    let game_move = (2, -2, i as i32);
                                                    possible_moves.push(game_move);
                                                    
                                                } else {
                                                    if !(board[i  + (last_roll.0 as usize)].0 == 1  && board[i + (last_roll.0 as usize)].1 >= 2) {
                                                        possible_moves.push((2,  (i as i32) + last_roll.0, i as i32));
                                                    }
                                                }
                                            }
                                        }
                                        if possible_moves.len() > 0{
                                            let ran_num = rng.gen_range(0..possible_moves.len());
                                            history_of_a_game.push(possible_moves[ran_num].clone());
                                            update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            possible_moves.clear();
                                        }
                                    } else {
                                        for i in 0..=23{
                                            if board[i].0 == 2 {
                                                // println!("DDDDD: {}", (last_roll.0 as usize));
                                                if i + (last_roll.0 as usize) <= 23{
                                                    if !(board[i  + (last_roll.0 as usize)].0 == 1  && board[i + (last_roll.0 as usize)].1 >= 2) {
                                                        possible_moves.push((2,  (i as i32) + last_roll.0, i as i32));
                                                    }                                                    
                                                }

                                            }
                                        }
                                        if possible_moves.len() > 0{
                                            let ran_num = rng.gen_range(0..possible_moves.len());
                                            history_of_a_game.push(possible_moves[ran_num].clone());
                                            update_board(&mut board, &possible_moves[ran_num], &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
                                            possible_moves.clear();
                                        }
                                    }
                                }
                            }, None => {
                            }
                        }
                    }
                }
            }
            // println!("number of steps: {}", number_of_steps);      
            if keep_count_of_1_stones > 15 {
                read_input();
            }
            if keep_count_of_2_stones > 15 {
                read_input();
            }
            if keep_count_of_1_stones <= 0{
                win_1 += 1;
            }
            if keep_count_of_2_stones <= 0{
                win_2 += 1;
            }
        }

        read_input();
        i += 1;
        write_to_file(&(0,0,0));
        write_to_file_dice(&(0,0));
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@")
        
    }
    println!("win rate: {}, {}", win_1, win_2);
}

// pub fn mut_vec(v: &mut Vec<i32>){
//     let mut i: i32 = 0;
//     while(i < 32){
//         v.push(i);
//         i += 1;
//     }
// }


// todo: figure out how to make the program in parallel with a mutex that keeps count of the file numbers
fn main() {
    
    // // (user (1,2), destination_position(0...23), number_of_stones)
    let mut history_of_a_game: Vec<(i32, i32, i32)> = vec![];
    // let mut history_of_dice: Vec<(i8, i8)> = vec![];

    // history_of_dice.push((4,3));
    // history_of_dice.push((4,5));

    // history_of_a_game.push((2, 4, 0));
    // history_of_a_game.push((2, 3, 0));
    // history_of_a_game.push((1, 19, 23));
    // history_of_a_game.push((1, 7, 12));
    // history_of_a_game.push((2, 19, 18));
    // history_of_a_game.push((2, 23, 18));

    // println!("history_of_a_game: {:?}", history_of_a_game);

    let mut history_of_games: Vec<(Vec<(i32, i32, i32)>, Vec<(i32, i32)>)> = vec![];
    // //history_of_games.push((history_of_a_game, history_of_dice));

    // println!("history_of_games: {:?}", history_of_games);
    // // let mut tmp_vec: Vec<i32> = vec![];
    // // tmp_vec.push(1);
    // // tmp_vec.push(10);
    // // println!("tmp_vec: {:?}", tmp_vec);
    // // mut_vec(&mut tmp_vec);
    // // println!("tmp_vec: {:?}", tmp_vec);

    // let mut rng = rand::thread_rng();

    // // Generate a random number in the range [0, 7)
    // let n1: i32 = rng.gen_range(1..7);
    // println!("Random number between 1 and inclusive 7: {}", n1);

    let mut board: Vec<(i32, i32)> = vec![(2,2), (0,0), (0,0), (0,0), (0,0), (1,5), 
                                        (0,0), (1,3), (0,0), (0,0), (0,0), (2,5), 
                                        (1,5), (0,0), (0,0), (0,0), (2,3), (0,0), 
                                        (2,5), (0,0), (0,0), (0,0), (0,0), (1,2)];
    
    let mut hit_stones_1 : i32 = 0;
    let mut hit_stones_2 : i32 = 0;
    let mut keep_count_of_1_stones: i32 = 15;
    let mut keep_count_of_2_stones: i32 = 15;
    // let mut board: Vec<(i32, i32)> = vec![(1,2), (1,2), (1,5), (1,5), (1,1), (0,0),
    //                                     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    //                                     (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
    //                                     (0,0), (2,5), (2,3), (2,5), (2,2), (0,0)];
    //display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    
    // let mut board: Vec<(i32, i32)> = vec![(2,2), (1,1), (1,1), (1,1), (1,1), (1,1),
    //                                     (0,0), (1,3), (0,0), (0,0), (0,0), (2,5),
    //                                     (1,5), (0,0), (0,0), (0,0), (2,3), (0,0),
    //                                     (2,3), (2,1), (0,0), (0,0), (1,1), (1,1)];
    
    // hit_stones_2 = 1;
    // display_board(&board, &hit_stones_1, &hit_stones_2);

    // history_of_a_game.push((1, 5, 7));
    // history_of_a_game.push((1, 3, 7));
    // history_of_a_game.clear();


    // for game_move in &history_of_a_game{
    //     println!("{}, {}, {}",game_move.0, game_move.1, game_move.2);
    //     update_board(&mut board, game_move, &mut hit_stones_1, &mut hit_stones_2);
    //     display_board(&board, &hit_stones_1, &hit_stones_2);
    // }
    // history_of_a_game.push((2, 4, -1));
    // history_of_a_game.push((2, 19, 18));
    // for game_move in &history_of_a_game{
    //     println!("{}, {}, {}", game_move.0, game_move.1, game_move.2);
    //     update_board(&mut board, game_move, &mut hit_stones_1, &mut hit_stones_2);
    //     display_board(&board, &hit_stones_1, &hit_stones_2);
    // }
    // ***************************************************************************************************************
    // println!("Hello");
    // display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    // history_of_a_game.push((2, -2, 20));
    // history_of_a_game.push((2, -2, 19));

    // history_of_a_game.push((1, -2, 0));
    // history_of_a_game.push((1, -2, 1));

    // history_of_a_game.push((2, -2, 20));
    // history_of_a_game.push((2, -2, 22));

    // for game_move in &history_of_a_game{
    //     //println!("{}, {}, {}", game_move.0, game_move.1, game_move.2);
    //     update_board(&mut board, game_move, &mut hit_stones_1, &mut hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    //     display_board(&board, &hit_stones_1, &hit_stones_2, &mut keep_count_of_1_stones, &mut keep_count_of_2_stones);
    // }
    // ***************************************************************************************************************
        

    // let number = read_input();
    // println!("You entered: {}", number);
    let start = Instant::now();
    play(1000, &mut history_of_games);
    let duration = start.elapsed();
    println!("Time taken by some_function: {:?}", duration);
    
    
}
