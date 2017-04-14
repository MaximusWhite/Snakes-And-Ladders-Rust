#[derive(Debug)]
struct Player {
    position: i32,
    name: char,
    powerup: PlayerPowerup,
}

impl Player {

    fn new(pos: i32, n: char, pu: PlayerPowerup) -> Player{

        Player{
            position: pos,
            name: n,
            powerup: pu,
        }
    }

    fn to_str(&self) -> String {

        self.name.to_string()

    }

    fn copy(self) -> Player {

        Player::new(self.position, self.name, self.powerup.copy())

    }

    fn mockPlayer() -> Player{

        Player{
            position: -1,
            name: '?',
            powerup: PlayerPowerup::new(false,false,false),
        }

    }


}

fn makePlayers(amount: i32) -> Vec<Player> {

    let mut id: u8 = 65;
    let mut res = vec![];

    for i in 0..amount {
        
        res.push(Player::new(amount-i-1, (id+(i as u8)) as char, PlayerPowerup::new(false,false,false)));

    }

    res
}
