//! Basic hello world example.

use oscae_chess::*;

use raylib::prelude::*;

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
    let mut game = Game::new();

    // Content
    let mut assets = ChessAssets::new(&mut rl, &thread, 2, 2);
    let circle = rl.load_texture(&thread, "assets/Circle.png").unwrap();
    let dot = rl.load_texture(&thread, "assets/Dot.png").unwrap();
    
    let mut notification = String::new();
    let mut notification_time = 0.0;

    let mut positions = Vec::<Square>::new();
    let mut selected_square = Square::from((-1, -1));

    while !rl.window_should_close() {
        
        // ------- Update ----------------------------------------
        
        // gui logic
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


        let square_x = ((mouse_x - board_left) / board_square_size).floor();
        let square_y = ((mouse_y - board_top) / board_square_size).floor();

        // chess logic
        if square_x >= 0.0 && square_x <= 7.0 && square_y >= 0.0 && square_y <= 7.0 {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {

                let square = Square::from((square_x as i8, 7 - square_y as i8));
                if !game.promotion {
                    if positions.contains(&square) {
                        game.do_move(&selected_square, &square);
                        positions.clear();
                        selected_square = Square::from((-1, -1));
                    } else {
                        selected_square = square;
                        positions = game.get_moves_list(&square);
                    }
                }
                else { // promotion
                    if square.x == game.last_moved_to.x {

                        let distance_to_pawn = if game.last_moved_to.y > square.y {
                            game.last_moved_to.y - square.y
                        } else {
                            square.y - game.last_moved_to.y
                        };

                        if square.y <= match game.last_moved_to.y { 0 => 4, 7 => 6, _ => -1 } &&
                            square.y >= match game.last_moved_to.y { 0 => 1, 7 => 3, _ => 100 } {

                            _ = match distance_to_pawn { // returns true if successful (play sound?)
                                1 => game.pawn_promotion(PieceType::Queen),
                                2 => game.pawn_promotion(PieceType::Rook),
                                3 => game.pawn_promotion(PieceType::Bishop),
                                4 => game.pawn_promotion(PieceType::Knight),
                                _ => false,
                            };
                        }
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
            Rectangle::new(window_width / 2.0, window_height / 2.0, board_size, board_size),
            Vector2::new(board_size / 2.0, board_size / 2.0), rotation, Color::WHITE);
        
        // pieces
        for (square, piece) in game.get_board_state() {

            let piece_asset = match piece.color {
                PieceColor::White => {
                    match piece.piece_type {
                        PieceType::King => &assets.white_king,
                        PieceType::Queen => &assets.white_queen,
                        PieceType::Bishop => &assets.white_bishop,
                        PieceType::Knight => &assets.white_knight,
                        PieceType::Rook => &assets.white_rook,
                        PieceType::Pawn => &assets.white_pawn,
                    }
                },
                PieceColor::Black => {
                    match piece.piece_type {
                        PieceType::King => &assets.black_king,
                        PieceType::Queen => &assets.black_queen,
                        PieceType::Bishop => &assets.black_bishop,
                        PieceType::Knight => &assets.black_knight,
                        PieceType::Rook => &assets.black_rook,
                        PieceType::Pawn => &assets.black_pawn,
                    }
                },
            };

            let source_rec = Rectangle::new(0.0, 0.0, asset_square_size, asset_square_size);
            d.draw_texture_pro(piece_asset, source_rec,
                Rectangle::new(
                    board_left + board_square_size * (square.x as f32 + 0.5),
                    board_top + board_square_size * ( 7.0 - square.y as f32 + 0.5),
                    board_square_size, board_square_size),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);
        }
        
        // positions
        for square in &positions {
            let pos_texture = if game.get_board_state().contains_key(square) {
                &circle
            } else {
                &dot
            };

            let source_rec = Rectangle::new(0.0, 0.0, asset_square_size, asset_square_size);
            d.draw_texture_pro(pos_texture, source_rec,
                Rectangle::new(
                    board_left + board_square_size * (square.x as f32 + 0.5),
                    board_top + board_square_size * (7.0 - square.y as f32 + 0.5),
                    board_square_size, board_square_size),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::new(128, 128, 128, 128));
        }

        // promotion
        if game.promotion {
            let (y, inc) = match game.last_moved_to.y {
                7 => (6, -1),
                0 => (4, 1),
                _ => (100, 0), // this should never happen
            };

            // draw outline
            d.draw_texture_pro(&assets.board, Rectangle::new(
                (assets.square_offset + assets.square_size) as f32,
                assets.square_offset as f32, 1.0, 1.0),
                Rectangle::new(
                    board_left + board_square_size * (game.last_moved_to.x as f32 + 0.5) - scale * 2.0,
                    board_top + board_square_size * (7.0 - y as f32 + 0.5) - scale * 2.0,
                    board_square_size + scale * 4.0, board_square_size * 4.0 + scale * 4.0),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);

            // draw inside outline
            d.draw_texture_pro(&assets.board, Rectangle::new(
                assets.square_offset as f32,
                assets.square_offset as f32, 1.0, 1.0),
                Rectangle::new(
                    board_left + board_square_size * (game.last_moved_to.x as f32 + 0.5),
                    board_top + board_square_size * (7.0 - y as f32 + 0.5),
                    board_square_size, board_square_size * 4.0),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);

            // draw inside
            d.draw_texture_pro(&assets.board, Rectangle::new(
                (assets.square_offset + 2) as f32,
                (assets.square_offset + 2) as f32, 1.0, 1.0),
                Rectangle::new(
                    board_left + board_square_size * (game.last_moved_to.x as f32 + 0.5) + scale * 2.0,
                    board_top + board_square_size * (7.0 - y as f32 + 0.5) + scale * 2.0,
                    board_square_size - scale * 4.0, board_square_size * 4.0 - scale * 4.0),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);

            // draw pieces
            let textures: [(&Texture2D, f32); 4] = match game.turn {
                PieceColor::White => [(&assets.white_queen, 1.0), (&assets.white_rook, 2.0), (&assets.white_bishop, 3.0), (&assets.white_knight, 4.0)],
                PieceColor::Black => [(&assets.black_queen, 6.0), (&assets.black_rook, 5.0), (&assets.black_bishop, 4.0), (&assets.black_knight, 3.0)],
            };
            let source_rec = Rectangle::new(0.0, 0.0, asset_square_size, asset_square_size);

            for (piece_texture, y) in textures {
                d.draw_texture_pro(piece_texture, source_rec,
                    Rectangle::new(
                        board_left + board_square_size * (game.last_moved_to.x as f32 + 0.5),
                        board_top + board_square_size * (y + 0.5),
                        board_square_size, board_square_size),
                    Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);
            }
        }

        // cursor
        if d.is_cursor_on_screen() {
            // highlight
            if square_x >= 0.0 && square_x <= 7.0 &&
            square_y >= 0.0 && square_y <= 7.0 {

                d.draw_rectangle_rec(Rectangle::new(board_left + square_x * board_square_size, board_top + square_y * board_square_size, board_square_size, board_square_size), Color::new(150, 150, 150, 100));
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
