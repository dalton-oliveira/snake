use crate::{
    render::GameRender,
    snake::{Snake, SnakeNode},
    types::{Direction, FieldElement, FieldPoint, GameState},
};
use rand::Rng;

#[derive(Debug)]
pub struct Game {
    pub food: Option<FieldPoint>,
    pub width: usize,
    pub height: usize,
    pub score: u16,
    pub snake: Snake,
    pub field: Vec<Vec<FieldElement>>,
    pub state: GameState,
}

pub struct GameConfig {
    pub size: usize,
    pub start: (usize, usize),
    pub dim: (usize, usize),
    pub direction: Direction,
}
impl Game {
    pub fn new(game_render: &mut impl GameRender, config: GameConfig) -> Game {
        let (width, height) = config.dim;
        let mut field: Vec<Vec<FieldElement>> = vec![vec![FieldElement::Empty; width]; height];
        let snake = Snake::new(&mut field, config);
        game_render.snake_full(&snake);
        return Game {
            food: None,
            score: 0,
            snake: snake,
            field: field,
            state: GameState::None,
            width,
            height,
        };
    }

    fn crawl(&mut self, game_render: &mut impl GameRender) {
        // @todo maybe move this to the snake
        let next_head = self.snake.next_head();
        let SnakeNode { position, .. } = next_head;
        match self.field[position.x][position.y] {
            FieldElement::Empty => {
                self.snake.nodes.push_back(next_head);
                self.field[position.x][position.y] = FieldElement::Snake;
                let tail = self.snake.nodes.pop_front().unwrap();
                self.field[tail.position.x][tail.position.y] = FieldElement::Empty;
                game_render.snake(self);
            }
            FieldElement::Treat => {
                //@todo sum points, check for game over
                self.snake.nodes.push_back(next_head);
                self.field[position.x][position.y] = FieldElement::Snake;
                self.score += 1;
                game_render.eat(self);
                self.add_food(game_render);
            }
            FieldElement::Snake => self.state = GameState::Over,
        }
    }

    pub fn add_food(&mut self, game_render: &mut impl GameRender) {
        let available = (self.width * self.height) - self.snake.nodes.len();
        let mut rng = rand::thread_rng();

        let rand_pos = rng.gen_range(0..available - 1);
        let mut pos = 0;

        for x in 0..self.field.len() {
            for y in 0..self.field[x].len() {
                if self.field[x][y] == FieldElement::Empty {
                    pos += 1;
                }
                if pos > rand_pos {
                    self.food = Some(FieldPoint { x, y });
                    game_render.food(&self.food.unwrap());
                    self.field[x][y] = FieldElement::Treat;
                    break;
                }
            }
            if pos > rand_pos {
                break;
            }
        }
    }

    pub fn tick(&mut self, game_render: &mut impl GameRender) {
        if self.state == GameState::Quit {
            return;
        }
        self.crawl(game_render);
    }
}
