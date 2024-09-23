//! Basic hello world example.

use angun_chess::*;
use other_functions::*;

use raylib::prelude::*;
use valid_moves::solves_check_move;

const BOARD_SIZE: usize = 8;

fn main() {
    //run_chess();

    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("oscae-chess-gui")
        .resizable()
        .build();

    rl.set_target_fps(60);

    // Init
    let mut chess_eninge = ChessEngine::new();
    

    // Content
    let mut assets = ChessAssets::new(&mut rl, &thread, 2, 2);
    let circle = rl.load_texture(&thread, "assets/Circle.png").unwrap();
    let dot = rl.load_texture(&thread, "assets/Dot.png").unwrap();
    
    let mut notification = String::new();
    let mut notification_time = 0.0;

    let mut positions = Vec::<(i32, i32)>::new();
    let mut selected_piece = String::new();

    while !rl.window_should_close() {
        
        // ------- Update ----------------------------------------



        let window_width = rl.get_screen_width() as f32;
        let window_height = rl.get_screen_height() as f32;

        let rotated = false;

        let asset_size = assets.size as f32;
        let asset_square_size = assets.square_size as f32;
        let asset_square_offset = assets.square_offset as f32;

        let scale = if window_width > window_height { window_height / asset_size } else { window_width / asset_size };

        let mouse_x = rl.get_mouse_x() as f32;
        let mouse_y = rl.get_mouse_y() as f32;
        let rotation = match rotated { true => 180.0, false => 0.0 };
        let board_size = if window_width > window_height { window_height } else { window_width };

        let board_square_size = asset_square_size * scale;
        let board_offset = asset_square_offset * scale;

        let window_height = if rl.is_key_released(KeyboardKey::KEY_R) {
            rl.set_window_size(window_width as i32, window_width as i32);
            window_width
        }
        else {
            window_height
        };

        //let fps = rl.get_fps();
        
        if notification_time > -1.0 {
            notification_time -= rl.get_frame_time();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            assets = assets.next_theme(&mut rl, &thread);
            notification = format!("Theme: {}", assets.theme_name);
            notification_time = 1.0;
        }


        let square_x = ((mouse_x - board_offset) / scale / asset_square_size).floor();
        let square_y = ((mouse_y - board_offset) / scale / asset_square_size).floor();

        if square_x >= 0.0 && square_x <= 7.0 && square_y >= 0.0 && square_y <= 7.0 {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let square = (square_x as i32, square_y as i32);
                if positions.contains(&square) {
                    chess_eninge.do_move(&selected_piece, square);
                    positions.clear();
                    selected_piece.clear();
                } else {
                    positions.clear();
                    selected_piece = chess_eninge.get_piece(square);
                    for pos in chess_eninge.get_valid_moves(&selected_piece) {
                        let index = letter_to_index(&pos);
                        positions.push((index.1, index.0));
                    }
                }

            }
        }
        // ------- Draw ------------------------------------------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::CORNFLOWERBLUE);

        // board
        d.draw_texture_pro(&assets.board,
            Rectangle::new(0.0, 0.0, asset_size, asset_size),
            Rectangle::new(window_width / 2.0, window_height / 2.0, board_size, board_size),
            Vector2::new(board_size / 2.0, board_size / 2.0), rotation, Color::WHITE);
        
        // pieces
        for y in 0..8 {
            for x in 0..8 {
                let piece_asset = match chess_eninge.board[y][x].as_str() {
                    "wKI" => &assets.white_king,
                    "wQU" => &assets.white_queen,
                    "wB1" | "wB2" => &assets.white_bishop,
                    "wK1" | "wK2" => &assets.white_knight,
                    "wR1" | "wR2" => &assets.white_rook,
                    "wP1" | "wP2" | "wP3" | "wP4" | "wP5" | "wP6" | "wP7" | "wP8" => &assets.white_pawn,
                    "bKI" => &assets.black_king,
                    "bQU" => &assets.black_queen,
                    "bB1" | "bB2" => &assets.black_bishop,
                    "bK1" | "bK2" => &assets.black_knight,
                    "bR1" | "bR2" => &assets.black_rook,
                    "bP1" | "bP2" | "bP3" | "bP4" | "bP5" | "bP6" | "bP7" | "bP8" => &assets.black_pawn,
                    _ => continue,
                };
                
                let source_rec = Rectangle::new(0.0, 0.0, asset_square_size, asset_square_size);
                d.draw_texture_pro(piece_asset, source_rec,
                    Rectangle::new(
                        window_width / 2.0 + board_square_size * (x as f32 - 3.5),
                        window_height / 2.0 + board_square_size * (y as f32 - 3.5),
                        board_square_size, board_square_size),
                    Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);
            }
        }
        
        // positions
        for pos in &positions {
            let pos_texture = if chess_eninge.board[pos.1 as usize][pos.0 as usize] == "   " {
                &dot
            } else {
                &circle
            };

            let source_rec = Rectangle::new(0.0, 0.0, asset_square_size, asset_square_size);
            d.draw_texture_pro(pos_texture, source_rec,
                Rectangle::new(
                    window_width / 2.0 + board_square_size * (pos.0 as f32 - 3.5),
                    window_height / 2.0 + board_square_size * (pos.1 as f32 - 3.5),
                    board_square_size, board_square_size),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::new(128, 128, 128, 128));
        }

        // cursor
        if d.is_cursor_on_screen() {
            // highlight
            if mouse_x > board_offset && mouse_x < board_offset + 8.0 * board_square_size &&
                mouse_y > board_offset && mouse_y < board_offset + 8.0 * board_square_size {

                d.draw_rectangle_rec(Rectangle::new(square_x * board_square_size + board_offset, square_y * board_square_size + board_offset, board_square_size, board_square_size), Color::new(150, 150, 150, 100));
            }

            // cursor
            d.draw_circle(mouse_x as i32, mouse_y as i32, 5.0, Color::RED);
        }

        // notification
        if notification_time > -1.0 {

            let topleft_x = asset_square_offset;
            let topleft_y = asset_square_offset + if notification_time < 0.0 { notification_time } else { 0.0 } * board_offset * 5.0;
            let font_size = (8.0 * scale) as i32;
            let width = d.measure_text(&notification, font_size) + (8.0 * scale) as i32;

            d.draw_rectangle_rec(Rectangle::new(topleft_x * scale, topleft_y * scale, width as f32, font_size as f32 + 8.0 * scale), Color::LIGHTGRAY);
            d.draw_rectangle_lines_ex(Rectangle::new(topleft_x * scale, topleft_y * scale, width as f32, font_size as f32 + 8.0 * scale), scale * 2.0, Color::GRAY);

            d.draw_text(&notification,
                ((topleft_x + 4.0) * scale) as i32,
                ((topleft_y + 4.0) * scale) as i32,
                font_size, Color::from(Color::BLACK))
        }
    }
}

