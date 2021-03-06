use itertools::Itertools;

fn part1(heights: &str) -> usize {
    heights
        .lines()
        .map(|num| num.parse::<u32>().unwrap())
        .tuple_windows::<(_,_)>()
        .filter(|pairs| pairs.0 < pairs.1)
        .count()
}

fn part2(heights: &str) -> usize {
    heights
        .lines()
        .map(|num| num.parse::<u32>().unwrap())
        .tuple_windows::<(_,_,_)>()
        .map(|triplets| triplets.0 + triplets.1 + triplets.2)
        .tuple_windows::<(_,_)>()
        .filter(|pairs| pairs.0 < pairs.1)
        .count()
}

fn main() {
    //let heights = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    let heights = include_str!("../../input.txt");
    println!("Increases part 1: {}", part1(heights));
    println!("Increases part 2: {}", part2(heights));
}
