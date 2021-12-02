mod input;

pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Default, PartialEq, Debug)]
struct State {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

pub fn main() {
    let commands = input::exercise();
    println!("Ex 01:");
    run(&commands, &execute_command_ex01);
    println!("Ex 02:");
    run(&commands, &execute_command_ex02);
}

fn run(commands: &Vec<Command>, f: &dyn Fn(State, &Command) -> State) {
    let state = execute_commands(&commands, &f);
    println!(
        "State = {:?}, res = {}",
        state,
        state.horizontal * state.depth
    );
}

fn execute_command_ex01(state: State, command: &Command) -> State {
    match command {
        &Command::Forward(x) => State {
            horizontal: state.horizontal + x,
            ..state
        },
        &Command::Down(x) => State {
            depth: state.depth + x,
            ..state
        },
        &Command::Up(x) => State {
            depth: state.depth - x,
            ..state
        },
    }
}

fn execute_command_ex02(state: State, command: &Command) -> State {
    match command {
        &Command::Forward(x) => State {
            horizontal: state.horizontal + x,
            depth: state.depth + state.aim * x,
            ..state
        },
        &Command::Down(x) => State {
            aim: state.aim + x,
            ..state
        },
        &Command::Up(x) => State {
            aim: state.aim - x,
            ..state
        },
    }
}

fn execute_commands(commands: &Vec<Command>, f: &dyn Fn(State, &Command) -> State) -> State {
    commands.iter().fold(State::default(), f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01_example() {
        let commands = input::example();
        let state = execute_commands(&commands, &execute_command_ex01);
        let expected = State {
            horizontal: 15,
            depth: 10,
            aim: 0,
        };
        assert_eq!(state, expected);
    }

    #[test]
    fn ex02_example() {
        let commands = input::example();
        let state = execute_commands(&commands, &execute_command_ex02);
        let expected = State {
            horizontal: 15,
            depth: 60,
            aim: 10,
        };
        assert_eq!(state, expected);
    }

    #[test]
    fn ex01() {
        let commands = input::exercise();
        let state = execute_commands(&commands, &execute_command_ex01);
        assert_eq!(state.horizontal * state.depth, 1989014);
    }

    #[test]
    fn ex02() {
        let commands = input::exercise();
        let state = execute_commands(&commands, &execute_command_ex02);
        assert_eq!(state.horizontal * state.depth, 2006917119);
    }
}
