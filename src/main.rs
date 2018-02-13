extern crate orbclient;
extern crate orbtk;
extern crate rand;

use orbtk::{Rect, Window, WindowBuilder, Grid, Label, Button, Style};
use orbtk::traits::{Place, Text, Click};
use orbtk::theme::Theme;
use std::sync::Arc;
use rand::Rng;
use std::sync::mpsc::{channel, Receiver, Sender};

static MINES_THEME_CSS: &'static str = include_str!("theme.css");

enum SweeperCommand {
    ButtonClicked(u32),
    ResetClicked()
}

enum GameState {
    Started(),
    GameOver(),
    GameFinished(),
}


#[derive(Copy, Clone, Debug)]
pub struct Cell {
    mine: bool,
    revealed: bool,
    neighbours: u8,
}


pub struct Mines {
    window: Window,
    grid: Arc<Grid>,
    width: u8,
    num_mines: u8,
    button_reset: Arc<Button>,
    game_state: GameState,
    cells: Box<[Cell]>,
    rx: Receiver<SweeperCommand>,
    tx: Sender<SweeperCommand>,
}


impl Mines {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let mut window_builder = WindowBuilder::new(Rect::new(-1, -1, 360, 420), "Mines");
        let theme = Theme::parse(MINES_THEME_CSS);
        window_builder = window_builder.theme(theme);
        let window = window_builder.build();
        let width: u8 = 9;

        let mines_left_label = Label::new();
        mines_left_label.text("10")
            .size(40, 40)
            .position(10, 10)
            .text_offset(12, 13)
            .with_class("mines-left");
        // There is no right click yet, so this is hiddne
        // window.add(&mines_left_label);

        let button_reset = Button::new();
        let _tx = tx.clone();
        button_reset.text(":)")
            .size(60, 40)
            .position(150, 10)
            .text_offset(22, 13)
            .with_class("reset")
            .on_click(move |_,_| {
                _tx.send(SweeperCommand::ResetClicked()).unwrap();
            });
        window.add(&button_reset);


        let grid = Grid::new();
        grid.columns(width as usize).size(340, 340).position(0, 60);
        window.add(&grid);

