
pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {

    let mut x=false;
    let mut o=false;
    if horizontal('X',table) || vertical('X',table) || diagonals('X',table){
        x=true;
    }

    if horizontal('O',table) || vertical('O',table) || diagonals('O',table){
        o=true;
    }

    if o==x{
        return "tie".to_owned();
    }

    if o {
        return "player O won".to_owned();
    }
    
    "player X won".to_owned()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
  
    let mut bl=true;
    for i in 0..3{
        if table[i][i]!=player{
               bl=false;
            break
    }
    }
    if bl==true{
        return true;
    }
    
    bl=true;
    for i in 0..3{
        if table[2-i][i]!=player{
               bl=false;
            break
    }
    }
    if bl==true{
        return true;
    }


   false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3{
        let mut bl=true;
        for j in 0..3{
            if table[i][j]!=player{
                bl=false;
                break
            }
        }
        if bl==true{
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut bl=true;
    for i in 0..3{
        bl=true;
        for j in 0..3{
            if table[j][i]!=player{
                bl=false;
                break
            }
        }
        if bl{
            return true;
        }
    }
    bl
}