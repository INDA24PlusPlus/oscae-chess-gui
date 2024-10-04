//! Basic hello world example.

use angun_chess::*;
use other_functions::*;
use chess_networking::*;
use ffi::GetRandomValue;


use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use raylib::prelude::*;

fn main() {
    //run_chess();

    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("oscae-chess-gui")
        .resizable()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    rl.set_target_fps(60);

    // Init
    let mut game = Game::new();

    // Content
    let mut assets = ChessAssets::new(&mut rl, &thread, 2, 3);
    let circle = rl.load_texture(&thread, "assets/Circle.png").unwrap();
    let dot = rl.load_texture(&thread, "assets/Dot.png").unwrap();
    
    let mut notification = String::new();
    let mut notification_time = 0.0;

    let mut positions = Vec::<(i32, i32)>::new();
    let mut selected_square = Square::from((-1, -1));
    let mut rotated = false;

    let mut pre_game = true;
    let mut addr_text = String::from("127.0.0.1:8787");
    let mut addr_input_active = false;
    let mut show_connect_menu = false;
    let mut pre_game_menu = UIBox {
        x: 0.0,
        y: 0.0,
        text: String::from("oscae-chess-gui"),
        font_size: 12,
        text_color: Color::BLACK,
        width: 160.0,
        height: 120.0,
        color: Color::LIGHTGRAY,
        outline_color: Color::BLACK,
        buttons: vec![
            UIButton { x: 0.0, y: -30.0, text: String::from("PLAY"), font_size: 20, text_color: Color::BLACK, width: 120.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY },
            UIButton { x: 0.0, y: 4.0, text: String::from("HOST"), font_size: 20, text_color: Color::BLACK, width: 120.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY },
            UIButton { x: 0.0, y: 38.0, text: String::from("JOIN"), font_size: 20, text_color: Color::BLACK, width: 120.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY },
        ],
    };
    let mut connect_menu = UIBox {
        x: 0.0,
        y: 0.0,
        text: String::from("Connect"),
        font_size: 12,
        text_color: Color::BLACK,
        width: 160.0,
        height: 120.0,
        color: Color::LIGHTGRAY,
        outline_color: Color::BLACK,
        buttons: vec![
            UIButton { x: 0.0, y: -30.0, text: addr_text.clone(), font_size: 20, text_color: Color::BLACK, width: 120.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY },
            UIButton { x: 0.0, y: 4.0, text: String::from("CONNECT"), font_size: 20, text_color: Color::BLACK, width: 120.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY },
            UIButton { x: 0.0, y: 38.0, text: String::from("BACK"), font_size: 20, text_color: Color::BLACK, width: 120.0, height: 30.0, color: Color::WHITE, outline_color: Color::DARKGRAY },
        ],
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
    

    // networking
    let mut chess_server: Option<ChessServer> = None;
    let mut chess_client: Option<ChessClient> = None;

    let mut opponent_name = String::new();
    let mut was_offered_draw = false;
    let mut promotion_network = false;
    let mut promotion_x = -1;

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

        // pre game menu logic
        if pre_game && !show_connect_menu {
            for button in &mut pre_game_menu.buttons {
                if button.is_hovered(Vector2::new(center_x + pre_game_menu.x, center_y + pre_game_menu.y), scale, mouse_x, mouse_y) {

                    button.color = Color::WHITE;

                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        chess_server = None;
                        chess_client = None;
                        match button.text.as_str() {
                            "PLAY" => pre_game = false,
                            "HOST" => chess_server = ChessServer::new(&String::from("127.0.0.1"), 8787, Some(String::from("Oscar Server"))),
                            "JOIN" => {
                                show_connect_menu = true;
                            }
                            _ => (),
                        }
                    }
                }
                else {
                    button.color = Color::LIGHTGRAY;
                }
            }
        } else if show_connect_menu {
            match connect_menu.buttons.get_mut(0) {
                Some(address_field) => address_field.text = addr_text.clone(),
                None => (),
            }

            for button in &mut connect_menu.buttons {
                if button.is_hovered(Vector2::new(center_x + pre_game_menu.x, center_y + pre_game_menu.y), scale, mouse_x, mouse_y) {

                    button.color = Color::WHITE;

                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        addr_input_active = false;

                        match button.text.as_str() {
                            "CONNECT" => {
                                chess_client = ChessClient::new(&addr_text, Some(String::from("Oscar Client")));
                                if let Some(client) = &mut chess_client {
                                    pre_game = false;
                                    show_connect_menu = false;
                                    client.send_start();
                                }
                            }
                            "BACK" => show_connect_menu = false,
                            _ => {
                                addr_input_active = true;
                            },
                        }
                    }
                }
                else {
                    button.color = Color::LIGHTGRAY;
                }
            }

            if addr_input_active {
                if let Some(key) = rl.get_key_pressed() {
                    match key {
                        KeyboardKey::KEY_BACKSPACE => _ = addr_text.pop(),
                        KeyboardKey::KEY_ENTER => {
                            chess_client = ChessClient::new(&addr_text, Some(String::from("Oscar Client")));
                            if let Some(client) = &mut chess_client {
                                pre_game = false;
                                show_connect_menu = false;
                                client.send_start();
                            }
                        },
                        KeyboardKey::KEY_LEFT_SHIFT | KeyboardKey::KEY_RIGHT_SHIFT => (),
                        KeyboardKey::KEY_PERIOD => {
                            match rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                                true => addr_text.push(':'),
                                false => addr_text.push('.'),
                            }
                        }
                        _ => {
                            let key = key as u8 as char;
                            addr_text.push(key.to_ascii_lowercase());
                        }
                    }
                }
            }
        }

        // post game menu logic
        if game.result != ChessResult::Ongoing {
            for button in &mut post_game_menu.buttons {
                if button.is_hovered(Vector2::new(center_x + post_game_menu.x, center_y + post_game_menu.y), scale, mouse_x, mouse_y) {

                    button.color = Color::WHITE;

                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        pre_game = true;
                        game = Game::new();
                    }
                }
                else {
                    button.color = Color::LIGHTGRAY;
                }
            }
        }
        
        // network phases
        let mut can_move = true;
        if let Some(server) = &mut chess_server {
            can_move = false;
            match server.network_phase {
                NetworkPhase::NoConnection => server.listen(), // listen for incoming connection
                NetworkPhase::FoundConnection => { // a connection was established
                    pre_game = false;
                    server.network_phase = NetworkPhase::Start
                },
                NetworkPhase::Start => { // listen for start message and then reply
                    if let Some(start) = server.receive_start() {
                        if let Some(op_name) = start.name {
                            opponent_name = op_name;
                            println!("Opponent name: {opponent_name}");
                        }

                        server.own_color = match unsafe { GetRandomValue(0, 1) } == 0 {
                            true => PieceColor::White,
                            false => PieceColor::Black,
                        };
                        server.send_start(Some(game.to_fen()));

                        server.network_phase = NetworkPhase::Move;

                        rotated = server.own_color == PieceColor::Black;

                        println!("My color: {}", match server.own_color {
                            PieceColor::White => "White",
                            PieceColor::Black => "Black",
                        });
                    }
                },
                NetworkPhase::Move => { // make a move or listen for one
                    if was_offered_draw {
                        notification = String::from(format!("{opponent_name} offers a draw. Accept? (Y/N)"));
                        notification_time = 0.0;
                        if rl.is_key_pressed(KeyboardKey::KEY_Y) {
                            server.send_ack(true, Some(GameState::Draw));
                            game.declare_draw();
                            server.network_phase = NetworkPhase::GameOver;
                            was_offered_draw = false;
                        } else if rl.is_key_pressed(KeyboardKey::KEY_N) {
                            server.send_ack(false, None);
                            was_offered_draw = false;
                        }
                    }

                    if game.turn != server.own_color {
                        if let Some(pmove) = server.receive_move() { // listen
                            // Receive a move
                            println!("Move recevied:\n From: ({}, {})\n To: ({}, {})\n Forfeit: {}\n Offer draw: {}", pmove.from.0, pmove.from.1, pmove.to.0, pmove.to.1, pmove.forfeit, pmove.offer_draw);

                            if pmove.forfeit {
                                game.declare_win(server.own_color);
                                server.send_ack(true, Some(GameState::CheckMate));
                                server.network_phase = NetworkPhase::GameOver;
                            } else if pmove.offer_draw {
                                was_offered_draw = true;
                            } else if game.do_move(&to_square(&pmove.from), &to_square(&pmove.to)) {
                                if game.promotion {
                                    match pmove.promotion {
                                        Some(promotion) => _ = game.pawn_promotion(match promotion {
                                            PromotionPiece::Queen => PieceType::Queen,
                                            PromotionPiece::Bishop => PieceType::Bishop,
                                            PromotionPiece::Knight => PieceType::Knight,
                                            PromotionPiece::Rook => PieceType::Rook,
                                        }),
                                        None => server.send_ack(false, None),
                                    }
                                }
                                if !game.promotion {
                                    server.send_ack(true, match game.result {
                                        ChessResult::Ongoing => None,
                                        ChessResult::WhiteWon | ChessResult::BlackWon => Some(GameState::CheckMate),
                                        ChessResult::Draw => Some(GameState::Draw),
                                    });
                                }
                            } else {
                                server.send_ack(false, match game.result {
                                    ChessResult::Ongoing => None,
                                    ChessResult::WhiteWon | ChessResult::BlackWon => Some(GameState::CheckMate),
                                    ChessResult::Draw => Some(GameState::Draw),
                                });
                            }
                        }
                    } else { // own turn
                        if rl.is_key_pressed(KeyboardKey::KEY_O) { 
                            // offer draw
                            server.send_move((0, 0), (0, 0), None, false, true);
                        } else if rl.is_key_pressed(KeyboardKey::KEY_P) {
                            // forfeit
                            server.send_move((0, 0), (0, 0), None, true, false);
                        }
                    }
                    can_move = game.turn == server.own_color; // allow the making of moves if it is own turn
                }
                NetworkPhase::Ack => { // listen for an ack response after a move was made
                    if let Some(ack) = server.receive_ack() {
                        // ack received
                        println!("Ack recevied:\n Ok: {}\n End state: {}", ack.ok, match ack.end_state { Some(end_state) => match end_state { GameState::CheckMate => "Checkmate", GameState::Draw => "Draw", }, None => "None" });

                        if let Some(saved_move) = &server.saved_move {
                            if saved_move.forfeit { // forfeit
                                game.declare_win(!server.own_color);
                            } else if saved_move.offer_draw { // offer draw
                                if ack.ok {
                                    // do draw
                                    game.declare_draw();
                                    server.saved_move = None;
                                }
                                // else don't do draw
                            } else { // move!
                                if ack.ok {
                                    // do move
                                    complete_move(&mut game, saved_move);
                                    server.saved_move = None;
                                } else {
                                    // server is boss: do move anyway
                                    complete_move(&mut game, saved_move);
                                    server.saved_move = None;
                                }
                            }
                        }

                        server.network_phase = match game.result {
                            ChessResult::Ongoing => NetworkPhase::Move,
                            _ => NetworkPhase::GameOver,
                        }
                    }
                }
                NetworkPhase::GameOver => chess_server = None,
            }
        } else if let Some(client) = &mut chess_client {
            can_move = false;
            match client.network_phase {
                NetworkPhase::NoConnection => (), // will literaly never happen
                NetworkPhase::FoundConnection => client.send_start(), // send start when a connection is established, will set phase to Start
                NetworkPhase::Start => { // listen for start
                    if let Some(start) = client.receive_start() {
                        if let Some(op_name) = start.name {
                            opponent_name = op_name;
                            println!("Opponent name: {opponent_name}");
                        }
                        client.own_color = if start.is_white { PieceColor::White } else { PieceColor::Black };

                        if let Some(fen) = start.fen {
                            game = Game::from_fen(&fen);
                        }

                        client.network_phase = NetworkPhase::Move;

                        rotated = client.own_color == PieceColor::Black;

                        println!("My color: {}", match client.own_color {
                            PieceColor::White => "White",
                            PieceColor::Black => "Black",
                        });
                    }
                },
                NetworkPhase::Move => { // make move or listen for one depending on turn
                    if was_offered_draw {
                        notification = String::from(format!("{opponent_name} offers a draw. Accept? (Y/N)"));
                        notification_time = 0.0;
                        if rl.is_key_pressed(KeyboardKey::KEY_Y) {
                            client.send_ack(true, Some(GameState::Draw));
                            game.declare_draw();
                            client.network_phase = NetworkPhase::GameOver;
                            was_offered_draw = false;
                        } else if rl.is_key_pressed(KeyboardKey::KEY_N) {
                            client.send_ack(false, None);
                            was_offered_draw = false;
                        }
                    }

                    if game.turn != client.own_color {
                        if let Some(pmove) = client.receive_move() { // listen
                            // move received
                            println!("Move recevied:\n From: ({}, {})\n To: ({}, {})\n Forfeit: {}\n Offer draw: {}", pmove.from.0, pmove.from.1, pmove.to.0, pmove.to.1, pmove.forfeit, pmove.offer_draw);

                            if pmove.forfeit {
                                game.declare_win(client.own_color);
                                client.send_ack(true, Some(GameState::CheckMate));
                                client.network_phase = NetworkPhase::GameOver;
                            } else if pmove.offer_draw {
                                was_offered_draw = true;
                            } else if game.do_move(&to_square(&pmove.from), &to_square(&pmove.to)) {
                                if game.promotion {
                                    match pmove.promotion {
                                        Some(promotion) => _ = game.pawn_promotion(match promotion {
                                            PromotionPiece::Queen => PieceType::Queen,
                                            PromotionPiece::Bishop => PieceType::Bishop,
                                            PromotionPiece::Knight => PieceType::Knight,
                                            PromotionPiece::Rook => PieceType::Rook,
                                        }),
                                        None => client.send_ack(false, None),
                                    }
                                }
                                if !game.promotion {
                                    client.send_ack(true, match game.result {
                                        ChessResult::Ongoing => None,
                                        ChessResult::WhiteWon | ChessResult::BlackWon => Some(GameState::CheckMate),
                                        ChessResult::Draw => Some(GameState::Draw),
                                    });
                                }
                            } else {
                                client.send_ack(false, match game.result {
                                    ChessResult::Ongoing => None,
                                    ChessResult::WhiteWon | ChessResult::BlackWon => Some(GameState::CheckMate),
                                    ChessResult::Draw => Some(GameState::Draw),
                                });
                            }
                        }
                    } else { // own turn
                        if rl.is_key_pressed(KeyboardKey::KEY_O) { 
                            // offer draw
                            client.send_move((0, 0), (0, 0), None, false, true);
                        } else if rl.is_key_pressed(KeyboardKey::KEY_P) {
                            // forfeit
                            client.send_move((0, 0), (0, 0), None, true, false);
                        }
                    }
                    can_move = game.turn == client.own_color; // allow the making of moves if it is own turn
                },
                NetworkPhase::Ack => { // listen for an ack response after a move was made
                    if let Some(ack) = client.receive_ack() {
                        // ack received
                        println!("Ack recevied:\n Ok: {}\n End state: {}", ack.ok, match ack.end_state { Some(end_state) => match end_state { GameState::CheckMate => "Checkmate", GameState::Draw => "Draw", }, None => "None" });

                        if let Some(saved_move) = &client.saved_move {
                            if saved_move.forfeit { // forfeit
                                game.declare_win(!client.own_color);
                                client.saved_move = None;
                            } else if saved_move.offer_draw { // offer draw
                                if ack.ok {
                                    // do draw
                                    game.declare_draw();
                                    client.saved_move = None;
                                }
                                // else don't do draw
                            } else { // move!
                                if ack.ok {
                                    // do move
                                    complete_move(&mut game, saved_move);
                                    client.saved_move = None;
                                } else {
                                    // server is boss: don't do move
                                    client.saved_move = None;
                                }
                            }
                        }

                        client.network_phase = match game.result {
                            ChessResult::Ongoing => NetworkPhase::Move,
                            _ => NetworkPhase::GameOver,
                        }
                    }
                }
                NetworkPhase::GameOver => chess_client = None,
            }
        }

        // chess logic
        // if clicked on the board
        if !pre_game && can_move && square_x >= 0.0 && square_x <= 7.0 && square_y >= 0.0 && square_y <= 7.0 &&
            rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                
            let square = Square::from((mirror(square_x as i8, rotated), mirror(square_y as i8, !rotated)));
            if !game.promotion && !promotion_network {
                if positions.contains(&square) {
                    if let Some(server) = chess_server.as_mut() {   // server
                        let (_, promotion, legal) = game.try_move(&selected_square, &square);

                        if legal {
                            if promotion {
                                promotion_network = true;
                                promotion_x = square.x;
                                server.saved_move = Some(Move { 
                                    from: (selected_square.x as u8, selected_square.y as u8),
                                    to: (square.x as u8, square.y as u8),
                                    promotion: None, forfeit: false, offer_draw: false });
                            } else {
                                server.send_move(
                                    (selected_square.x as u8, selected_square.y as u8),
                                    (square.x as u8, square.y as u8), None, false, false);
                            }
                        }
                    } else if let Some(client) = chess_client.as_mut() { // client
                        let (_, promotion, legal) = game.try_move(&selected_square, &square);

                        if legal {
                            if promotion {
                                promotion_network = true;
                                promotion_x = square.x;
                                client.saved_move = Some(Move { 
                                    from: (selected_square.x as u8, selected_square.y as u8),
                                    to: (square.x as u8, square.y as u8),
                                    promotion: None, forfeit: false, offer_draw: false });
                            } else {
                                client.send_move(
                                    (selected_square.x as u8, selected_square.y as u8),
                                    (square.x as u8, square.y as u8), None, false, false);
                            }
                        }

                    } else { // offline
                        game.do_move(&selected_square, &square);
                    }
                    positions.clear();
                    selected_square = Square::from((-1, -1));
                } else {
                    selected_square = square;
                    positions = game.get_moves_list(&square);
                }
            }
            else if square.x == game.last_moved_to.x || promotion_network && square.x == promotion_x { // promotion && clicked on piece in promotion window

                let y = match game.turn {
                    PieceColor::White => 7,
                    PieceColor::Black => 0,
                };

                let distance_to_pawn = if y > square.y {
                    y - square.y
                } else {
                    square.y - y
                };

                if square.y <= match y { 0 => 4, 7 => 6, _ => -1 } &&
                    square.y >= match y { 0 => 1, 7 => 3, _ => 100 } {
                    
                    let promotion_piece = match distance_to_pawn { // returns true if successful (play sound?)
                        1 => Some(PieceType::Queen),
                        2 => Some(PieceType::Rook),
                        3 => Some(PieceType::Bishop),
                        4 => Some(PieceType::Knight),
                        _ => None, // Invalid
                    };

                    if let Some(promotion_piece) = promotion_piece {
                        // play sound?
                        // do promotion
                        if promotion_network { // networked promotion
                            let promotion_piece = match promotion_piece {
                                PieceType::Queen => Some(PromotionPiece::Queen),
                                PieceType::Bishop => Some(PromotionPiece::Bishop),
                                PieceType::Knight => Some(PromotionPiece::Knight),
                                PieceType::Rook => Some(PromotionPiece::Rook),
                                _ => Some(PromotionPiece::Queen),
                            };

                            if let Some(server) = chess_server.as_mut() {
                                if let Some(saved_move) = &server.saved_move {
                                    server.send_move(saved_move.from, saved_move.to, promotion_piece, saved_move.forfeit, saved_move.offer_draw)
                                }
                            } else if let Some(client) = chess_client.as_mut() {
                                if let Some(saved_move) = &client.saved_move {
                                    client.send_move(saved_move.from, saved_move.to, promotion_piece, saved_move.forfeit, saved_move.offer_draw)
                                }
                            }
                            promotion_network = false;
                            promotion_x = -1;
                        }
                        else {  // offline
                            game.pawn_promotion(promotion_piece);
                        }
                    }
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_D) {
            let mut phase = "x";
            if let Some(server) = &chess_server {
                phase = match server.network_phase {
                    NetworkPhase::NoConnection => "NoConnection",
                    NetworkPhase::FoundConnection => "FoundConnection",
                    NetworkPhase::Start => "Start",
                    NetworkPhase::Move => "Move",
                    NetworkPhase::Ack => "Ack",
                    NetworkPhase::GameOver => "GameOver",
                }
            }
            if let Some(client) = &chess_client {
                phase = match client.network_phase {
                    NetworkPhase::NoConnection => "NoConnection",
                    NetworkPhase::FoundConnection => "FoundConnection",
                    NetworkPhase::Start => "Start",
                    NetworkPhase::Move => "Move",
                    NetworkPhase::Ack => "Ack",
                    NetworkPhase::GameOver => "GameOver",
                }
            }
            let turn = match game.turn {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            };
            
            println!("DEBUG INFO:\n Phase: {phase}\n Turn: {turn}\n ",);
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
        for square in &positions {
            let pos_texture = if game.get_board_state().contains_key(square) {
                &circle
            } else {
                &dot
            };

            let source_rec = Rectangle::new(0.0, 0.0, asset_square_size, asset_square_size);
            d.draw_texture_pro(pos_texture, source_rec,
                Rectangle::new(
                    board_left + board_square_size * (mirror(square.x, rotated) as f32 + 0.5),
                    board_top + board_square_size * (mirror(square.y, !rotated) as f32 + 0.5),
                    board_square_size, board_square_size),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::new(128, 128, 128, 128));
        }

        // promotion
        if game.promotion || promotion_network {
            let y = match game.turn == PieceColor::White && !rotated || game.turn == PieceColor::Black && rotated {
                true => 6,
                false => 4,
            };

            let x = if let Some(server) = &chess_server {
                if let Some(saved_move) = &server.saved_move {
                    saved_move.to.0 as i8
                }
                else {
                    game.last_moved_to.x
                }
            } else if let Some(client) = &chess_client {
                if let Some(saved_move) = &client.saved_move {
                    saved_move.to.0 as i8
                } else {
                    game.last_moved_to.x
                }
            } else { game.last_moved_to.x };

            // draw outline
            d.draw_texture_pro(&assets.board, Rectangle::new(
                (assets.square_offset + assets.square_size) as f32,
                assets.square_offset as f32, 1.0, 1.0),
                Rectangle::new(
                    board_left + board_square_size * (mirror(x, rotated) as f32 + 0.5) - scale * 2.0,
                    board_top + board_square_size * (mirror(y, true) as f32 + 0.5) - scale * 2.0,
                    board_square_size + scale * 4.0, board_square_size * 4.0 + scale * 4.0),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);

            // draw inside outline
            d.draw_texture_pro(&assets.board, Rectangle::new(
                assets.square_offset as f32,
                assets.square_offset as f32, 1.0, 1.0),
                Rectangle::new(
                    board_left + board_square_size * (mirror(x, rotated) as f32 + 0.5),
                    board_top + board_square_size * (mirror(y, true) as f32 + 0.5),
                    board_square_size, board_square_size * 4.0),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);

            // draw inside
            d.draw_texture_pro(&assets.board, Rectangle::new(
                (assets.square_offset + 2) as f32,
                (assets.square_offset + 2) as f32, 1.0, 1.0),
                Rectangle::new(
                    board_left + board_square_size * (mirror(x, rotated) as f32 + 0.5) + scale * 2.0,
                    board_top + board_square_size * (mirror(y, true) as f32 + 0.5) + scale * 2.0,
                    board_square_size - scale * 4.0, board_square_size * 4.0 - scale * 4.0),
                Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);

            // draw pieces
            let textures: [(&Texture2D, i8); 4] = match game.turn {
                PieceColor::White => [(&assets.white_queen, 1), (&assets.white_rook, 2), (&assets.white_bishop, 3), (&assets.white_knight, 4)],
                PieceColor::Black => [(&assets.black_queen, 6), (&assets.black_rook, 5), (&assets.black_bishop, 4), (&assets.black_knight, 3)],
            };
            let source_rec = Rectangle::new(0.0, 0.0, asset_square_size, asset_square_size);

            for (piece_texture, y) in textures {
                d.draw_texture_pro(piece_texture, source_rec,
                    Rectangle::new(
                        board_left + board_square_size * (mirror(x, rotated) as f32 + 0.5),
                        board_top + board_square_size * (mirror(y, rotated) as f32 + 0.5),
                        board_square_size, board_square_size),
                    Vector2::new(board_square_size / 2.0, board_square_size / 2.0), 0.0, Color::WHITE);
            }
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

        // pre game menu
        if pre_game && !show_connect_menu {
            pre_game_menu.draw(&mut d, Vector2::new(center_x, center_y), scale);
        }

        // connect menu
        if show_connect_menu {
            connect_menu.draw(&mut d, Vector2::new(center_x, center_y), scale);
        }

        // post game menu
        if game.result != ChessResult::Ongoing {
            let t = match game.result {
                ChessResult::Ongoing => "",
                ChessResult::WhiteWon => "White won",
                ChessResult::BlackWon => "Black won",
                ChessResult::Draw => "Draw",
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
            (origin.y + (self.y - self.height as f32 / 2.0 + 2.0) * scale) as i32,
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

struct ChessServer {
    listener: TcpListener,
    stream: Option<TcpStream>,
    own_color: PieceColor,
    name: Option<String>,
    network_phase: NetworkPhase,
    saved_move: Option<Move>,
}

impl ChessServer {
    fn new(address: &String, port: u16, name: Option<String>) -> Option<Self> {
        let addr = format!("{}:{}", address, port);
        let listener = match TcpListener::bind(&addr) {
            Ok(listener) => listener,
            Err(_) => return None,
        };

        listener.set_nonblocking(true).unwrap();

        let stream = None;

        let own_color = PieceColor::White;
        let network_phase = NetworkPhase::NoConnection;
        let last_move = None;

        Some(Self { listener, stream, own_color, name, network_phase, saved_move: last_move })
    }

    fn listen(&mut self) {
        self.stream = match self.listener.accept() {
            Ok((stream, _)) => {
                println!("Connection established!");
                self.network_phase = NetworkPhase::FoundConnection;
                Some(stream)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Non-blocking mode; no connection yet
                None
            }
            Err(e) => {
                println!("Connection failed: {}", e);
                None
            }
        }
    }

    fn _has_active_connection(&self) -> bool {
        match &self.stream {
            Some(_) => true,
            None => false,
        }
    }

    fn send_start(&mut self, fen: Option<String>) {
        let start = Start {
            is_white: self.own_color != PieceColor::White,
            name: self.name.clone(),
            fen: fen,
            time: None,
            inc: None,
        };

        let mut buf = Vec::<u8>::try_from(start).unwrap();
        let mut stream = match &self.stream {
            Some(stream) => stream,
            None => return,
        };

        stream.write_all(&mut buf).unwrap();
    }

    fn send_move(&mut self, from: (u8, u8), to: (u8, u8), promotion: Option<PromotionPiece>, forfeit: bool, offer_draw: bool) {
        let pmove = Move {
            from,
            to,
            promotion,
            forfeit,
            offer_draw,
        };

        let mut buf = Vec::<u8>::try_from(pmove.clone()).unwrap();

        let mut stream = match &self.stream {
            Some(stream) => stream,
            None => return,
        };

        self.saved_move = Some(pmove);

        stream.write_all(&mut buf).unwrap();

        self.network_phase = NetworkPhase::Ack;
    }

    fn send_ack(&mut self, ok: bool, end_state: Option<GameState>) {
        let ack = Ack {
            ok,
            end_state,
        };

        let mut buf = Vec::<u8>::try_from(ack).unwrap();

        let mut stream = match &self.stream {
            Some(stream) => stream,
            None => return,
        };

        stream.write_all(&mut buf).unwrap();
    }

    fn receive_start(&mut self) -> Option<Start> {
        let mut stream = match &self.stream {
            Some(stream) => stream,
            None => return None,
        };

        let mut buf = [0; 512];
        
        match stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                match Start::try_from(&buf[..size]) {
                    Ok(start) => Some(start),
                    Err(_) => None,
                }
            },
            Ok(_) => None, // no data
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                None
            },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    fn receive_move(&mut self) -> Option<Move> {
        let mut stream = match &self.stream {
            Some(stream) => stream,
            None => return None,
        };


        let mut buf = [0; 512];
        
        match stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                match Move::try_from(&buf[..size]) {
                    Ok(pmove) => Some(pmove),
                    Err(_) => None,
                }
            },
            Ok(_) => None, // no data
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                None
            },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    fn receive_ack(&mut self) -> Option<Ack> {
        let mut stream = match &self.stream {
            Some(stream) => stream,
            None => return None,
        };

        let mut buf = [0; 512];
        
        match stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                match Ack::try_from(&buf[..size]) {
                    Ok(ack) => Some(ack),
                    Err(_) => None,
                }
            },
            Ok(_) => None, // no data
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                None
            },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}

struct ChessClient {
    stream: TcpStream,
    own_color: PieceColor,
    name: Option<String>,
    network_phase: NetworkPhase,
    saved_move: Option<Move>,
}

impl ChessClient {
    fn new(address: &String, name: Option<String>) -> Option<Self> {
        let stream = match TcpStream::connect(address) {
            Ok(stream) => stream,
            Err(_) => return None,
        };
        stream.set_nonblocking(true).unwrap();

        let own_color = PieceColor::Black;
        let network_phase = NetworkPhase::FoundConnection;
        let last_move = None;

        Some(Self { stream, own_color, name, network_phase, saved_move: last_move })
    }

    //fn new_a(address: &String, port: u16, name: Option<String>) -> Option<Self> {
    //    let addr = format!("{}:{}", address, port);
    //    Self::new(&addr, name)
    //}

    fn send_start(&mut self) {
        let start = Start {
            is_white: self.own_color == PieceColor::White,
            name: self.name.clone(),
            fen: None,
            time: None,
            inc: None,
        };

        let mut buf = Vec::<u8>::try_from(start).unwrap();

        self.stream.write_all(&mut buf).unwrap();

        self.network_phase = NetworkPhase::Start;
    }

    fn send_move(&mut self, from: (u8, u8), to: (u8, u8), promotion: Option<PromotionPiece>, forfeit: bool, offer_draw: bool) {
        let pmove = Move {
            from,
            to,
            promotion,
            forfeit,
            offer_draw,
        };

        let mut buf = Vec::<u8>::try_from(pmove.clone()).unwrap();

        self.saved_move = Some(pmove);

        self.stream.write_all(&mut buf).unwrap();

        self.network_phase = NetworkPhase::Ack;
    }

    fn send_ack(&mut self, ok: bool, end_state: Option<GameState>) {
        let ack = Ack {
            ok,
            end_state,
        };

        let mut buf = Vec::<u8>::try_from(ack).unwrap();

        self.stream.write_all(&mut buf).unwrap();
    }

    fn receive_start(&mut self) -> Option<Start> {
        let mut buf = [0; 512];
        
        match self.stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                match Start::try_from(&buf[..size]) {
                    Ok(start) => Some(start),
                    Err(_) => None,
                }
            },
            Ok(_) => None, // no data
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                None
            },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    fn receive_move(&mut self) -> Option<Move> {
        let mut buf = [0; 512];
        
        match self.stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                match Move::try_from(&buf[..size]) {
                    Ok(pmove) => Some(pmove),
                    Err(_) => None,
                }
            },
            Ok(_) => None, // no data
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                None
            },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    fn receive_ack(&mut self) -> Option<Ack> {
        let mut buf = [0; 512];
        
        match self.stream.read(&mut buf) {
            Ok(size) if size > 0 => {
                match Ack::try_from(&buf[..size]) {
                    Ok(ack) => Some(ack),
                    Err(_) => None,
                }
            },
            Ok(_) => None, // no data
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                None
            },
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}

enum NetworkPhase {
    NoConnection,
    FoundConnection, // client sends start and server listens for start
    Start,      // server sends start and client listens
    Move,       // doing / listening for move
    Ack,        // listening for ack
    GameOver,
}

fn to_square(pos: &(u8, u8)) -> Square {
    Square { x: pos.0 as i8, y: pos.1 as i8 }
}

fn complete_move(game: &mut Game, pmove: &Move) -> bool {
    if !game.do_move(&to_square(&pmove.from), &to_square(&pmove.to)) {
        return false
    }

    if game.promotion {
        game.pawn_promotion(if let Some(promotion_piece) = &pmove.promotion {
            match promotion_piece {
                PromotionPiece::Queen => PieceType::Queen,
                PromotionPiece::Bishop => PieceType::Bishop,
                PromotionPiece::Knight => PieceType::Knight,
                PromotionPiece::Rook => PieceType::Rook,
            }
        }
        else {
            // no promotion_piece was supplied, defaulting to queen
            println!("no promotion_piece was supplied, defaulting to queen");
            PieceType::Queen
        });
            
    }
    true
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