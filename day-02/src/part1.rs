use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let colors = vec![("red",12), ("green",13), ("blue", 14)];
    let mut successfull = true;
    let mut start_index = 0;
    Ok(input.lines().enumerate().fold(0_i32, |acc , (game, x)| {
        successfull = true;
        let games = x[8+(game+1).ilog10() as usize..].split("; ").collect::<Vec<&str>>();
        'outer: for game in games {
            start_index = 0;
            while let Some(end_index) = game[start_index..].find(',') {
                for (color, max_num) in &colors {
                    if game[start_index..start_index+end_index].ends_with(color) {
                        if let Ok(num) = game[start_index..start_index+end_index-color.len() - 1].parse::<i32>() {
                            if num > *max_num {
                                successfull = false;
                                break 'outer;
                            }
                        }
                    }
                }
                start_index += end_index + 2;
            }
            for (color, max_num) in &colors {
                if game[start_index..].ends_with(color) {
                    if let Ok(num) = game[start_index..game.len() - color.len() - 1].parse::<i32>() {
                        if num > *max_num {
                            successfull = false;
                            break 'outer;
                        }
                    }
                }
            }
        }
        start_index = 0;
        match successfull {
            true => acc + game as i32 + 1,
            false => acc
        }
    }).to_string())
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