struct ChessAssets {
    theme_name: String,
    theme: u8,
    board_type: u8,

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

    size: i32,
    square_size: i32,
    square_offset: i32,
}

impl ChessAssets {
    // board_type is from 0-3
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread, theme: u8, board_type: u8) -> Self {
        let themes = ["1-Bit", "Casual", "Classic", "GameBoy", "Ice v. Fire", "Matte",
            "Purple v. Green", "Red v. Black", "Red v. Blue", "Wooden", "Wooden 2", "Wooden 3"];

        let theme = theme as usize % themes.len();
        let theme_path = format!("assets/{}/", themes[theme]);
        let white_piece_path = format!("{}Pieces/White/", theme_path);
        let black_piece_path = format!("{}Pieces/Black/", theme_path);
        let board_path = format!("{}Boards/Board{}.png", theme_path, board_type);

        Self {
            theme_name: themes[theme].to_string(),
            theme: theme as u8,
            board_type: board_type,

            board: rl.load_texture(thread, &board_path).unwrap(),

            white_king: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "King").as_str()).unwrap(),
            white_queen: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "Queen").as_str()).unwrap(),
            white_bishop: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "Bishop").as_str()).unwrap(),
            white_knight: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "Knight").as_str()).unwrap(),
            white_rook: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "Rook").as_str()).unwrap(),
            white_pawn: rl.load_texture(thread, format!("{}{}.png", &white_piece_path, "Pawn").as_str()).unwrap(),
        
            black_king: rl.load_texture(thread, format!("{}{}.png", &black_piece_path, "King").as_str()).unwrap(),
            black_queen: rl.load_texture(thread, format!("{}{}.png", &black_piece_path, "Queen").as_str()).unwrap(),
            black_bishop: rl.load_texture(thread, format!("{}{}.png", &black_piece_path, "Bishop").as_str()).unwrap(),
            black_knight: rl.load_texture(thread, format!("{}{}.png", &black_piece_path, "Knight").as_str()).unwrap(),
            black_rook: rl.load_texture(thread, format!("{}{}.png", &black_piece_path, "Rook").as_str()).unwrap(),
            black_pawn: rl.load_texture(thread, format!("{}{}.png", &black_piece_path, "Pawn").as_str()).unwrap(),

            size: 288,
            square_size: 32,
            square_offset: 16,
        }
    }

    fn next_theme(&self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self::new(rl, thread, self.theme + 1, self.board_type)
    }
}


