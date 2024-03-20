use crate::game_of_life::structures::*;
use crate::game_of_life::board::Board;
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    Sdl
};
use std::time::Duration;

pub struct Game {
    board: Board,
    color_alive: Color,
    color_dead: Color,
    color_bg: Color,
    color_ghost_alive: Color,
    color_ghost_dead: Color,
    color_cursor: Color,
    dark_mode: bool,

    screen_width: u32,
    screen_height: u32,

    cells_width: i32,
    cells_height: i32,
    cell_width: i32,
    cell_height: i32,

    generation: u64,

    pan_cam: bool,
    cam_offset_x: i32,
    cam_offset_y: i32,

    strctr_selected: bool,
    strctr_idx: usize,
    strctr_cursor: Vec<Vec<u8>>,

    cursor_rect: Rect,

    run_sim: bool,
    denom: i32,

    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl Game {
    pub fn with_size(width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Game of Life", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let cells_width = 1000;
        let cells_height = 1000;
        let cell_width = 12;
        let cell_height = 12;

        Game {
            board: Board::with_size(cells_width, cells_height),
            color_alive: Color::RGB(0x17, 0x17, 0x17),
            color_dead: Color::RGB(0xF7, 0xF7, 0xF7),
            color_bg: Color::RGB(0x7F, 0x7F, 0x7F),
            color_ghost_alive: Color::RGBA(0, 0, 0xFF, 0x8F),
            color_ghost_dead: Color::RGBA(0, 0, 0xFF, 0x2F),
            color_cursor: Color::RGBA(0xFF, 0, 0, 0x7F),
            dark_mode: false,
            screen_width: width,
            screen_height: height,
            cells_width,
            cells_height,
            cell_width: 12,
            cell_height: 12,
            generation: 0,
            pan_cam: false,
            cam_offset_x: 0,
            cam_offset_y: 0,

            strctr_selected: false,
            strctr_idx: 0,
            strctr_cursor: vec![],

            cursor_rect: Rect::new(0, 0, cell_width as u32, cell_height as u32),

            run_sim: false,
            denom: 60,

            sdl_context,
            canvas,
        }
    }

    fn mouse_to_coords(&self, x: i32, y: i32, camera_x: i32, camera_y: i32) -> (i32, i32) {
        let array_x = (x - camera_x) / self.cell_width;
        let array_y = (y - camera_y) / self.cell_height;

        (array_x, array_y)
    }

    fn zoom_in_out(&mut self, zoom_in: bool, keymod: Option<Mod>) {
        let rect_width = self.cell_width;
        let rect_height = self.cell_height;
        let max_dim = 60;
        let min_dim = 4;

        let delta_length = match keymod {
            Some(km) if km.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD) => 5,
            Some(_) => 1,
            None => 2
        };

        let (new_width, new_height) = if zoom_in {
            (rect_width + delta_length, rect_height + delta_length)
        } else {
            (rect_width - delta_length, rect_height - delta_length)
        };
        let old_per_screen = self.screen_width as i32 / rect_width;
        let new_per_screen = self.screen_width as i32 / new_width;

        if zoom_in {
            self.cell_width = new_width.min(max_dim);
            self.cell_height = new_height.min(max_dim);
            self.cam_offset_x += (new_per_screen - old_per_screen).abs() as i32;
            self.cam_offset_y += (new_per_screen - old_per_screen).abs() as i32;
        } else {
            self.cell_width = new_width.max(min_dim);
            self.cell_height = new_height.max(min_dim);
            self.cam_offset_x -= (old_per_screen - new_per_screen).abs() as i32;
            self.cam_offset_y -= (old_per_screen - new_per_screen).abs() as i32;
        }
    }

