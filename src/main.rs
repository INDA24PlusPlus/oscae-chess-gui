//! Basic hello world example.

use std::collections::HashMap;

use angun_chess::*;
use other_functions::*;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("oscae-chess-gui")
        .build();

    // Init
    let white_pieces: Vec<&str> = vec![ "wR1", "wK1", "wB1", "wKI", "wQU", "wB2", "wK2", "wR2", "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8" ];
    let black_pieces: Vec<&str> = vec![ "bR1", "bK1", "bB1", "bKI", "bQU", "bB2", "bK2", "bR2", "bP1", "bP2", "bP3", "bP4", "bP5", "bP6", "bP7", "bP8" ];

    // Content
    let mut assets = ChessAssets::new(&mut rl, &thread, "Classic", "Board - classic 2");
    
    let mut textures = HashMap::<&str, Texture2D>::new();
    //textures.insert(white_pieces[], white_king);

    while !rl.window_should_close() {
        
        // ------- Update ----------------------------------------
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();
        

        // ------- Draw ------------------------------------------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::CORNFLOWERBLUE);
        // board
        d.draw_texture_pro(&assets.board, Rectangle::new(0.0, 0.0, 288.0, 288.0), Rectangle::new(360.0, 360.0, 720.0, 720.0), Vector2::new(360.0, 360.0), 0.0, Color::WHITE);
        
        // pieces
        
        
        d.draw_circle(mouse_x, mouse_y, 5.0, Color::RED);
    }
}

struct ChessAssets {
    board: Texture2D,

    white_king: Texture2D,
    white_queen: Texture2D,
    white_bishop: Texture2D,
    white_knight: Texture2D,
    white_rook: Texture2D,
    white_pawn: Texture2D,

    black_king: Texture2D,
    black_queen: Texture2D,
    black_bishop: Texture2D,
    black_knight: Texture2D,
    black_rook: Texture2D,
    black_pawn: Texture2D,
}

impl ChessAssets {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread, theme: &str, board: &str) -> Self {
        let theme_path = format!("assets/{}/", theme);
        let white_piece_path = format!("{}Pieces/White/", theme_path);
        let black_piece_path = format!("{}Pieces/White/", theme_path);
        let board_path = format!("{}Board/{}.png", theme_path, board);

        Self {
            board: rl.load_texture(thread, &board_path).unwrap(),

            white_king: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "King").as_str()).unwrap(),
            white_queen: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "Queen").as_str()).unwrap(),
            white_bishop: rl.load_texture(&thread, format!("{}{}.png", &white_piece_path, "Bishop").as_str()).unwrap(),
            white_knight: rl.load_texture(&thread, format!("{}{}.png", &white_piece_path, "Knight").as_str()).unwrap(),
            white_rook: rl.load_texture(&thread, format!("{}{}.png", &white_piece_path, "Rook").as_str()).unwrap(),
            white_pawn: rl.load_texture(&thread, format!("{}{}.png", &white_piece_path, "Pawn").as_str()).unwrap(),
        
            black_king: rl.load_texture(&thread, format!("{}{}.png", &black_piece_path, "King").as_str()).unwrap(),
            black_queen: rl.load_texture(&thread, format!("{}{}.png", &black_piece_path, "Queen").as_str()).unwrap(),
            black_bishop: rl.load_texture(&thread, format!("{}{}.png", &black_piece_path, "Bishop").as_str()).unwrap(),
            black_knight: rl.load_texture(&thread, format!("{}{}.png", &black_piece_path, "Knight").as_str()).unwrap(),
            black_rook: rl.load_texture(&thread, format!("{}{}.png", &black_piece_path, "Rook").as_str()).unwrap(),
            black_pawn: rl.load_texture(&thread, format!("{}{}.png", &black_piece_path, "Pawn").as_str()).unwrap(),
        }
    }
}