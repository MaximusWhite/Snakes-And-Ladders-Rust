#[derive(Debug)]

struct Game {
    board: Board,
    dice: Vec<i32>,
    players: Vec<Player>,
    diceIndex: i32,
    currentPlayer: i32,
    winner: char,
}

impl Game{

    fn new(brd: Board, dc: Vec<i32>, pl: Vec<Player>) -> Game{
        Game {
            board: brd,
            dice: dc,
            players: pl,
            diceIndex: 0,
            currentPlayer: 0,
            winner: '?',
        }
    }

    fn addBoard(&mut self,brd: Board) {
        self.board = brd;
    }

    fn addDice(&mut self,dc: Vec<i32>) {
        self.dice = dc;
    }

    fn addPlayers(&mut self, pl: Vec<Player>) {
        self.players = pl;
    }

    fn to_str(self) -> String{

        if self.winner == '?'{
            return self.board.to_str()
        }else {

            return self.board.to_str()+"Player "+self.winner.to_string().as_str() + " won\n"

        }
    }

    fn updatePlayersOnBrd(&mut self){

        for i in 0..self.board.cells.len() as i32{
            self.board.cells[i as usize].player = '?';
        }

        for p in &self.players {
            self.board.cells[(p.position) as usize].player = p.name;
        }
        let mut last = self.board.size-1;
        if self.board.cells[last as usize].player != '?'{
            self.winner = self.board.cells[last as usize].player;
        }

    }

    fn popDice(&mut self) -> i32{


        if self.diceIndex > self.dice.len() as i32-1 {
            self.diceIndex = 0;
        }

        let mut tmp = self.dice[self.diceIndex as usize];
        self.diceIndex = self.diceIndex + 1;

        tmp
    }

    fn doTurn(&mut self){

        if self.winner != '?' {return};
        for i in 0..self.players.len() as i32{
            let mut die = self.popDice();
            let mut playerName = self.players[i as usize].name;
            self.movePlayer(playerName, die);
            self.updatePlayersOnBrd();
            if self.winner != '?' {return};

        }



    }

    fn movePlayer(&mut self, player: char, mut amount: i32 ){
        let mut playerIndex = self.getPlayerIndex(player);
        if self.players[self.getPlayerIndex(player) as usize].powerup.d == true {

            amount = amount * 2;
            self.players[playerIndex as usize].powerup.d = false;
        }

        let mut destination = self.players[playerIndex as usize].position + amount;

        if destination > self.board.size-1 {return;};

        match self.board.cells[destination as usize].special.t {
            'L' => {
                self.players[playerIndex as usize].position = destination;
                let mut newAmount = (self.board.cells[destination as usize].special.disp-1) - self.players[self.getPlayerIndex(player) as usize].position;
                if self.players[playerIndex as usize].powerup.e == true {
                    newAmount = newAmount * 2;
                    self.players[playerIndex as usize].powerup.e = false;
                }
                self.movePlayer(player, newAmount);
                self.updatePlayersOnBrd();
                return;
            },

            'S' => {
                let mut newAmount =  self.board.cells[destination as usize].special.disp-1 - destination;
                self.players[playerIndex as usize].position = destination;
                if self.players[playerIndex as usize].powerup.a == true {

                    self.players[playerIndex as usize].powerup.a = false;
                    return;
                }
                self.movePlayer(player, newAmount);
                self.updatePlayersOnBrd();
                return;

            },

            'd' => {
                self.players[playerIndex as usize].powerup.d = true;
                self.updatePlayersOnBrd();

            },

            'e' => {

                self.players[playerIndex as usize].powerup.e = true;
                self.updatePlayersOnBrd();

            },
            'a' => {

                self.players[playerIndex as usize].powerup.a = true;
                self.updatePlayersOnBrd();

            },

            _ => {}
        }

        if self.board.cells[destination as usize].player != '?'{

            let mut otherPLayer = self.board.cells[destination as usize].player;
            self.movePlayer(otherPLayer, 1);
            self.updatePlayersOnBrd();
        }

        self.players[playerIndex as usize].position = destination;
        self.updatePlayersOnBrd();


    }

    fn getPlayerIndex(&self, name: char) -> i32{
        let mut res = 0;
        for i in 0..self.players.len() as i32{

            if self.players[i as usize].name == name {

                res = i;
            }

        }

        res
    }

}

fn makeDice(rest: &[&str]) -> Vec<i32>{

    let mut res = vec![];
    for i in rest{
        res.push(i.parse::<i32>().unwrap());
    }
    res

}