        Mines {
            window: window,
            grid: grid,
            width: width,
            num_mines: 10,
            button_reset: button_reset,
            game_state: GameState::Started(),
            cells: Vec::new().into_boxed_slice(),
            rx: rx,
            tx: tx,
        }
    }

    pub fn redraw(&mut self) {
        self.grid.clear();

        {

            for (idx, cell) in self.cells.iter().enumerate() {

                if cell.revealed {
                    let label = Label::new();
                    label
                        .size(40, 40)
                        .text_offset(16, 14)
                        .with_class("revealed");

                    match cell.neighbours {
                        0 => {},
                        num => {
                            label.text(format!("{}", num))
                                .with_class(format!("number-{}", num));
                        }
                    }

                    if cell.mine {
                        label.text("x")
                            .with_class("mine");
                    }

                    self.grid.add(&label);

                } else {

                    let button = Button::new();
                    let tx = self.tx.clone();

                    button
                        .text("")
                        .size(40, 40)
                        .text_offset(16, 14)
                        .on_click(move |_, _| {
                            tx.send(SweeperCommand::ButtonClicked(idx as u32)).unwrap();
                        });
                    self.grid.add(&button);
                }
            }
        }
    }

    pub fn element_click(&mut self, position: u32) {


        let element = self.cells[position as usize];

        if element.revealed {
            return;
        }

        if element.mine {
            // Show all
            self.game_over();
            return;
        }

        if element.neighbours == 0 {
            // Uncover empty neighbours
            self.uncover_empty_neighbours(position);
        }

        self.cells[position as usize].revealed = true;

        {
            let mut revealed = 0;
            for cell in self.cells.iter() {
                if cell.revealed {
                    revealed += 1;
                }
            }
            if revealed == self.width * self.width - self.num_mines {
                self.game_finished();
            }
        }

        self.redraw();
    }

    pub fn game_finished(&mut self) {
        self.game_state = GameState::GameFinished();
        self.button_reset.text(":D");
    }

    pub fn game_over(&mut self) {
        self.game_state = GameState::GameOver();
        self.show_all();
        
        self.button_reset.text(":x");
    }

    pub fn show_all(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.revealed = true;
        }
        self.redraw();
    }

    pub fn uncover_empty_neighbours(&mut self, position: u32) {

        let mut to_check = vec![position as i32];
        let mut checked: Vec<i32> = Vec::new();
        let width = self.width as i32;
        let copy = self.cells.clone();

        while let Some(pos) = to_check.pop() {
            checked.push(pos);
            self.cells[pos as usize].revealed = true;

            {
                let not_top = || {
                    pos / width > 0
                };
                let not_left = || {
                    pos % width != 0
                };
                let not_right = || {
                    pos % width != width-1
                };
                let not_bottom = || {
                    pos / width < width -1
                };

                let b = to_check.clone();
                let mut uncover = |x| {
                    if !b.contains(&(x)) && !checked.contains(&(x)) {
                        to_check.push(x);
                    }
                };

                if copy[pos as usize].neighbours == 0 {
                    // Uncover all neighbors

                    // Check left
                    if not_left() {
                        uncover(pos -1);
                    }
                    // Check right
                    if not_right() {
                        uncover(pos +1);
                    }
                    // Check above
                    if not_top() {
                        uncover(pos - width);
                    }
                    // Check below
                    if not_bottom() {
                        uncover(pos + width);
                    }
                    // Check above left
                    if not_left() && not_top() {
                        uncover(pos - width -1);
                    }
                    // Check above right
                    if not_top() && not_right() {
                        uncover(pos - width +1);
                    }
                    // Check bottom left
                    if not_bottom() && not_left() {
                        uncover(pos + width -1);
                    }
                    // Check bottom right
                    if not_bottom() && not_right() {
                        uncover(pos + width +1);
                    }
                }
            }
        }
    }

    pub fn init(&mut self) {
        {
            self.cells = vec![Cell {
                mine: false,
                revealed: false,
                neighbours: 0,
            }; self.width as usize * self.width as usize].into_boxed_slice();

            let mut mines = 0;           
            let mut rng = rand::thread_rng();
            let max = self.width * self.width;
            while mines < self.num_mines {
                let pos = rng.gen::<usize>() % max as usize;
                if self.cells[pos].mine {
                    continue;
                }
                self.cells[pos].mine = true;
                mines += 1;
            }
        }

        {
            let width = self.width as usize;
            let copy = self.cells.clone();
            for (idx, cell) in self.cells.iter_mut().enumerate() {

                let not_top = || {
                    idx / width > 0
                };
                let not_left = || {
                    idx % width != 0
                };
                let not_right = || {
                    idx % width != width-1
                };
                let not_bottom = || {
                    idx / width < width -1
                };
                // Check left
                if not_left() {
                    if copy[idx - 1].mine {
                        cell.neighbours += 1;
                    }
                }
                // Check right
                if not_right() {
                    if copy[idx + 1].mine {
                        cell.neighbours += 1;
                    }
                }
                // Check above
                if not_top() {
                    if copy[idx - width].mine {
                        cell.neighbours += 1;
                    }
                }
                // Check below
                if not_bottom() {
                    if copy[idx + width].mine {
                        cell.neighbours += 1;
                    }
                }
                // Check above left
                if not_left() && not_top() {
                    if copy[idx -width -1].mine {
                        cell.neighbours += 1;
                    }
                }
                // Check above right
                if not_top() && not_right() {
                    if copy[idx - width +1].mine {
                        cell.neighbours += 1;
                    }
                }
                // Check bottom left
                if not_bottom() && not_left() {
                    if copy[idx + width -1].mine {
                        cell.neighbours += 1;
                    }
                }
                // Check bottom right
                if not_bottom() && not_right() {
                    if copy[idx + width +1].mine {
                        cell.neighbours += 1;
                    }
                }

            }
        }
        self.redraw();
    }

    pub fn exec(&mut self) {
        self.init();
        self.window.draw_if_needed();

        while self.window.running.get() {
            self.window.step();

            while let Ok(event) = self.rx.try_recv() {
                match event {
                    SweeperCommand::ButtonClicked(pos) => {
                        self.element_click(pos);
                    },
                    SweeperCommand::ResetClicked() => {
                        self.init();
                        self.button_reset.text(":)");
                    }
                }
            }
            self.window.draw_if_needed();
        }
    }
}

fn main(){
    Mines::new().exec();
}
