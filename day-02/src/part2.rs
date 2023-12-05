use crate::custom_error::AocError;

struct Color<'a> {
    name: &'a str,
    max: i32,
}

impl<'a> Color<'a> {
    fn new(name: &'a str) -> Self {
        Self { name, max: 0 }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut colors = vec![Color::new("red"), Color::new("green"), Color::new("blue")];
    let mut successfull = true;
    let mut start_index = 0;
    Ok(input
        .lines()
        .enumerate()
        .fold(0_i32, |acc, (game, x)| {
            (colors[0].max, colors[1].max, colors[2].max) = (0, 0, 0);
            successfull = true;
            let games = x[8 + (game + 1).ilog10() as usize..]
                .split("; ")
                .collect::<Vec<&str>>();
            for game in games {
                start_index = 0;
                while let Some(end_index) = game[start_index..].find(',') {
                    for color in colors.iter_mut() {
                        if game[start_index..start_index + end_index].ends_with(color.name) {
                            if let Ok(num) = game
                                [start_index..start_index + end_index - color.name.len() - 1]
                                .parse::<i32>()
                            {
                                color.max = num.max(color.max)
                            }
                        }
                    }
                    start_index += end_index + 2;
                }
                for color in colors.iter_mut() {
                    if game[start_index..].ends_with(color.name) {
                        if let Ok(num) =
                            game[start_index..game.len() - color.name.len() - 1].parse::<i32>()
                        {
                            color.max = num.max(color.max)
                        }
                    }
                }
            }
            start_index = 0;
            acc + colors[0].max * colors[1].max * colors[2].max
        })
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
