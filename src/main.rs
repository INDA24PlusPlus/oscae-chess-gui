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
    let mut assets = ChessAssets::new(&mut rl, &thread, 2, 3);
    let circle = rl.load_texture(&thread, "assets/Circle.png").unwrap();
    let dot = rl.load_texture(&thread, "assets/Dot.png").unwrap();
    
    let mut notification = String::new();
    let mut notification_time = 0.0;

    let mut positions = Vec::<(i32, i32)>::new();
    let mut selected_piece = String::new();
    let mut rotated = false;

    let mut pre_game = true;
    let mut pre_game_menu = UIBox {
        x: 0.0,
        y: 0.0,
        text: String::from("oscae-chess-gui"),
        font_size: 12,
        text_color: Color::BLACK,
        width: 120.0,
        height: 60.0,
        color: Color::LIGHTGRAY,
        outline_color: Color::BLACK,
        buttons: vec![UIButton { x: 0.0, y: 10.0, text: String::from("PLAY"), font_size: 20, text_color: Color::BLACK, width: 100.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY }],
    };
    let mut post_game_menu = UIBox {
        x: 0.0,
        y: 0.0,
        text: String::new(),
        font_size: 15,
        text_color: Color::BLACK,
        width: 120.0,
        height: 60.0,
        color: Color::LIGHTGRAY,
        outline_color: Color::BLACK,
        buttons: vec![UIButton { x: 0.0, y: 10.0, text: String::from("continue"), font_size: 20, text_color: Color::BLACK, width: 100.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY }],
    };

    while !rl.window_should_close() {
        
        // ------- Update ----------------------------------------

        // gui logic
        let window_width = rl.get_screen_width() as f32;
        let window_height = rl.get_screen_height() as f32;

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

        let center_x = window_width / 2.0;
        let center_y = window_height / 2.0;
        
        let board_left = (window_width - board_size) / 2.0 + board_offset;
        let board_top = (window_height - board_size) / 2.0 + board_offset;

        //let fps = rl.get_fps();
        
        if notification_time > -1.0 {
            notification_time -= rl.get_frame_time();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) {
            assets = assets.next_theme(&mut rl, &thread);
            notification = format!("Theme: {}", assets.theme_name);
            notification_time = 1.0;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            rotated = !rotated;
        }


        let square_x = ((mouse_x - board_left) / board_square_size).floor();
        let square_y = ((mouse_y - board_top) / board_square_size).floor();

        if pre_game {
            for button in &mut pre_game_menu.buttons {
                if button.is_hovered(Vector2::new(center_x + pre_game_menu.x, center_y + pre_game_menu.y), scale, mouse_x, mouse_y) {

                    button.color = Color::WHITE;

                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        pre_game = false;
                    }
                }
                else {
                    button.color = Color::LIGHTGRAY;
                }
            }
        }

        if chess_eninge.game_over {
            for button in &mut post_game_menu.buttons {
                if button.is_hovered(Vector2::new(center_x + post_game_menu.x, center_y + post_game_menu.y), scale, mouse_x, mouse_y) {

                    button.color = Color::WHITE;

                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        pre_game = true;
                        chess_eninge = ChessEngine::new();
                    }
                }
                else {
                    button.color = Color::LIGHTGRAY;
                }
            }
        }
        

        // chess logic
        if !pre_game && square_x >= 0.0 && square_x <= 7.0 && square_y >= 0.0 && square_y <= 7.0 {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                
                let square = (mirror(square_x as i32, rotated), mirror(square_y as i32, rotated));

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
        d.clear_background(Color::BLACK);

        // board
        d.draw_texture_pro(&assets.board,
            Rectangle::new(0.0, 0.0, asset_size, asset_size),
            Rectangle::new(center_x, center_y, board_size, board_size),
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
                    board_left + board_square_size * (mirror(x as i32, rotated) as f32 + 0.5),
                    board_top + board_square_size * (mirror(y as i32, rotated) as f32 + 0.5),
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
                    board_left + board_square_size * (mirror(pos.0, rotated) as f32 + 0.5),
                    board_top + board_square_size * (mirror(pos.1, rotated) as f32 + 0.5),
                    board_square_size, board_square_size),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::new(128, 128, 128, 128));
        }

        // cursor
        if d.is_cursor_on_screen() {
            // highlight
            if !pre_game && square_x >= 0.0 && square_x <= 7.0 &&
            square_y >= 0.0 && square_y <= 7.0 {

                d.draw_rectangle_rec(Rectangle::new(board_left + square_x * board_square_size, board_top + square_y * board_square_size, board_square_size, board_square_size), Color::new(150, 150, 150, 100));
            }

            // cursor
            //d.draw_circle(mouse_x as i32, mouse_y as i32, 5.0, Color::RED);
        }

        // menu
        if pre_game {
            pre_game_menu.draw(&mut d, Vector2::new(center_x, center_y), scale);
        }

        if chess_eninge.game_over {
            let t = match chess_eninge.whites_turn_to_move {
                false => "White won",
                true => "Black won",
            };

            post_game_menu.text = String::from(t);
            post_game_menu.draw(&mut d, Vector2::new(center_x, center_y), scale);
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

fn mirror(i: i32, mirror: bool) -> i32 {
    if mirror {
        7 - i
    } else {
        i
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
    game_over: bool,
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
        let game_over = false;

        Self {
            board,
            whites_turn_to_move,
            game_over,
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

        // calculate checkmate
        if self.get_valid_pieces().len() == 0 {
            self.game_over = true;
        }

        true
    }

    fn get_piece(&self, (x, y): (i32, i32)) -> String {
        self.board[y as usize][x as usize].clone()
    }
}

struct UIBox {
    x: f32,
    y: f32,

    text: String,
    font_size: i32,
    text_color: Color,
    width: f32,
    height: f32,
    color: Color,
    outline_color: Color,
    buttons: Vec<UIButton>
}

impl UIElement for UIBox {
    fn draw(&self, d: &mut RaylibDrawHandle, origin: Vector2, scale: f32) {
        d.draw_rectangle_rec(Rectangle::new(origin.x + (self.x - self.width / 2.0) * scale, origin.y + (self.y - self.height / 2.0) * scale, self.width * scale, self.height * scale), self.color);
        d.draw_rectangle_lines_ex(Rectangle::new(origin.x + (self.x - self.width / 2.0) * scale, origin.y + (self.y - self.height / 2.0) * scale, self.width * scale, self.height * scale), scale * 2.0, self.outline_color);
    
        let text_width = d.measure_text(&self.text, self.font_size) as f32;

        d.draw_text(&self.text,
            (origin.x + (self.x - text_width / 2.0) * scale) as i32,
            (origin.y + (self.y - self.height as f32 / 2.0) * scale) as i32,
            (self.font_size as f32 * scale) as i32, self.text_color);

        for button in &self.buttons {
            button.draw(d, origin + Vector2::new(self.x, self.y), scale);
        }
    }
}

struct UIButton {
    x: f32,
    y: f32,

    text: String,
    font_size: i32,
    text_color: Color,
    width: f32,
    height: f32,
    color: Color,
    outline_color: Color,
}

pub trait UIElement {
    fn draw(&self, d: &mut RaylibDrawHandle, origin: Vector2, scale: f32);
}

impl UIElement for UIButton {
    fn draw(&self, d: &mut RaylibDrawHandle, origin: Vector2, scale: f32) {
        d.draw_rectangle_rec(Rectangle::new(origin.x + (self.x - self.width / 2.0) * scale, origin.y + (self.y - self.height / 2.0) * scale, self.width * scale, self.height * scale), self.color);
        d.draw_rectangle_lines_ex(Rectangle::new(origin.x + (self.x - self.width / 2.0) * scale, origin.y + (self.y - self.height / 2.0) * scale, self.width * scale, self.height * scale), scale * 2.0, self.outline_color);
        
        let text_width = d.measure_text(&self.text, self.font_size) as f32;

        d.draw_text(&self.text,
            (origin.x + (self.x - text_width / 2.0) * scale) as i32,
            (origin.y + (self.y - self.font_size as f32 / 2.0) * scale) as i32,
            (self.font_size as f32 * scale) as i32, self.text_color);
    }
}

impl UIButton {
    fn is_hovered(&self, origin: Vector2, scale: f32, mouse_x: f32, mouse_y: f32) -> bool {
        let left = origin.x + (self.x - self.width / 2.0) * scale;
        let right = left + self.width * scale;

        let top = origin.y + (self.y - self.height / 2.0) * scale;
        let bottom = top + self.height * scale;
        
        left < mouse_x && mouse_x < right && top < mouse_y && mouse_y < bottom
    }
}