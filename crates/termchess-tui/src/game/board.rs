use {
    super::Game,
    divrem::DivFloor,
    log::{debug, info},
    // ascii::Tile,
    ratatui::{
        style::{Color, Stylize},
        widgets::Paragraph,
    },
    std::error::Error,
    termchess_common::TResult,
    termchess_engine::board::{
        self,
        ascii::{HorizSide, VertSide},
        print::{BoardOptions, Size},
        Board, Co as ChessColor, Piece,
    },
};

impl Game {
    pub fn board_widget(&self) -> TResult<Paragraph> {
        let board = self.board.print(
            &self.board_options,
            ChessColor::White,
            self.mouse_over_board(),
            // Some((0, 7)),
        );

        Ok(Paragraph::new(board))
    }

    // TODO: use board::Position here
    pub fn mouse_over_board(&self) -> Option<(usize, usize)> {
        #[allow(unstable_name_collisions)]
        let pos = (
            ((self.mouse_pos.x as isize
                - self.board_pos.x as isize
                - self.board_options.border_width(HorizSide::Left) as isize)
                .div_floor(self.board_options.tile_width() as isize)),
            (self.mouse_pos.y as isize
                - self.board_pos.y as isize
                - self.board_options.border_height(VertSide::Top) as isize)
                .div_floor(self.board_options.tile_height() as isize),
        );

        if (pos.0 >= 0) && (pos.0 <= 7) && (pos.1 >= 0) && (pos.1 <= 7) {
            Some((pos.0 as usize, pos.1 as usize))
        } else {
            None
        }
    }
}
