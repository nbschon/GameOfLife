mod structures;

use crate::board::Board;
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
use crate::game::structures::structuress::{get_structure_vec, Structure};

pub struct Game {
    board: Board,
    color_alive: Color,
    color_dead: Color,
    color_grid: Color,
    color_bg: Color,
    screen_width: u32,
    screen_height: u32,
    cells_width: i32,
    cells_height: i32,
    cell_width: i32,
    cell_height: i32,
    generation: u32,
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

        Game {
            board: Board::with_size(cells_width, cells_height),
            color_alive: Color::RGB(0x17, 0x17, 0x17),
            color_dead: Color::RGB(0xF7, 0xF7, 0xF7),
            color_grid: Color::RGB(0x7F, 0x7F, 0x7F),
            color_bg: Color::RGB(0, 0, 0),
            screen_width: width,
            screen_height: height,
            cells_width,
            cells_height,
            cell_width: 12,
            cell_height: 12,
            generation: 0,
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
        let min_dim = 2;

        let cell_length = match keymod {
            Some(km) => {
                if km.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD) {
                    5
                } else {
                    1
                }
            },
            None => {
                2
            }
        };

        if zoom_in {
            self.cell_width = (rect_width + cell_length).min(max_dim);
            self.cell_height = (rect_height + cell_length).min(max_dim);
        } else {
            self.cell_width = (rect_width - cell_length).max(min_dim);
            self.cell_height = (rect_height - cell_length).max(min_dim);
        }
    }

    pub fn game_loop(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut run_sim = false;
        let mut denom: u32 = 60;

        let mut pan_camera = false;
        let mut camera_offset_x = 0;
        let mut camera_offset_y = 0;

        let mut structure_selected = false;
        let mut structure_idx: usize = 0;
        let mut cursor_structure: Vec<Vec<u8>> = vec![];
        let color_alive_ghost = Color::RGBA(0, 0, 0xFF, 0x8F);
        let color_dead_ghost = Color::RGBA(0, 0, 0xFF, 0x2F);

        let mut cursor_rect =
            Rect::new(0, 0, self.cell_width as u32, self.cell_height as u32);
        let color_cursor = Color::RGBA(0xFF, 0, 0, 0x7F);

        'running: loop {
            let mut cell_rect = Rect::new(
                camera_offset_x + 1,
                camera_offset_y + 1,
                self.cell_width as u32 - 2,
                self.cell_height as u32 - 2,
            );

            self.canvas.set_draw_color(self.color_grid);
            self.canvas.clear();

            let mouse_x = event_pump.mouse_state().x();
            let mouse_y = event_pump.mouse_state().y();

            let cells = &self.board.cells;
            for row in cells {
                for col in row {
                    if (-self.cell_width..self.screen_width as i32).contains(&cell_rect.x)
                        && (-self.cell_height..self.screen_height as i32).contains(&cell_rect.y) {
                        if *col {
                            self.canvas.set_draw_color(self.color_alive);
                        } else {
                            self.canvas.set_draw_color(self.color_dead);
                        }

                        let _ = self.canvas.fill_rect(cell_rect);
                    }
                    cell_rect.y += self.cell_height;
                }

                cell_rect.x += self.cell_width;
                cell_rect.y = camera_offset_y + 1;
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode, keymod, ..
                    } => match keycode {
                        Some(Keycode::Return) => {
                            run_sim = !run_sim;
                        }
                        Some(Keycode::R) => {
                            self.board.reset();
                            self.generation = 0;
                        }
                        Some(Keycode::Z) => {
                            camera_offset_x = 0;
                            camera_offset_y = 0;
                            self.cell_width = 12;
                            self.cell_height = 12;
                            cursor_rect.w = self.cell_width;
                            cursor_rect.h = self.cell_height;
                        }
                        Some(Keycode::Space) => {
                            if !run_sim {
                                self.board.step_game();
                                self.generation += 1;
                            }
                        }
                        Some(Keycode::Up) => {
                            denom += 5;
                        }
                        Some(Keycode::Down) => {
                            if denom > 5 {
                                denom -= 5;
                            } else {
                                denom = 1;
                            }
                        }
                        Some(Keycode::Right) => {
                            denom += 1;
                        }
                        Some(Keycode::Left) => {
                            if denom > 1 {
                                denom -= 1;
                            } else {
                                denom = 1;
                            }
                        }
                        Some(Keycode::Minus) => {
                            self.zoom_in_out(false, Some(keymod));
                            cursor_rect.w = self.cell_width;
                            cursor_rect.h = self.cell_height;
                        }
                        Some(Keycode::Equals) => {
                            self.zoom_in_out(true, Some(keymod));
                            cursor_rect.w = self.cell_width;
                            cursor_rect.h = self.cell_height;
                        }
                        Some(Keycode::S) => {
                            if !structure_selected {
                                let strctr = Structure::from_usize(structure_idx);
                                structure_selected = true;
                                if let Some(s) = strctr {
                                    cursor_structure = get_structure_vec(s);
                                }
                            } else {
                                structure_selected = !structure_selected;
                            }
                        }
                        Some(Keycode::D) => {
                            if structure_selected && Structure::from_usize(structure_idx + 1).is_some() {
                                structure_idx += 1;
                                let strctr = Structure::from_usize(structure_idx);
                                cursor_structure = get_structure_vec(strctr.unwrap());
                            }
                        }
                        Some(Keycode::A) => {
                            if structure_selected {
                                structure_idx = structure_idx.saturating_sub(1);
                                let strctr = Structure::from_usize(structure_idx);
                                if let Some(s) = strctr {
                                    cursor_structure = get_structure_vec(s);
                                }
                            }
                        }
                        Some(Keycode::F) => {
                            if structure_selected {
                                for row in &mut cursor_structure {
                                    row.reverse();
                                }
                            }
                        }
                        Some(Keycode::E) => {
                            if structure_selected {
                                cursor_structure.reverse();
                            }
                        }
                        Some(Keycode::I) => {
                            camera_offset_y += 10;
                        }
                        Some(Keycode::K) => {
                            camera_offset_y -= 10;
                        }
                        Some(Keycode::J) => {
                            camera_offset_x += 10;
                        }
                        Some(Keycode::L) => {
                            camera_offset_x -= 10;
                        }
                        _ => {}
                    },
                    Event::MouseButtonDown {
                        x, y, mouse_btn, ..
                    } => match mouse_btn {
                        MouseButton::Left => {
                            let (new_x, new_y) =
                                self.mouse_to_coords(x, y, camera_offset_x, camera_offset_y);
                            if structure_selected {
                                let mut x_offset = 0;

                                for (y_offset, row) in cursor_structure.iter().enumerate() {
                                    for col in row {
                                        let status = match *col {
                                            0 => false,
                                            1 => true,
                                            _ => unreachable!("Bad value in structure array")
                                        };
                                        self.board.set_coords(x_offset + new_x as u32, y_offset as u32 + new_y as u32, status);
                                        x_offset += 1;
                                    }
                                    x_offset = 0;
                                }

                                structure_selected = false;
                            } else if !pan_camera && new_x < self.cells_width && new_x >= 0 && new_y < self.cells_height && new_y >= 0 {
                                let cell_status =
                                    self.board.get_cell_status(new_x as u32, new_y as u32);
                                self.board
                                    .set_coords(new_x as u32, new_y as u32, !cell_status);
                            }
                        }
                        MouseButton::Right => {
                            pan_camera = true;
                        }
                        _ => {}
                    },
                    Event::MouseButtonUp { mouse_btn, .. } => if mouse_btn == MouseButton::Right {
                        pan_camera = false;
                    },
                    Event::MouseWheel { y, ..} => {
                        match y {
                            y if y > 0 => self.zoom_in_out(true, None),
                            y if y < 0 => self.zoom_in_out(false, None),
                            _ => {}
                        }
                        cursor_rect.w = self.cell_width;
                        cursor_rect.h = self.cell_height;
                    }
                    _ => {}
                }
            }

            let (cursor_x, cursor_y) =
                self.mouse_to_coords(mouse_x, mouse_y, camera_offset_x, camera_offset_y);

            let cursor_rect_x = (cursor_x * self.cell_width) + camera_offset_x;
            let cursor_rect_y = (cursor_y * self.cell_height) + camera_offset_y;
            cursor_rect.x = cursor_rect_x;
            cursor_rect.y = cursor_rect_y;

            if pan_camera {
                let new_mouse_x = event_pump.mouse_state().x();
                let new_mouse_y = event_pump.mouse_state().y();
                camera_offset_x += new_mouse_x - mouse_x;
                camera_offset_y += new_mouse_y - mouse_y;
            } else if structure_selected {
                let mut ghost_rect = Rect::new(
                    cursor_rect_x, cursor_rect_y, self.cell_width as u32, self.cell_height as u32);


                for row in &cursor_structure {
                    for col in row {
                        match *col {
                            0 => self.canvas.set_draw_color(color_dead_ghost),
                            1 => self.canvas.set_draw_color(color_alive_ghost),
                            _ => unreachable!("Bad value in structure array")
                        }

                        let _ = self.canvas.fill_rect(ghost_rect);
                        ghost_rect.x += self.cell_width;
                    }
                    ghost_rect.y += self.cell_height;
                    ghost_rect.x = (cursor_x * self.cell_width) + camera_offset_x;
                }
            } else {
                self.canvas.set_draw_color(color_cursor);
                let _ = self.canvas.fill_rect(cursor_rect);
            }

            if run_sim {
                self.board.step_game();
                self.generation += 1;
            }

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / denom));
        }
    }
}
