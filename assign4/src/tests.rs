#![allow(dead_code)]
include!("main.rs");

#[test]
fn simple_board() {
    let a = readFrom("board 2 2");
    let b = a.as_str();
    let ans = "+---+---+
|  4|  3|
|   |   |
+---+---+
|  1|  2|
|   |   |
+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn simple_board_players() {
    let a = readFrom("board 2 3
players 2");
    let b = a.as_str();
    let ans = "+---+---+
|  5|  6|
|   |   |
+---+---+
|  4|  3|
|   |   |
+---+---+
|  1|  2|
|B  |A  |
+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn read_0() {
    let a = readFrom("board 3 4
players 3");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|   |   |   |
+---+---+---+
|  7|  8|  9|
|   |   |   |
+---+---+---+
|  6|  5|  4|
|   |   |   |
+---+---+---+
|  1|  2|  3|
|C  |B  |A  |
+---+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn read_1() {
    let a = readFrom("board 3 4
players 2
dice 1
turns 4");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|   |   |   |
+---+---+---+
|  7|  8|  9|
|   |   |   |
+---+---+---+
|  6|  5|  4|
|A  |B  |   |
+---+---+---+
|  1|  2|  3|
|   |   |   |
+---+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn read_2() {
    let a = readFrom("board 3 4
players 2
dice 1 2
turns 5");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|A  |B  |   |
+---+---+---+
|  7|  8|  9|
|   |   |   |
+---+---+---+
|  6|  5|  4|
|   |   |   |
+---+---+---+
|  1|  2|  3|
|   |   |   |
+---+---+---+
Player A won
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn read_3() {
    let a = readFrom("board 3 4
players 2
dice 1 2 2 2 2
ladder 5 11
snake 8 4
turns 5");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|B  |   |   |
+---+---+---+
|  7|  8|  9|
|   |  S|   |
+---+---+---+
|  6|  5|  4|
|   |  L|A  |
+---+---+---+
|  1|  2|  3|
|   |   |   |
+---+---+---+
Player B won
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn read_3_no_snakes() {
    let a = readFrom("board 3 4
players 2
dice 1 2 2 2 2
ladder 5 11
turns 5");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|B  |   |   |
+---+---+---+
|  7|  8|  9|
|   |A  |   |
+---+---+---+
|  6|  5|  4|
|   |  L|   |
+---+---+---+
|  1|  2|  3|
|   |   |   |
+---+---+---+
Player B won
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn read_4() {
    let a = readFrom("board 3 4
players 2
dice 1 2 2 2 2
ladder 5 11
snake 8 4
powerup escalator 6 9
powerup antivenom 7
powerup double 4
turns 10");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|B  |   |   |
+---+---+---+
|  7|  8|  9|
| a |  S| e |
+---+---+---+
|  6|  5|  4|
| e |  L|Ad |
+---+---+---+
|  1|  2|  3|
|   |   |   |
+---+---+---+
Player B won
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn antivenom() {
    let a = readFrom("board 3 4
powerup antivenom 3
snake 4 1
players 1
dice 3 1
turns 2");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|   |   |   |
+---+---+---+
|  7|  8|  9|
|   |   |   |
+---+---+---+
|  6|  5|  4|
|   |   |  S|
+---+---+---+
|  1|  2|  3|
|   |A  | a |
+---+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn ladder_simple() {
    let a = readFrom("board 3 4
ladder 3 4
players 1
dice 3
turns 1");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|   |   |   |
+---+---+---+
|  7|  8|  9|
|   |   |   |
+---+---+---+
|  6|  5|  4|
|   |   |A  |
+---+---+---+
|  1|  2|  3|
|   |   |  L|
+---+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn ladder_complex() {
    let a = readFrom("board 3 4
ladder 3 4
ladder 4 8
players 1
dice 3
turns 1");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|   |   |   |
+---+---+---+
|  7|  8|  9|
|   |A  |   |
+---+---+---+
|  6|  5|  4|
|   |   |  L|
+---+---+---+
|  1|  2|  3|
|   |   |  L|
+---+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn ladder_complex_with_snake() {
    let a = readFrom("board 3 4
ladder 3 4
ladder 4 8
snake 8 2
players 1
dice 3
turns 1");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|   |   |   |
+---+---+---+
|  7|  8|  9|
|   |  S|   |
+---+---+---+
|  6|  5|  4|
|   |   |  L|
+---+---+---+
|  1|  2|  3|
|   |A  |  L|
+---+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}

#[test]
fn powerups() {
    let a = readFrom("board 3 4
powerup escalator 7 10
powerup antivenom 3 6
powerup double 4 8");
    let b = a.as_str();
    let ans = "+---+---+---+
| 12| 11| 10|
|   |   | e |
+---+---+---+
|  7|  8|  9|
| e | d |   |
+---+---+---+
|  6|  5|  4|
| a |   | d |
+---+---+---+
|  1|  2|  3|
|   |   | a |
+---+---+---+
";
    let b : bool = b == ans;
    assert!(b);
}
