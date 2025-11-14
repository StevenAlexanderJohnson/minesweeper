use axum::{extract::State as AxumState, Json};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::minesweeper;

#[derive(Clone)]
pub struct ApiCtx {
    pub app: tauri::AppHandle,
    pub game: Arc<Mutex<minesweeper::MinesweeperGame>>,
}

#[derive(Deserialize)]
pub struct NewGameRequest {
    difficulty: String,
}

#[derive(Deserialize)]
pub struct CellRequest {
    row: usize,
    col: usize,
}

use tauri::Emitter;

pub async fn http_get_state(
    AxumState(ctx): AxumState<ApiCtx>,
) -> Result<Json<minesweeper::DisplayBoard>, String> {
    let game = ctx.game.lock().unwrap();
    Ok(Json(game.get_display_board()))
}

pub async fn http_new_game(
    AxumState(ctx): AxumState<ApiCtx>,
    Json(payload): Json<NewGameRequest>,
) -> Result<Json<minesweeper::DisplayBoard>, String> {
    let board = {
        let difficulty_enum = match payload.difficulty.as_str() {
            "easy" => minesweeper::Difficulty::Easy,
            "medium" => minesweeper::Difficulty::Medium,
            "hard" => minesweeper::Difficulty::Hard,
            _ => return Err("Invalid difficulty".to_string()),
        };
        let mut game = ctx.game.lock().unwrap();
        *game = minesweeper::MinesweeperGame::new(difficulty_enum);
        game.get_display_board()
    };
    let _ = ctx.app.emit("board:update", &board);
    Ok(Json(board))
}

pub async fn http_flag_cell(
    AxumState(ctx): AxumState<ApiCtx>,
    Json(payload): Json<CellRequest>,
) -> Result<Json<minesweeper::DisplayBoard>, String> {
    let board = {
        let mut game = ctx.game.lock().unwrap();
        if game.game_state != minesweeper::GameState::Ongoing {
            return Err("Game is not ongoing".to_string());
        }
        game.flag_cell(payload.row, payload.col);
        game.validate_board();
        game.get_display_board()
    };
    let _ = ctx.app.emit("board:update", &board);
    Ok(Json(board))
}

pub async fn http_reveal_cell(
    AxumState(ctx): AxumState<ApiCtx>,
    Json(payload): Json<CellRequest>,
) -> Result<Json<minesweeper::DisplayBoard>, String> {
    let board = {
        let mut game = ctx.game.lock().unwrap();
        if game.game_state != minesweeper::GameState::Ongoing {
            return Err("Game is not ongoing".to_string());
        }
        game.reveal_cell(payload.row, payload.col);
        game.validate_board();
        game.get_display_board()
    };
    let _ = ctx.app.emit("board:update", &board);
    Ok(Json(board))
}
