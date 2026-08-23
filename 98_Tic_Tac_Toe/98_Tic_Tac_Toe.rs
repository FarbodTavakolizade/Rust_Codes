/*  cargo.toml should be same as this

[package]
name = "tic_tac_toe_gui"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe="0.27" 

at last part  for running the project use this command: cargo run in powershell
*/
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic Tac Toe AI",
        options,
        Box::new(|_cc| Box::new(TicTacToeAI::default())),
    )
}

#[derive(Default)]
struct TicTacToeAI {
    board: [[char; 3]; 3],
    human_turn: bool,
    winner: Option<char>,
    game_over: bool,
}

impl TicTacToeAI {
    fn reset(&mut self) {
        self.board = [['\0'; 3]; 3];
        self.winner = None;
        self.game_over = false;
        self.human_turn = true;
    }

    fn check_winner(&self) -> Option<char> {
        let lines = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for &line in &lines {
            let [a, b, c] = line;
            let (x1, y1) = a;
            let (x2, y2) = b;
            let (x3, y3) = c;
            let v1 = self.board[x1][y1];
            let v2 = self.board[x2][y2];
            let v3 = self.board[x3][y3];

            if v1 != '\0' && v1 == v2 && v2 == v3 {
                return Some(v1);
            }
        }
        None
    }

    fn is_draw(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&c| c != '\0'))
    }

    fn minimax(&mut self, is_maximizing: bool) -> i32 {
        if let Some(w) = self.check_winner() {
            return match w {
                'O' => 1,
                'X' => -1,
                _ => 0,
            };
        }
        if self.is_draw() {
            return 0;
        }

        if is_maximizing {
            let mut best_score = -1000;
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[i][j] == '\0' {
                        self.board[i][j] = 'O';
                        let score = self.minimax(false);
                        self.board[i][j] = '\0';
                        if score > best_score {
                            best_score = score;
                        }
                    }
                }
            }
            best_score
        } else {
            let mut best_score = 1000;
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[i][j] == '\0' {
                        self.board[i][j] = 'X';
                        let score = self.minimax(true);
                        self.board[i][j] = '\0';
                        if score < best_score {
                            best_score = score;
                        }
                    }
                }
            }
            best_score
        }
    }

    fn best_move(&mut self) -> Option<(usize, usize)> {
        let mut best_score = -1000;
        let mut move_pos = None;

        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == '\0' {
                    self.board[i][j] = 'O';
                    let score = self.minimax(false);
                    self.board[i][j] = '\0';

                    if score > best_score {
                        best_score = score;
                        move_pos = Some((i, j));
                    }
                }
            }
        }

        move_pos
    }
}

impl eframe::App for TicTacToeAI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tic Tac Toe - you are [X] ( at first restart the game)");

            for i in 0..3 {
                ui.horizontal(|ui| {
                    for j in 0..3 {
                        let mut label = self.board[i][j].to_string();
                        if label == "\0" {
                            label = " ".to_string();
                        }
                        if ui
                            .add(egui::Button::new(label).min_size([60.0, 60.0].into()))
                            .clicked()
                        {
                            if self.board[i][j] == '\0'
                                && self.winner.is_none()
                                && !self.game_over
                                && self.human_turn
                            {
                                self.board[i][j] = 'X';
                                self.winner = self.check_winner();

                                if self.winner.is_none() && !self.is_draw() {
                                    self.human_turn = false;

                                    if let Some((x, y)) = self.best_move() {
                                        self.board[x][y] = 'O';
                                    }

                                    self.winner = self.check_winner();

                                    if self.winner.is_some() || self.is_draw() {
                                        self.game_over = true;
                                    } else {
                                        self.human_turn = true;
                                    }
                                } else {
                                    self.game_over = true;
                                }

                                ctx.request_repaint();
                            }
                        }
                    }
                });
            }

            if let Some(winner) = self.winner {
                if winner == 'X' {
                    ui.label("you win");
                } else {
                    ui.label("computer wins");
                }
            } else if self.is_draw() {
                ui.label("withdraw");
            }

            if ui.button("restart").clicked() {
                self.reset();
                ctx.request_repaint();
            }
        });
    }
}