    pub fn game_loop(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            let mut cell_rect = Rect::new(
                self.cam_offset_x + 1,
                self.cam_offset_y + 1,
                self.cell_width as u32 - 2,
                self.cell_height as u32 - 2,
            );

            self.canvas.set_draw_color(self.color_bg);
            self.canvas.clear();

            let mouse_x = event_pump.mouse_state().x();
            let mouse_y = event_pump.mouse_state().y();

            let cells = &self.board.cells;
            for row in cells {
                for col in row {
                    if (-self.cell_width..self.screen_width as i32).contains(&cell_rect.x)
                        && (-self.cell_height..self.screen_height as i32).contains(&cell_rect.y) {
                        self.canvas.set_draw_color(
                            if *col {
                                self.color_alive
                            } else {
                                self.color_dead
                            }
                        );

                        let _ = self.canvas.fill_rect(cell_rect);
                    }
                    cell_rect.y += self.cell_height;
                }

                cell_rect.x += self.cell_width;
                cell_rect.y = self.cam_offset_y + 1;
            }

            for event in event_pump.poll_iter() {
                if self.do_input(event) { break 'running; }
            }

            let (cursor_x, cursor_y) =
                self.mouse_to_coords(mouse_x, mouse_y, self.cam_offset_x, self.cam_offset_y);

            let cursor_rect_x = (cursor_x * self.cell_width) + self.cam_offset_x;
            let cursor_rect_y = (cursor_y * self.cell_height) + self.cam_offset_y;
            self.cursor_rect.x = cursor_rect_x;
            self.cursor_rect.y = cursor_rect_y;

            if self.pan_cam {
                let new_mouse_x = event_pump.mouse_state().x();
                let new_mouse_y = event_pump.mouse_state().y();
                self.cam_offset_x += new_mouse_x - mouse_x;
                self.cam_offset_y += new_mouse_y - mouse_y;
            } else if self.strctr_selected {
                let mut ghost_rect = Rect::new(
                    cursor_rect_x, cursor_rect_y, self.cell_width as u32, self.cell_height as u32);

                for row in &self.strctr_cursor {
                    for col in row {
                        match *col {
                            0 => self.canvas.set_draw_color(self.color_ghost_dead),
                            1 => self.canvas.set_draw_color(self.color_ghost_alive),
                            _ => unreachable!("Bad value in structure array")
                        }

                        let _ = self.canvas.fill_rect(ghost_rect);
                        ghost_rect.x += self.cell_width;
                    }
                    ghost_rect.y += self.cell_height;
                    ghost_rect.x = (cursor_x * self.cell_width) + self.cam_offset_x;
                }
            } else {
                self.canvas.set_draw_color(self.color_cursor);
                let _ = self.canvas.fill_rect(self.cursor_rect);
            }

            if self.run_sim {
                self.board.step_game();
                self.generation += 1;
            }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.denom as u32));
        }
    }

    fn do_input(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            Event::KeyDown {
                keycode, keymod, ..
            } => match keycode {
                Some(Keycode::Return) => {
                    self.run_sim = !self.run_sim;
                }
                Some(Keycode::R) => {
                    self.board.reset();
                    self.generation = 0;
                }
                Some(Keycode::Z) => {
                    self.cam_offset_x = 0;
                    self.cam_offset_y = 0;
                    self.cell_width = 12;
                    self.cell_height = 12;
                    self.cursor_rect.w = self.cell_width;
                    self.cursor_rect.h = self.cell_height;
                }
                Some(Keycode::Space) => {
                    if !self.run_sim {
                        self.board.step_game();
                        self.generation += 1;
                    }
                }
                Some(Keycode::Up) => {
                    self.denom += 5;
                }
                Some(Keycode::Down) => {
                    self.denom = (self.denom - 5).max(1);
                }
                Some(Keycode::Right) => {
                    self.denom += 1;
                }
                Some(Keycode::Left) => {
                    self.denom = (self.denom - 1).max(1);
                }
                Some(Keycode::Minus) => {
                    self.zoom_in_out(false, Some(keymod));
                    self.cursor_rect.w = self.cell_width;
                    self.cursor_rect.h = self.cell_height;
                }
                Some(Keycode::Equals) => {
                    self.zoom_in_out(true, Some(keymod));
                    self.cursor_rect.w = self.cell_width;
                    self.cursor_rect.h = self.cell_height;
                }
                Some(Keycode::S) => {
                    if !self.strctr_selected {
                        let strctr = Structure::from_usize(self.strctr_idx);
                        self.strctr_selected = true;
                        if let Some(s) = strctr {
                            self.strctr_cursor = get_structure_vec(s);
                        }
                    } else {
                        self.strctr_selected = !self.strctr_selected;
                    }
                }
                Some(Keycode::D) => {
                    if self.strctr_selected && Structure::from_usize(self.strctr_idx + 1).is_some() {
                        self.strctr_idx += 1;
                        let strctr = Structure::from_usize(self.strctr_idx);
                        self.strctr_cursor = get_structure_vec(strctr.unwrap());
                    }
                }
                Some(Keycode::A) => {
                    if self.strctr_selected {
                        self.strctr_idx = self.strctr_idx.saturating_sub(1);
                        let strctr = Structure::from_usize(self.strctr_idx);
                        if let Some(s) = strctr {
                            self.strctr_cursor = get_structure_vec(s);
                        }
                    }
                }
                Some(Keycode::F) => {
                    if self.strctr_selected {
                        for row in &mut self.strctr_cursor {
                            row.reverse();
                        }
                    }
                }
                Some(Keycode::E) => {
                    if self.strctr_selected {
                        self.strctr_cursor.reverse();
                    }
                }
                Some(Keycode::H) => {
                    self.cam_offset_x += 10;
                }
                Some(Keycode::K) => {
                    self.cam_offset_y += 10;
                }
                Some(Keycode::J) => {
                    self.cam_offset_y -= 10;
                }
                Some(Keycode::L) => {
                    self.cam_offset_x -= 10;
                }
                Some(Keycode::C) => {
                    self.dark_mode = !self.dark_mode;
                    if self.dark_mode {
                        self.color_alive = Color::RGB(0xA7, 0xA7, 0xA7);
                        self.color_dead = Color::RGB(0x17, 0x17, 0x17);
                        self.color_bg = Color::RGB(0x27, 0x27, 0x27);
                        self.color_ghost_alive = Color::RGBA(0, 0x96, 0xFF, 0x7F);
                        self.color_ghost_dead = Color::RGBA(0, 0x96, 0xFF, 0x3F);
                        self.color_cursor = Color::RGBA(0xFF, 0, 0, 0x7F);
                    } else {
                        self.color_alive = Color::RGB(0x17, 0x17, 0x17);
                        self.color_dead = Color::RGB(0xF7, 0xF7, 0xF7);
                        self.color_bg = Color::RGB(0x7F, 0x7F, 0x7F);
                        self.color_ghost_alive = Color::RGBA(0, 0, 0xFF, 0x8F);
                        self.color_ghost_dead = Color::RGBA(0, 0, 0xFF, 0x2F);
                        self.color_cursor = Color::RGBA(0xFF, 0, 0, 0x7F);
                    }
                }
                _ => {}
            },
            Event::MouseButtonDown {
                x, y, mouse_btn, ..
            } => match mouse_btn {
                MouseButton::Left => {
                    let (new_x, new_y) = self.mouse_to_coords(x, y, self.cam_offset_x, self.cam_offset_y);
                    if self.strctr_selected {
                        let mut x_offset = 0;

                        for (y_offset, row) in self.strctr_cursor.iter().enumerate() {
                            for col in row {
                                let status = match *col {
                                    0 => false,
                                    1 => true,
                                    _ => panic!("Bad value in structure array")
                                };
                                self.board.set_coords(x_offset + new_x as u32, y_offset as u32 + new_y as u32, status);
                                x_offset += 1;
                            }
                            x_offset = 0;
                        }

                        self.strctr_selected = false;
                    } else if !self.pan_cam && (0..self.cell_width).contains(&new_x) && (0..self.cell_height).contains(&new_y) {
                        let cell_status = self.board.get_cell_status(new_x as u32, new_y as u32);
                        self.board .set_coords(new_x as u32, new_y as u32, !cell_status);
                    }
                }
                MouseButton::Right => {
                    self.pan_cam = true;
                }
                _ => {}
            },
            Event::MouseButtonUp { mouse_btn, .. } => if mouse_btn == MouseButton::Right {
                self.pan_cam = false;
            },
            Event::MouseWheel { y, .. } => {
                match y {
                    y if y > 0 => self.zoom_in_out(true, None),
                    y if y < 0 => self.zoom_in_out(false, None),
                    _ => {}
                }
                self.cursor_rect.w = self.cell_width;
                self.cursor_rect.h = self.cell_height;
            }
            _ => {}
        }
        false
    }
}
