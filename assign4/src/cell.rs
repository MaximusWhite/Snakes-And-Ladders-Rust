#[derive(Debug)]
struct Cell {
    index: i32,
    player: char,
    special: Powerup,
}

impl Cell{

    fn new(ind: i32, pl: char, sp: Powerup) -> Cell {
            Cell {

                index: ind,
                player: pl,
                special: sp,
            }
    }

    fn topStr(&self) -> String {
        let mut res = "".to_string();
        let mut len = self.index.to_string().len();

        for i in 0..3-len {

            res = res + " ";
        }

        res = res + self.index.to_string().as_str();

        res+"|"
    }

    fn bottomStr(&self) -> String {
        let mut res = "".to_string();

        if self.player == '?'{

            res = res + " ";

        }else {
            res = res + self.player.to_string().as_str();
        }

        match self.special.t {
                'L' => {res = res + " L"},
                'S' => {res = res + " S"},
                'a' => {res = res + "a "},
                'd' => {res = res + "d "},
                'e' => {res = res + "e "},
                _ => { res = res + "  "},
        }

        res+"|"
    }

    fn to_str(&self) -> String {
        self.index.to_string()
    }

}
