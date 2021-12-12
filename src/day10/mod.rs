mod input;

#[derive(Debug)]
enum SyntaxError {
    Incomplete(Vec<char>),
    ExpectedOpeningCharacter(char),
    WrongClosingCharacter(char, char),
}

const CHARS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

fn is_opening(ch: char) -> bool {
    CHARS.iter().any(|&(c, _)| c == ch)
}

fn is_closing(ch: char) -> bool {
    CHARS.iter().any(|&(_, c)| c == ch)
}

fn wrong_closing_character_score(ch: char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid argument"),
    }
}

fn closing_for_opening(opening: char) -> Option<char> {
    CHARS
        .iter()
        .find_map(|&(op, cl)| if op == opening { Some(cl) } else { None })
}

fn check_opening_closing(opening: char, closing: char) -> Result<(), SyntaxError> {
    let cl = closing_for_opening(opening).expect("Wrong opening character.");
    if cl == closing {
        Ok(())
    } else {
        Err(SyntaxError::WrongClosingCharacter(cl, closing))
    }
}

fn interpret_instruction(line: &str) -> Result<(), SyntaxError> {
    let mut stack: Vec<char> = vec![];
    for ch in line.chars() {
        if is_opening(ch) {
            stack.push(ch);
        } else if is_closing(ch) {
            if let Some(opening) = stack.pop() {
                check_opening_closing(opening, ch)?;
            } else {
                return Err(SyntaxError::ExpectedOpeningCharacter(ch));
            }
        }
    }
    if !stack.is_empty() {
        return Err(SyntaxError::Incomplete(stack));
    }
    Ok(())
}

fn incomplete_score(stack: &[char]) -> u64 {
    let mut score = 0;
    for &ch in stack {
        score = score * 5
            + match &ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
    }
    score
}

fn total_incomplete_score(results: &[Result<(), SyntaxError>]) -> u64 {
    let mut incomplete_scores: Vec<u64> = results
        .iter()
        .filter_map(|res| {
            if let Err(SyntaxError::Incomplete(stack)) = res {
                Some(incomplete_score(stack))
            } else {
                None
            }
        })
        .collect();
    incomplete_scores.sort_unstable();
    incomplete_scores[incomplete_scores.len() / 2]
}

fn total_wrong_char_score(results: &[Result<(), SyntaxError>]) -> u64 {
    results
        .iter()
        .filter_map(|res| {
            if let Err(SyntaxError::WrongClosingCharacter(_, ch)) = res {
                Some(wrong_closing_character_score(*ch))
            } else {
                None
            }
        })
        .sum()
}

fn calculate_scores(input: &str) -> (u64, u64) {
    let results: Vec<Result<(), SyntaxError>> = input.lines().map(interpret_instruction).collect();
    let wrong_total_score = total_wrong_char_score(&results);
    let total_incomplete_score = total_incomplete_score(&results);
    (wrong_total_score, total_incomplete_score)
}

pub fn main() {
    let (wrong_total_score, total_incomplete_score) = calculate_scores(input::exercise());
    println!("Total wrong score: {}", wrong_total_score);
    println!("Total incomplete score: {:?}", total_incomplete_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (wrong_total_score, total_incomplete_score) = calculate_scores(input::example());
        assert_eq!(wrong_total_score, 26397);
        assert_eq!(total_incomplete_score, 182193);
    }

    #[test]
    fn exercise() {
        let (wrong_total_score, total_incomplete_score) = calculate_scores(input::exercise());
        assert_eq!(wrong_total_score, 318099);
        assert_eq!(total_incomplete_score, 2801771873);
    }
}
