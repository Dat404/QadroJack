#![warn(clippy::all, clippy::pedantic)]

use std::io;
use std::{thread, time};
use rand::Rng;
//0.2

fn game() {
    //Ugly spaghetti code 
    println!("1 - TAKE | 0 - PASS | ANY OTHER - EXIT");
    
    let cards: [u8; 5] = [3, 4, 1, 6, 2];

    let mut player_points_count: u8 = cards[rand::thread_rng().gen_range(0..4)];
    let mut bot_points_count: u8 = cards[rand::thread_rng().gen_range(0..4)];

    loop {
        let mut player_choice: String = String::new();
        let bot_take_chance: u8 = rand::thread_rng().gen_range(1..3);

        println!("You have {player_points_count} points");

        io::stdin().read_line(&mut player_choice).unwrap();

        let in_int_p_choice: u8 = player_choice.trim().parse::<u8>().expect("Error!");

        match in_int_p_choice {
            1 => {
                player_points_count += cards[rand::thread_rng().gen_range(0..4)];
                println!("YOU take a card!");

                if bot_points_count >= 6 {
                    if bot_take_chance == 1 {
                        bot_points_count += cards[rand::thread_rng().gen_range(0..4)];
                        println!("BOT take a card!");

                    } else {
                        println!("BOT pass!");
                    }
                } else {
                    println!("BOT take a card!");
                    bot_points_count += cards[rand::thread_rng().gen_range(0..4)];
                }
            },
            0 => {
                println!("YOU pass!");

                if bot_points_count >= 6 {
                    if bot_take_chance == 1 {
                        bot_points_count += cards[rand::thread_rng().gen_range(0..4)];
                        println!("BOT take a card!");
                    } else{
                        println!("BOT pass!");
                        println!("End! \n BOT count {bot_points_count} | YOU count {player_points_count}");
                        thread::sleep(time::Duration::from_secs(5));
                        panic!()
                    }
                } else {
                    println!("BOT take a card!");
                    bot_points_count += cards[rand::thread_rng().gen_range(0..4)];
                }
            },
            _ => panic!(),
        }
        if bot_points_count >= 11 {
            println!("End! \n BOT count {bot_points_count} | YOU count {player_points_count}");
            thread::sleep(time::Duration::from_secs(5));
            panic!();
        }

        if player_points_count >= 11 {
            println!("End! \n BOT count {bot_points_count} | YOU count {player_points_count}");
            thread::sleep(time::Duration::from_secs(5));
            panic!();
        }
    }
}

fn main() {
    println!("Hi! Is a Qadrojack! on first iteration you get random value of points (1-4 or 6");
    println!("Each next iteration you choice, take random value of points (1-4, or 6) or pass");
    println!("You should win. Get 11 points or higher than you AI opponent. Good Luck!");

    println!("1 - START | ANY OTHER - EXIT");

    let mut user_choice: String = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();

    let in_integer_choice: u8 = user_choice.trim().parse::<u8>().expect("Error!");

    match in_integer_choice {
        1 => game(),
        _ => panic!(), 
    }
}