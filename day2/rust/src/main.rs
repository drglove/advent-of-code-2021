#[test]
fn sample() {
    let test_moves =
    r#"forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2"#;
    assert_eq!(part1(test_moves), 150);
}

struct SubmarineMove {
    horiz_move: i32,
    vert_move: i32,
}

fn line_to_submove(submove: &str) -> SubmarineMove {
    let split_move: Vec<&str> = submove.split_whitespace().collect();
    let amount: i32 = split_move[1].parse().unwrap();
    match split_move[0] {
        "forward" => SubmarineMove{horiz_move: amount, vert_move: 0},
        "up" => SubmarineMove{horiz_move: 0, vert_move: -amount},
        "down" => SubmarineMove{horiz_move: 0, vert_move: amount},
        _ => unreachable!()
    }
}

fn part1(moves: &str) -> i32 {
    let overall_move = moves
        .lines()
        .map(|submove| line_to_submove(submove))
        .reduce(|a, b| SubmarineMove{horiz_move: a.horiz_move + b.horiz_move, vert_move: a.vert_move + b.vert_move})
        .unwrap();
    overall_move.horiz_move * overall_move.vert_move
}

fn main() {
    let moves = include_str!("../../input.txt");
    println!("Part 1: {}", part1(moves));
}
