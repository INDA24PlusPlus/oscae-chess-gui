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
    let white_piece_path = String::from("assets/Classic/Pieces/Chess - white classic/");
    let black_piece_path = String::from("assets/Classic/Pieces/Chess - black classic/");
    let white_king = rl.load_texture(&thread, format!("{}{}", &white_piece_path, "King.png").as_str()).unwrap();
    let white_queen = rl.load_texture(&thread, format!("{}{}", &white_piece_path, "Queen.png").as_str()).unwrap();
    let white_bishop = rl.load_texture(&thread, format!("{}{}", &white_piece_path, "Bishop.png").as_str()).unwrap();
    let white_knight = rl.load_texture(&thread, format!("{}{}", &white_piece_path, "Knight.png").as_str()).unwrap();
    let white_rook = rl.load_texture(&thread, format!("{}{}", &white_piece_path, "Rook.png").as_str()).unwrap();
    let white_pawn = rl.load_texture(&thread, format!("{}{}", &white_piece_path, "Pawn.png").as_str()).unwrap();
 
    let black_king = rl.load_texture(&thread, format!("{}{}", &black_piece_path, "King.png").as_str()).unwrap();
    let black_queen = rl.load_texture(&thread, format!("{}{}", &black_piece_path, "Queen.png").as_str()).unwrap();
    let black_bishop = rl.load_texture(&thread, format!("{}{}", &black_piece_path, "Bishop.png").as_str()).unwrap();
    let black_knight = rl.load_texture(&thread, format!("{}{}", &black_piece_path, "Knight.png").as_str()).unwrap();
    let black_rook = rl.load_texture(&thread, format!("{}{}", &black_piece_path, "Rook.png").as_str()).unwrap();
    let black_pawn = rl.load_texture(&thread, format!("{}{}", &black_piece_path, "Pawn.png").as_str()).unwrap();
    
    let mut textures = HashMap::<&str, Texture2D>::new();
    //textures.insert(white_pieces[], white_king);

    while !rl.window_should_close() {
        
        // Update
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();
        

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::CORNFLOWERBLUE);
        d.draw_circle(mouse_x, mouse_y, 5.0, Color::RED);
    }
}