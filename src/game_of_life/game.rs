use crate::game_of_life::board::Board;
use crate::game_of_life::structures::*;
use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{Window, self},
    Sdl,
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
    tex_width: u32,
    tex_height: u32,
    tex_offset: i32,
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

        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let cells_width = 200;
        let cells_height = 200;
        let cell_width = 12;
        let cell_height = 12;

        let tex_offset = 0;
        let tex_width = 1000;
        let tex_height = height - tex_offset as u32 * 2;

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
            cell_width: 12,
            cell_height: 12,
            generation: 0,
            pan_cam: false,
            cam_offset_x: 40,
            cam_offset_y: 40,

            strctr_selected: false,
            strctr_idx: 0,
            strctr_cursor: vec![],

            cursor_rect: Rect::new(0, 0, cell_width as u32, cell_height as u32),

            run_sim: false,
            denom: 60,

            sdl_context,
            canvas,
            tex_width,
            tex_height,
            tex_offset,
        }
    }

    fn mouse_to_coords(&self, x: i32, y: i32) -> (i32, i32) {
        let array_x = (x - self.cam_offset_x) / self.cell_width;
        let array_y = (y - self.cam_offset_y) / self.cell_height;

        (array_x, array_y)
    }

    fn zoom_in_out(&mut self, zoom_in: bool, keymod: Option<Mod>, mouse_pos: Option<(i32, i32)>) {
        let old_width = self.cell_width;
        let old_height = self.cell_height;
        let max_dim = 60;
        let min_dim = 4;

        let mut delta_length = match keymod {
            Some(km) if km.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD) => 6,
            _ => 2,
        };
        if !zoom_in { delta_length *= -1; }

        let new_width = (old_width + delta_length).clamp(min_dim, max_dim);
        let new_height = (old_height + delta_length).clamp(min_dim, max_dim);
        if new_width == old_width && new_height == old_height {
            return;
        }

        let (target_screen_x, target_screen_y) = mouse_pos.unwrap_or((self.screen_width as i32 / 2, self.screen_height as i32 / 2));
        let (target_cell_x, target_cell_y) = self.mouse_to_coords(target_screen_x as i32, target_screen_y as i32);

        let old_pix_x = (target_screen_x - (self.cam_offset_x + 1)) - target_cell_x * old_width;
        let old_pix_y = (target_screen_y - (self.cam_offset_y + 1)) - target_cell_y * old_height;

        let new_pix_x = ((old_pix_x as i64) * (new_width as i64) + (old_width as i64 / 2)) / (old_width as i64);
        let new_pix_y = ((old_pix_y as i64) * (new_height as i64) + (old_height as i64 / 2)) / (old_height as i64);

        self.cell_width = new_width;
        self.cell_height = new_height;

        self.cam_offset_x = target_screen_x - 1 - (target_cell_x * new_width + new_pix_x as i32);
        self.cam_offset_y = target_screen_y - 1 - (target_cell_y * new_height + new_pix_y as i32);
    }

    pub fn game_loop(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let texture_creator = self.canvas.texture_creator();
        let mut game_tex = texture_creator
            .create_texture(
                None,
                sdl2::render::TextureAccess::Target,
                self.screen_width,
                self.screen_height,
            )
            .unwrap();
        // let pause_tex = texture_creator.load_texture("img/pause.png")?;
        // let pause_rect = Rect::new(
        //     self.tex_width as i32 + self.tex_offset * 2,
        //     self.tex_offset,
        //     100,
        //     100,
        // );
        // let _ = self.canvas.draw_rect(pause_rect);

        'running: loop {
            let mut cell_rect = Rect::new(
                self.cam_offset_x + 1,
                self.cam_offset_y + 1,
                self.cell_width as u32 - 2,
                self.cell_height as u32 - 2,
            );

            self.canvas.set_draw_color(Color::RGB(0x27, 0x2D, 0x36));
            self.canvas.clear();

            let _ = self.canvas.with_texture_canvas(&mut game_tex, |tc| {
                tc.set_draw_color(self.color_bg);
                tc.clear();
                let cells = &self.board.cells;
                for row in cells {
                    for col in row {
                        if (-self.cell_width..self.screen_width as i32).contains(&cell_rect.x)
                            && (-self.cell_height..self.screen_height as i32).contains(&cell_rect.y)
                        {
                            tc.set_draw_color(if *col {
                                self.color_alive
                            } else {
                                self.color_dead
                            });

                            let _ = tc.fill_rect(cell_rect);
                        }
                        cell_rect.y += self.cell_height;
                    }

                    cell_rect.x += self.cell_width;
                    cell_rect.y = self.cam_offset_y + 1;
                }
            });

            let mouse_x = event_pump.mouse_state().x() - self.tex_offset;
            let mouse_y = event_pump.mouse_state().y() - self.tex_offset;

            for event in event_pump.poll_iter() {
                if self.do_input(event, (mouse_x, mouse_y)) {
                    break 'running;
                }
            }

            let (cursor_x, cursor_y) = self.mouse_to_coords(mouse_x, mouse_y);

            let cursor_rect_x = (cursor_x * self.cell_width) + self.cam_offset_x;
            let cursor_rect_y = (cursor_y * self.cell_height) + self.cam_offset_y;
            self.cursor_rect.x = cursor_rect_x;
            self.cursor_rect.y = cursor_rect_y;

            let _ = self.canvas.with_texture_canvas(&mut game_tex, |tc| {
                if self.pan_cam {
                    let new_mouse_x = event_pump.mouse_state().x() - self.tex_offset;
                    let new_mouse_y = event_pump.mouse_state().y() - self.tex_offset;
                    self.cam_offset_x += new_mouse_x - mouse_x;
                    self.cam_offset_y += new_mouse_y - mouse_y;
                } else if self.strctr_selected {
                    let mut ghost_rect = Rect::new(
                        cursor_rect_x,
                        cursor_rect_y,
                        self.cell_width as u32,
                        self.cell_height as u32,
                    );

                    for row in &self.strctr_cursor {
                        for col in row {
                            match *col {
                                0 => tc.set_draw_color(self.color_ghost_dead),
                                1 => tc.set_draw_color(self.color_ghost_alive),
                                _ => unreachable!("Bad value in structure array"),
                            }

                            let _ = tc.fill_rect(ghost_rect);
                            ghost_rect.x += self.cell_width;
                        }
                        ghost_rect.y += self.cell_height;
                        ghost_rect.x = (cursor_x * self.cell_width) + self.cam_offset_x;
                    }
                } else {
                    tc.set_draw_color(self.color_cursor);
                    let _ = tc.fill_rect(self.cursor_rect);
                }
            });

            let draw_rect = Rect::new(
                0, 0, self.screen_width, self.screen_height,
            );
            let _ = self.canvas.copy(&game_tex, None, draw_rect);
            // let _ = self.canvas.copy(&pause_tex, None, pause_rect);

            if self.run_sim {
                self.board.step_game();
                self.generation += 1;
            }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.denom as u32));
        }

        Ok(())
    }

    fn do_input(&mut self, event: Event, mouse_pos: (i32, i32)) -> bool {
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
                    self.zoom_in_out(false, Some(keymod), None);
                    self.cursor_rect.w = self.cell_width;
                    self.cursor_rect.h = self.cell_height;
                }
                Some(Keycode::Equals) => {
                    self.zoom_in_out(true, Some(keymod), None);
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
                    if self.strctr_selected && Structure::from_usize(self.strctr_idx + 1).is_some()
                    {
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
                Some(Keycode::V) => {
                    self.board.randomize();
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
                    let (new_x, new_y) = self.mouse_to_coords(
                        x - self.tex_offset,
                        y - self.tex_offset,
                    );

                    if self.strctr_selected {
                        if (0..self.board.width).contains(&new_x)
                            && (0..self.board.height).contains(&new_y)
                        {
                            for (y_offset, row) in self.strctr_cursor.iter().enumerate() {
                                for (x_offset, col) in row.iter().enumerate() {
                                    let status = match *col {
                                        0 => false,
                                        1 => true,
                                        _ => panic!("Bad value in structure array"),
                                    };
                                    self.board.cells[x_offset + new_x as usize]
                                        [y_offset + new_y as usize] = status;
                                }
                            }
                        }

                        self.strctr_selected = false;
                    } else if !self.pan_cam
                        && (0..self.board.width).contains(&new_x)
                        && (0..self.board.height).contains(&new_y)
                    {
                        let cell_status = self.board.cells[new_x as usize][new_y as usize];
                        self.board.cells[new_x as usize][new_y as usize] = !cell_status;
                    }
                }
                MouseButton::Right => {
                    self.pan_cam = true;
                }
                _ => {}
            },
            Event::MouseButtonUp { mouse_btn, .. } => {
                if mouse_btn == MouseButton::Right {
                    self.pan_cam = false;
                }
            }
            Event::MouseWheel { y, .. } => {
                match y {
                    y if y > 0 => self.zoom_in_out(true, None, Some(mouse_pos)),
                    y if y < 0 => self.zoom_in_out(false, None, Some(mouse_pos)),
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
