use {
    crate::{Screen, Terminal},
    crossterm::event::{Event, KeyEvent, KeyModifiers, MouseEvent},
    ratatui::{layout::Position, prelude::*, widgets::Paragraph},
    std::{cell::RefCell, rc::Rc},
    termchess_common::TResult,
    termchess_engine::board::{Board, PieceColor},
};

pub mod board;

pub struct Game {
    board: Board,
    exit: bool,
    mouse_pos: Position,
    terminal: Rc<RefCell<Terminal>>,
}

impl Game {
    pub fn new(terminal: Rc<RefCell<Terminal>>) -> Self {
        Self {
            board: Board::new(),
            exit: false,
            mouse_pos: Position::default(),
            terminal,
        }
    }

    fn board_widget(&self) -> TResult<Paragraph> {
        // let board = self.board.print_to_string(&PieceColor::White)?;

        // Ok(Paragraph::new(Text::from(board)))

        todo!()
    }
}

#[allow(unused_variables)]
impl Screen for Game {
    fn render_frame(&mut self, frame: &mut Frame) -> TResult<()> {
        let layout = Layout::default();

        frame.render_widget(self.board_widget()?, frame.size());

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> TResult<()> {
        if key.modifiers == KeyModifiers::NONE {
            match key.code {
                _ => (),
            }
        } else if key.modifiers == KeyModifiers::CONTROL {
            match key.code {
                crossterm::event::KeyCode::Char('c') => self.exit = true,
                _ => (),
            }
        }

        Ok(())
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) -> TResult<()> {
        // TODO
        Ok(())
    }

    fn handle_misc(&mut self, event: Event) -> TResult<()> {
        Ok(())
    }

    fn terminal(&self) -> &Rc<RefCell<Terminal>> {
        &self.terminal
    }

    fn exit(&self) -> bool {
        self.exit
    }
}