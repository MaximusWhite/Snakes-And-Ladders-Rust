use std::io::BufRead;
use std::io;
use std::io::stdin;
use std::io::prelude::*;
include!("game.rs");
include!("cell.rs");
include!("player.rs");
include!("board.rs");

#[derive(Debug)]
#[allow(dead_code)]
struct Powerup{
    t: char,
    disp: i32,
}
impl Powerup {
    fn new(tp: char, d: i32) -> Powerup{
        Powerup {
            t: tp,
            disp: d,
        }
    }
    fn mockPu() -> Powerup{

        Powerup {
            t: '?',
            disp: 0,
        }

    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct PlayerPowerup {
    a: bool,
    d: bool,
    e: bool,
}

impl PlayerPowerup{

    fn new(ant: bool, double: bool, esc: bool) -> PlayerPowerup{

        PlayerPowerup{
                a: ant,
                d: double,
                e: esc,
        }
    }
    fn copy(&self) -> PlayerPowerup{

        PlayerPowerup::new(self.a,self.d,self.e)

    }

}

#[allow(dead_code)]
fn readFrom(tmp: &str) -> Game{

    let lines = tmp.lines();

    let mut game = Game::new( Board::new(0,0) , vec![], vec![]);

    for st in lines {

        readFromHelp(st, &mut game);

    }

    game

}
#[allow(dead_code)]
fn readFromHelp(tmp: &str, mut game: &mut Game){

    let mut words = tmp.split(" ");

    let vec = words.collect::<Vec<&str>>();

    match vec[0] {

        "board" => {
            game.board = Board::new(vec[1].parse::<i32>().unwrap(),vec[2].parse::<i32>().unwrap());
        },

        "players" => {

            game.players = makePlayers(vec[1].parse::<i32>().unwrap());
            game.updatePlayersOnBrd();

        },

        "dice" => {

            game.dice = makeDice(&vec[1..]);

        },

        "snake" => {
            game.board.addSpecial(vec[1].parse::<i32>().unwrap(), Powerup::new('S', vec[2].parse::<i32>().unwrap()));
        },

        "ladder" => {
            game.board.addSpecial(vec[1].parse::<i32>().unwrap(), Powerup::new('L', vec[2].parse::<i32>().unwrap()));
        },

        "powerup" => {

            match vec[1] {

                "escalator" => {

                        for i in 2..vec.len(){
                            game.board.addSpecial(vec[i as usize].parse::<i32>().unwrap(), Powerup::new('e', 0));
                        }
                },

                "double" => {
                        for i in 2..vec.len(){
                            game.board.addSpecial(vec[i as usize].parse::<i32>().unwrap(), Powerup::new('d', 0));
                        }
                },

                "antivenom" => {

                        for i in 2..vec.len(){
                            game.board.addSpecial(vec[i as usize].parse::<i32>().unwrap(), Powerup::new('a', 0));
                        }
                },

                _ => {}
            }

        },

        "turns" => {

            let mut amount = vec[1].parse::<i32>().unwrap();

            for i in 0..amount{

                game.doTurn();
            }
        }

        _ => {},

    }

}
#[allow(dead_code)]
fn main() {

    let mut input = "".to_owned();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        input = input + line.unwrap().as_str() + "\n";

    }

    let game = readFrom(input.as_str());

    let result = print(game);

    println!("{}", result);
}

fn print(game: Game) -> String{

    game.to_str()

}
