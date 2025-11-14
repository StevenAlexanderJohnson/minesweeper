use axum::{
    routing::{get, post},
    Router,
    
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tauri::Manager;

mod api;
mod minesweeper;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_game_state(
    game: tauri::State<Arc<Mutex<minesweeper::MinesweeperGame>>>,
) -> minesweeper::DisplayBoard {
    let game = game.lock().unwrap();
    game.get_display_board()
}

#[tauri::command]
fn flag_cell(
    game: tauri::State<Arc<Mutex<minesweeper::MinesweeperGame>>>,
    row: usize,
    col: usize,
) -> minesweeper::DisplayBoard {
    if game.lock().unwrap().game_state != minesweeper::GameState::Ongoing {
        return game.lock().unwrap().get_display_board();
    }
    let mut game = game.lock().unwrap();
    game.flag_cell(row, col);
    game.validate_board();
    game.get_display_board()
}

#[tauri::command]
fn reveal_cell(
    game: tauri::State<Arc<Mutex<minesweeper::MinesweeperGame>>>,
    row: usize,
    col: usize,
) -> minesweeper::DisplayBoard {
    let mut game = game.lock().unwrap();
    if game.game_state != minesweeper::GameState::Ongoing {
        return game.get_display_board();
    }
    game.reveal_cell(row, col);
    game.validate_board();
    game.get_display_board()
}

#[tauri::command]
fn new_game(
    game: tauri::State<Arc<Mutex<minesweeper::MinesweeperGame>>>,
    difficulty: String,
) -> minesweeper::DisplayBoard {
    let mut game = game.lock().unwrap();
    let difficulty_enum = match difficulty.as_str() {
        "easy" => minesweeper::Difficulty::Easy,
        "medium" => minesweeper::Difficulty::Medium,
        "hard" => minesweeper::Difficulty::Hard,
        _ => minesweeper::Difficulty::Medium,
    };
    *game = minesweeper::MinesweeperGame::new(difficulty_enum);
    game.get_display_board()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(minesweeper::MinesweeperGame::new(
            minesweeper::Difficulty::Medium,
        ))))
        .setup(|app| {
            let game_state = app
                .state::<Arc<Mutex<minesweeper::MinesweeperGame>>>()
                .inner()
                .clone();
            let ctx = api::ApiCtx {
                app: app.handle().clone(),
                game: game_state,
            };
            tauri::async_runtime::spawn(async move {
                let router = Router::new()
                    .route("/api/get_game_state", get(api::http_get_state))
                    .route("/api/new_game", post(api::http_new_game))
                    .route("/api/flag_cell", post(api::http_flag_cell))
                    .route("/api/reveal_cell", post(api::http_reveal_cell))
                    .with_state(ctx);

                let addr: SocketAddr = "127.0.0.1:9091".parse().unwrap();
                println!("Starting server on {}", addr);
                let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
                axum::serve(listener, router).await.unwrap();
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            new_game,
            get_game_state,
            flag_cell,
            reveal_cell
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