struct ChessEngine {
    board: Vec<Vec<String>>,
    whites_turn_to_move: bool,
}

impl ChessEngine {
    fn new() -> Self {
        // Create pieces
        let black_pieces: Vec<&str> = vec![
            "bR1", "bK1", "bB1", "bQU", "bKI", "bB2", "bK2", "bR2", 
            "bP1", "bP2", "bP3", "bP4", "bP5", "bP6", "bP7", "bP8"
        ];
        
        let white_pieces: Vec<&str> = vec![
            "wR1", "wK1", "wB1", "wQU", "wKI", "wB2", "wK2", "wR2", 
            "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8"
        ];

        // Set turn order
        let whites_turn_to_move = true;

        // Initilize board
        let mut board: Vec<Vec<String>> = vec![vec!["   ".to_string(); BOARD_SIZE]; BOARD_SIZE];
        for i in 0..8{
            board[0][i] = black_pieces.get(i)
            .expect("Piece at index {i} does not exist in white_pieces").to_string();
            board[1][i] = black_pieces.get(i + 8)
            .expect("Piece at index {i} does not exist in white_pieces").to_string();
        }

        for i in 0..8{
            board[7][i] = white_pieces.get(i)
            .expect("Piece at index {i} does not exist in white_pieces").to_string();
            board[6][i] = white_pieces.get(i + 8)
            .expect("Piece at index {i} does not exist in white_pieces").to_string();
        }

        Self {
            board,
            whites_turn_to_move,
        }
    }

    fn get_valid_pieces(&self) -> Vec<String> {
        //Checking which pieces can be moved
        
        let is_in_check = valid_moves::is_in_check(if self.whites_turn_to_move { "wKI" } else { "bKI" }, &self.board);

        valid_pieces(self.whites_turn_to_move, &self.board, is_in_check)
    }

    fn get_valid_moves(&self, piece_to_move: &String) -> Vec<String> {

        let avaliable_pieces = self.get_valid_pieces();
        
        let is_in_check = valid_moves::is_in_check(if self.whites_turn_to_move { "wKI" } else { "bKI" }, &self.board);
        
        if !avaliable_pieces.contains(piece_to_move) {
            return Vec::new();
        }

        //If check -> filter out positions that do not solve check
        let moves = valid_moves(piece_to_move.as_str(), &self.board);

        if is_in_check {
            moves.iter().filter(|m| solves_check_move(piece_to_move, &self.board, m.to_string())).cloned().collect::<Vec<String>>()
        } else {
            moves
        }
    }

    fn do_move(&mut self, piece: &String, to: (i32, i32)) -> bool {
        let to = index_to_letter((to.1, to.0));

        println!("{}", to);
        
        let valid_moves = self.get_valid_moves(piece);
        if !valid_moves.contains(&to) {
            return false;
        }

        //Moving piece
        let move_from = get_position_index(piece, &self.board);
        let move_to = letter_to_index(to.as_str());
        self.board[move_from.0 as usize][move_from.1 as usize] = "   ".to_string();
        self.board[move_to.0 as usize][move_to.1 as usize] = piece.clone();    
    
    
        //Change turns
        self.whites_turn_to_move = !self.whites_turn_to_move;

        true
    }

    fn get_piece(&self, (x, y): (i32, i32)) -> String {
        self.board[y as usize][x as usize].clone()
    }
}