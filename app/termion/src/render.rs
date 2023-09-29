use std::io::{stdout, Stdout, Write};

use snake::{
    game::Game,
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldPoint, Food, FoodType},
};
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen, ToAlternateScreen},
};

pub struct TermionRender {
    screen: AlternateScreen<RawTerminal<Stdout>>,
    tail: Option<SnakeNode>,
}

impl TermionRender {
    pub fn new() -> TermionRender {
        let stdout = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();
        let mut screen = AlternateScreen::from(stdout);
        write!(screen, "{}{}", termion::cursor::Hide, ToAlternateScreen).unwrap();
        TermionRender { screen, tail: None }
    }
    fn save_tail(&mut self, tail: SnakeNode) {
        self.tail = Some(tail);
    }
    fn clear_tail(&mut self) {
        if let Some(tail) = self.tail {
            write(" ", &tail, &mut self.screen);
        }
    }
    fn snake_tail(node: &SnakeNode) -> &str {
        return match node.direction {
            Direction::Up | Direction::Down => "Ꞌ",
            Direction::Right | Direction::Left => "-",
        };
    }

    fn snake_mounth_treat(node: &SnakeNode) -> &str {
        return match node.direction {
            Direction::Up => "v",
            Direction::Down => "ʌ",
            Direction::Right => "<",
            Direction::Left => ">",
        };
    }

    fn snake_mounth(node: &SnakeNode) -> &str {
        return match node.direction {
            Direction::Up => "⩀",
            Direction::Down => "⨃",
            Direction::Right => "⪾",
            Direction::Left => "⪽",
        };
    }
    fn replace_tail(&mut self, snake: &Snake) {
        self.clear_tail();

        let mut iter = snake.nodes.iter();
        let tail = iter.next().unwrap();
        let tail = SnakeNode {
            position: tail.position,
            direction: iter.next().unwrap().direction,
        };

        self.save_tail(tail.clone());

        write(TermionRender::snake_tail(&tail), &tail, &mut self.screen);
    }
    fn replace_head(&mut self, game: &Game) {
        let next_position = game.snake.next_head().position;
        let mut iter = game.snake.nodes.iter();
        let (head, neck) = (iter.next_back().unwrap(), iter.next_back().unwrap());
        let food_ahead = game.food.has_at(&next_position);
        let sprite = match food_ahead {
            true => TermionRender::snake_mounth_treat(&head),
            false => TermionRender::snake_mounth(&head),
        };
        write(sprite, &head, &mut self.screen);
        write("*", neck, &mut self.screen);
    }
}

impl GameRender for TermionRender {
    fn snake_full(&mut self, game: &Game) {
        let nodes = &game.snake.nodes;
        let mut iter = nodes.iter();
        let tail = iter.next().unwrap();

        self.save_tail(tail.clone());
        write(TermionRender::snake_tail(&tail), &tail, &mut self.screen);

        for _i in 1..nodes.len() - 1 {
            write("*", &iter.next().unwrap(), &mut self.screen);
        }
        let head = iter.next().unwrap();
        write(TermionRender::snake_mounth(&head), &head, &mut self.screen);
        self.screen.flush().unwrap();
    }

    fn snake(&mut self, game: &Game) {
        self.replace_tail(&game.snake);
        self.replace_head(game);
        self.screen.flush().unwrap();
    }
    fn eat(&mut self, game: &Game, food: &Food) {
        let clear = match food.shape {
            FoodType::Basic => " ",
            _ => "  ",
        };
        write_point(clear, &food.location, &mut self.screen);
        self.replace_head(game);
        self.screen.flush().unwrap();
    }
    fn added_food(&mut self, food: &Food) {
        let icon = match food.shape {
            FoodType::Basic => "@",
            _ => ":)",
        };
        write_point(icon, &food.location, &mut self.screen);
        self.screen.flush().unwrap();
    }

    fn removed_food(&mut self, food: &Food) {
        write_point("  ", &food.location, &mut self.screen);
        self.screen.flush().unwrap();
    }
}

const X_OFFSET: u16 = 2;
const Y_OFFSET: u16 = 2;

pub fn write(c: &str, node: &SnakeNode, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    write_point(c, &node.position, screen);
}

pub fn write_point(c: &str, point: &FieldPoint, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    let x = point.x as u16;
    let y = point.y as u16;
    write!(
        screen,
        "{}{}",
        termion::cursor::Goto(x + X_OFFSET, y + Y_OFFSET),
        c
    )
    .unwrap();
}
