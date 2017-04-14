#[derive(Debug)]
struct Board{
    width: i32,
    height: i32,
    size: i32,
    cells: Vec<Cell>,
}

impl Board{
    fn new(w: i32, h: i32) -> Board{
        let s = w*h;
        let mut c = vec![];
        for i in 0..s {
            let mut tmp = Cell::new(i+1, '?' , Powerup::new('?', 0) );
            c.push(tmp);
        }

        let mut rev = false;
        let mut ind: i32= 0;

        Board{
            width: w,
            height: h,
            size: s,
            cells: c,
        }
    }

    fn getCellBasedOnNumber(&self, num: i32) -> i32{
        let mut res = 0;
        for i in 0..self.size{
        
            if self.cells[i as usize].index == num{

                res = i;

            }
        }

        res
    }


    fn addSpecial(&mut self, i: i32 , p: Powerup){

        let mut ind = self.getCellBasedOnNumber(i);
        self.cells[ind as usize].special = p;
    }

    fn to_str(self) -> String {

        let mut resTop = "|".to_owned();
        let mut resBot = "".to_owned();
        let mut separator = "+".to_owned();

        for i in 0..self.width{
            separator+="---+";
        }
        separator+= "\n";

        let mut result = "".to_owned();

        let mut cnt = 0;
        let mut rev = false;
        for i in 0..self.height {

            for j in 0..self.width{

                let mut ind = (self.width * i) + j;

                if rev == false{

                    resTop = resTop +  self.cells[ind as usize].topStr().as_str();
                    resBot = resBot + self.cells[ind as usize].bottomStr().as_str();
                }else {

                    resTop = self.cells[ind as usize].topStr() + resTop.as_str();
                    resBot = self.cells[ind as usize].bottomStr() + resBot.as_str() ;
                }
            }

            if rev == true {
                result = "|".to_string() + resTop.as_str() + "\n|" +  resBot.as_str() + "\n" + separator.as_str() + &result;
                resTop = "|".to_string();
                resBot = "".to_string();
                rev = false;
            }else {

                result =  resTop + "\n|" +  resBot.as_str() + "\n" + separator.as_str() + &result;
                resTop = "".to_string();
                resBot = "".to_string();
                rev = true;
            }


        }
        result = separator + &result;

        String::from(result)
    }
}
