use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Debug)]
struct GameDraw {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameDraw {
    fn from_draw_pair(draw_pair: Pair<'_, Rule>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for draw_item in draw_pair.into_inner() {
            let mut draw_item_iter = draw_item.into_inner();

            let count = draw_item_iter
                .next()
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            match draw_item_iter.next().unwrap().as_str() {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => unreachable!(),
            }
        }

        Self { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    #[allow(dead_code)]
    id: u32,
    draws: Vec<GameDraw>,
}

impl Game {
    fn from_game_pair(game_pair: Pair<'_, Rule>) -> Self {
        let mut id = 0;
        let mut draws = vec![];

        for game_item in game_pair.into_inner() {
            match game_item.as_rule() {
                Rule::id => id = game_item.as_str().parse::<u32>().unwrap(),
                Rule::draw_list => {
                    for draw_item in game_item.into_inner() {
                        draws.push(GameDraw::from_draw_pair(draw_item));
                    }
                }
                _ => unreachable!(),
            }
        }

        Self { id: id, draws }
    }

    fn get_minimal_cubes(&self) -> GameDraw {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for draw in self.draws.iter() {
            if draw.red > red {
                red = draw.red;
            }
            if draw.green > green {
                green = draw.green;
            }
            if draw.blue > blue {
                blue = draw.blue;
            }
        }

        GameDraw { red, green, blue }
    }
}

#[derive(Parser)]
#[grammar = "game.pest"]
pub struct GameParser;

fn main() {
    let mut sum = 0;

    let input = include_str!("input01.txt");

    let file = GameParser::parse(Rule::file, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    for game_line in file.into_inner() {
        match game_line.as_rule() {
            Rule::game => {
                let game = Game::from_game_pair(game_line);

                let min_cubes = game.get_minimal_cubes();

                sum += min_cubes.red * min_cubes.green * min_cubes.blue;
            }
            _ => {}
        }
    }

    println!("Sum = {}", sum);
}
