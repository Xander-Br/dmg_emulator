use std;
use crate::utils::bit;


const NUMBER_OF_OBJECTS: usize = 40;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White = 255,
    LightGray = 192,
    DarkGray = 96,
    Black = 0,
}

impl std::convert::From<u8> for Color {
    fn from(n: u8) -> Self {
        match n {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Cannot covert {} to color", n),
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BackgroundColors(Color, Color, Color, Color);

impl BackgroundColors {
    fn new() -> BackgroundColors {
        BackgroundColors(
            Color::White,
            Color::LightGray,
            Color::DarkGray,
            Color::Black,
        )
    }
}

impl std::convert::From<u8> for BackgroundColors {
    fn from(value: u8) -> Self {
        BackgroundColors(
            (value & 0b11).into(),
            ((value >> 2) & 0b11).into(),
            ((value >> 4) & 0b11).into(),
            (value >> 6).into(),
        )
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileMap {
    X9800,
    X9C00,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BackgroundAndWindowDataSelect {
    X8000,
    X8800,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ObjectSize {
    OS8X8,
    OS8X16,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    HorizontalBlank,
    VerticalBlank,
    OAMAccess,
    VRAMAccess,
}

impl std::convert::From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::HorizontalBlank => 0,
            Mode::VerticalBlank => 1,
            Mode::OAMAccess => 2,
            Mode::VRAMAccess => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

impl Default for TilePixelValue {
    fn default() -> Self {
        TilePixelValue::Zero
    }
}

type TileRow = [TilePixelValue; 8];
type Tile = [TileRow; 8];

#[inline(always)]
fn empty_tile() -> Tile {
    [[Default::default(); 8]; 8]
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ObjectData {
    x: i16,
    y: i16,
    tile: u8,
    palette: ObjectPalette,
    xflip: bool,
    yflip: bool,
    priority: bool,
}

impl Default for ObjectData {
    fn default() -> Self {
        ObjectData {
            x: -16,
            y: -8,
            tile: Default::default(),
            palette: Default::default(),
            xflip: Default::default(),
            yflip: Default::default(),
            priority: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ObjectPalette {
    Zero,
    One,
}

impl Default for ObjectPalette {
    fn default() -> Self {
        ObjectPalette::Zero
    }
}

#[derive(Eq, PartialEq)]
pub enum InterruptRequest {
    None,
    VBlank,
    LCDStat,
    Both,
}

impl InterruptRequest {
    fn add(&mut self, other: InterruptRequest) {
        match self {
            InterruptRequest::None => *self = other,
            InterruptRequest::VBlank if other == InterruptRequest::LCDStat => {
                *self = InterruptRequest::Both
            }
            InterruptRequest::LCDStat if other == InterruptRequest::VBlank => {
                *self = InterruptRequest::Both
            }
            _ => {}
        };
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Window {
    pub x: u8,
    pub y: u8,
}

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct GPU {
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub canvas_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub tile_set: [Tile; 384],
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub object_data: [ObjectData; NUMBER_OF_OBJECTS],
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub vram: [u8; 0x2000],
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    pub oam: [u8; 0xA0],
    pub background_colors: BackgroundColors,
    pub viewport_x_offset: u8,
    pub viewport_y_offset: u8,
    pub lcd_display_enabled: bool,
    pub window_display_enabled: bool,
    pub background_display_enabled: bool,
    pub object_display_enabled: bool,
    pub line_equals_line_check_interrupt_enabled: bool,
    pub oam_interrupt_enabled: bool,
    pub vblank_interrupt_enabled: bool,
    pub hblank_interrupt_enabled: bool,
    pub line_check: u8,
    pub line_equals_line_check: bool,
    pub window_tile_map: TileMap,
    pub background_tile_map: TileMap,
    pub background_and_window_data_select: BackgroundAndWindowDataSelect,
    pub object_size: ObjectSize,
    pub obj_0_color_1: Color,
    pub obj_0_color_2: Color,
    pub obj_0_color_3: Color,
    pub obj_1_color_1: Color,
    pub obj_1_color_2: Color,
    pub obj_1_color_3: Color,
    pub window: Window,
    pub line: u8,
    pub mode: Mode,
    cycles: u16,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            canvas_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
            tile_set: [empty_tile(); 384],
            object_data: [Default::default(); NUMBER_OF_OBJECTS],
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            background_colors: BackgroundColors::new(),
            viewport_x_offset: 0,
            viewport_y_offset: 0,
            lcd_display_enabled: false,
            window_display_enabled: false,
            background_display_enabled: false,
            object_display_enabled: false,
            line_equals_line_check_interrupt_enabled: false,
            oam_interrupt_enabled: false,
            vblank_interrupt_enabled: false,
            hblank_interrupt_enabled: false,
            line_check: 0,
            line_equals_line_check: false,
            window_tile_map: TileMap::X9800,
            background_tile_map: TileMap::X9800,
            background_and_window_data_select: BackgroundAndWindowDataSelect::X8800,
            object_size: ObjectSize::OS8X8,
            obj_0_color_1: Color::LightGray,
            obj_0_color_2: Color::DarkGray,
            obj_0_color_3: Color::Black,
            obj_1_color_1: Color::LightGray,
            obj_1_color_2: Color::DarkGray,
            obj_1_color_3: Color::Black,
            window: Window { x: 0, y: 0 },
            line: 0,
            cycles: 0,
            mode: Mode::HorizontalBlank,
        }
    }

    pub fn gpu_read(&mut self, mut address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => {
                address -= 0x8000;
                self.vram[address as usize]
            }
            0xFE00..=0xFE9F => {
                address -= 0xFE00;
                self.oam[address as usize]
            },
            0xFF40 => {
                bit(self.lcd_display_enabled) << 7
                    | bit(self.window_tile_map == TileMap::X9C00) << 6
                    | bit(self.window_display_enabled) << 5
                    | bit(self.background_and_window_data_select
                    == BackgroundAndWindowDataSelect::X8000)
                    << 4
                    | bit(self.background_tile_map == TileMap::X9C00) << 3
                    | bit(self.object_size == ObjectSize::OS8X16) << 2
                    | bit(self.object_display_enabled) << 1
                    | bit(self.background_display_enabled)
            },
            0xFF42 => self.viewport_y_offset,
            0xFF44 => self.line, //0x90 when blargss else self.line
            _ => panic!("GPU read address not implemented: {:04X}", address)
        }
    }

    pub fn gpu_write(&mut self, mut address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => {
                address -= 0x8000;
                self.write_vram(address as usize, value);
            }
            0xFE00..=0xFE9F => {
                address -= 0xFE00;
                self.write_oam(address as usize, value);
            },

            0xFF40 => {
                // LCD Control
                self.lcd_display_enabled = (value >> 7) == 1;
                self.window_tile_map = if ((value >> 6) & 0b1) == 1 {
                    TileMap::X9C00
                } else {
                    TileMap::X9800
                };
                self.window_display_enabled = ((value >> 5) & 0b1) == 1;
                self.background_and_window_data_select = if ((value >> 4) & 0b1) == 1 {
                    BackgroundAndWindowDataSelect::X8000
                } else {
                    BackgroundAndWindowDataSelect::X8800
                };
                self.background_tile_map = if ((value >> 3) & 0b1) == 1 {
                    TileMap::X9C00
                } else {
                    TileMap::X9800
                };
                self.object_size = if ((value >> 2) & 0b1) == 1 {
                    ObjectSize::OS8X16
                } else {
                    ObjectSize::OS8X8
                };
                self.object_display_enabled = ((value >> 1) & 0b1) == 1;
                self.background_display_enabled = (value & 0b1) == 1;
            }
            0xFF41 => {
                // LCD Controller Status
                self.line_equals_line_check_interrupt_enabled =
                    (value & 0b1000000) == 0b1000000;
                self.oam_interrupt_enabled = (value & 0b100000) == 0b100000;
                self.vblank_interrupt_enabled = (value & 0b10000) == 0b10000;
                self.hblank_interrupt_enabled = (value & 0b1000) == 0b1000;
            }
            0xFF42 => {
                // Viewport Y Offset
                self.viewport_y_offset = value;
            }
            0xFF43 => {
                // Viewport X Offset
                self.viewport_x_offset = value;
            }
            0xFF45 => {
                self.line_check = value;
            }
            0xFF47 => {
                // Background Colors Setting
                self.background_colors = value.into();
            }
            0xFF48 => {
                self.obj_0_color_3 = (value >> 6).into();
                self.obj_0_color_2 = ((value >> 4) & 0b11).into();
                self.obj_0_color_1 = ((value >> 2) & 0b11).into();
            }
            0xFF49 => {
                self.obj_1_color_3 = (value >> 6).into();
                self.obj_1_color_2 = ((value >> 4) & 0b11).into();
                self.obj_1_color_1 = ((value >> 2) & 0b11).into();
            }
            0xFF4A => {
                self.window.y = value;
            }
            0xFF4B => {
                self.window.x = value;
            }
            _ => ()
        }
    }

    pub fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;
        if index >= 0x1800 {
            return;
        }


        let normalized_index = index & 0xFFFE;

        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];


        let tile_index = index / 16;
        let row_index = (index % 16) / 2;

        for pixel_index in 0..8 {

            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;


            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    pub fn write_oam(&mut self, index: usize, value: u8) {
        self.oam[index] = value;
        let object_index = index / 4;
        if object_index > NUMBER_OF_OBJECTS {
            return;
        }

        let byte = index % 4;

        let mut data = self.object_data.get_mut(object_index).unwrap();
        match byte {
            0 => data.y = (value as i16) - 0x10,
            1 => data.x = (value as i16) - 0x8,
            2 => data.tile = value,
            _ => {
                data.palette = if (value & 0x10) != 0 {
                    ObjectPalette::One
                } else {
                    ObjectPalette::Zero
                };
                data.xflip = (value & 0x20) != 0;
                data.yflip = (value & 0x40) != 0;
                data.priority = (value & 0x80) == 0;
            }
        }
    }

    pub fn step(&mut self, cycles: u8) -> InterruptRequest {
        let mut request = InterruptRequest::None;
        if !self.lcd_display_enabled {
            return request;
        }
        self.cycles += cycles as u16;

        let mode = self.mode;
        match mode {
            Mode::HorizontalBlank => {
                if self.cycles >= 200 {
                    self.cycles = self.cycles % 200;
                    self.line += 1;

                    if self.line >= 144 {
                        self.mode = Mode::VerticalBlank;
                        request.add(InterruptRequest::VBlank);
                        if self.vblank_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    } else {
                        self.mode = Mode::OAMAccess;
                        if self.oam_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::VerticalBlank => {
                if self.cycles >= 456 {
                    self.cycles = self.cycles % 456;
                    self.line += 1;
                    if self.line == 154 {
                        self.mode = Mode::OAMAccess;
                        self.line = 0;
                        if self.oam_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::OAMAccess => {
                if self.cycles >= 80 {
                    self.cycles = self.cycles % 80;
                    self.mode = Mode::VRAMAccess;
                }
            }
            Mode::VRAMAccess => {
                if self.cycles >= 172 {
                    self.cycles = self.cycles % 172;
                    if self.hblank_interrupt_enabled {
                        request.add(InterruptRequest::LCDStat)
                    }
                    self.mode = Mode::HorizontalBlank;
                    self.render_scan_line()
                }
            }
        }
        request
    }

    fn set_equal_lines_check(&mut self, request: &mut InterruptRequest) {
        let line_equals_line_check = self.line == self.line_check;
        if line_equals_line_check && self.line_equals_line_check_interrupt_enabled {
            request.add(InterruptRequest::LCDStat);
        }
        self.line_equals_line_check = line_equals_line_check;
    }

    pub fn background_as_buffer(&self, outline_tiles: bool, show_viewport: bool) -> Vec<u8> {
        if self.background_tile_map != TileMap::X9800 {
            panic!("We only support tilemap at 0x9800 for now");
        }

        let width_in_tiles = 32;
        let height_in_tiles = 32;

        let tile_width_in_pixels = 8;
        let tile_height_in_pixels = 8;

        let values_per_pixel = 4;

        let row_width_in_canvas_values = tile_width_in_pixels * width_in_tiles * values_per_pixel;

        let data_length = width_in_tiles
            * height_in_tiles
            * tile_height_in_pixels
            * tile_width_in_pixels
            * values_per_pixel;
        let mut data = vec![0; data_length];

        let tiles = self
            .background_1()
            .iter()
            .map(|byte| self.tile_set[*byte as usize]);

        for (tile_index, tile) in tiles.enumerate() {
            let tile_row = tile_index / height_in_tiles;
            let tile_column = tile_index % width_in_tiles;
            let final_tile_row = tile_row == height_in_tiles - 1;
            let final_tile_column = tile_column == width_in_tiles - 1;

            for (row_index, row) in tile.iter().enumerate() {
                let pixel_row_index = (tile_row * tile_height_in_pixels) + row_index;
                let beginning_of_canvas_row = pixel_row_index * row_width_in_canvas_values;
                let beginning_of_column = tile_column * tile_width_in_pixels;
                let final_pixel_row = final_tile_row && row_index == 7;
                let mut index = beginning_of_canvas_row + (beginning_of_column * values_per_pixel);

                for (pixel_index, pixel) in row.iter().enumerate() {
                    let pixel_column_index = beginning_of_column + pixel_index;
                    let viewport_x_offset = self.viewport_x_offset as usize;
                    let viewport_y_offset = self.viewport_y_offset as usize;
                    let (screen_border_right, did_overflow_x) =
                        self.viewport_x_offset.overflowing_add(SCREEN_WIDTH as u8);
                    let (screen_border_bottom, did_overflow_y) =
                        self.viewport_y_offset.overflowing_add(SCREEN_HEIGHT as u8);
                    let is_inside_screen_horizontally = if did_overflow_x {
                        pixel_column_index < (screen_border_right as usize)
                            || pixel_column_index > viewport_x_offset
                    } else {
                        pixel_column_index < (screen_border_right as usize)
                            && pixel_column_index > viewport_x_offset
                    };
                    let is_on_screen_horizontal_edge = viewport_y_offset == pixel_row_index
                        || pixel_row_index == (screen_border_bottom as usize);
                    let on_screen_border_x =
                        is_inside_screen_horizontally && is_on_screen_horizontal_edge;

                    let is_inside_screen_vertically = if did_overflow_y {
                        pixel_row_index < (screen_border_bottom as usize)
                            || pixel_row_index > viewport_y_offset
                    } else {
                        pixel_row_index < (screen_border_bottom as usize)
                            && pixel_row_index > viewport_y_offset
                    };
                    let is_on_screen_vertical_edge = viewport_x_offset == pixel_column_index
                        || pixel_column_index == (screen_border_right as usize);
                    let on_screen_border_y =
                        is_inside_screen_vertically && is_on_screen_vertical_edge;

                    let on_tile_border_x = pixel_row_index % 8 == 0;
                    let on_tile_border_y = pixel_column_index % 8 == 0;
                    let final_pixel_column = final_tile_column && pixel_index == 7;

                    if show_viewport && (on_screen_border_x || on_screen_border_y) {
                        data[index] = 255;
                        data[index + 1] = 0;
                        data[index + 2] = 0;
                    } else if outline_tiles
                        && (on_tile_border_x
                        || on_tile_border_y
                        || final_pixel_row
                        || final_pixel_column)
                    {
                        data[index] = 0;
                        data[index + 1] = 0;
                        data[index + 2] = 255;
                    } else {
                        let color = self.tile_value_to_background_color(pixel);
                        data[index] = color as u8;
                        data[index + 1] = color as u8;
                        data[index + 2] = color as u8;
                    }
                    data[index + 3] = 255;

                    index = index + values_per_pixel;
                }
            }
        }

        data
    }

    pub fn tile_set_as_buffer(&self, outline_tiles: bool) -> Vec<u8> {
        let values_per_pixel = 4;
        let tile_width = 8;
        let tile_height = 8;

        let width_in_tiles = 24;
        let height_in_tiles = self.tile_set.len() / width_in_tiles;

        let row_width = tile_width * width_in_tiles * values_per_pixel;
        let mut data =
            vec![0; width_in_tiles * height_in_tiles * tile_height * tile_width * values_per_pixel];

        for (tile_index, tile) in self.tile_set.iter().enumerate() {
            let tile_row = tile_index / width_in_tiles;
            let tile_column = tile_index % width_in_tiles;
            let final_tile_row = tile_row == height_in_tiles - 1;
            let final_tile_column = tile_column == width_in_tiles - 1;

            for (row_index, row) in tile.iter().enumerate() {
                let pixel_row_index = (tile_row * tile_height) + row_index;
                let beginning_of_canvas_row = pixel_row_index * row_width;
                let on_tile_row_border = pixel_row_index % 8 == 0;
                let beginning_of_column = tile_column * tile_width;
                let final_pixel_row = final_tile_row && row_index == 7;
                let mut index = beginning_of_canvas_row + (beginning_of_column * values_per_pixel);

                for (pixel_index, pixel) in row.iter().enumerate() {
                    let on_tile_column_border = pixel_index == 0;
                    let final_pixel_column = final_tile_column && pixel_index == 7;
                    if outline_tiles
                        && (on_tile_row_border
                        || on_tile_column_border
                        || final_pixel_row
                        || final_pixel_column)
                    {
                        data[index] = 0;
                        data[index + 1] = 0;
                        data[index + 2] = 255;
                    } else {
                        let color = self.tile_value_to_background_color(pixel);
                        data[index] = color as u8;
                        data[index + 1] = color as u8;
                        data[index + 2] = color as u8;
                    }
                    data[index + 3] = 255;
                    index = index + values_per_pixel;
                }
            }
        }

        data
    }

    // Get a specific tile at the specified coordinates within the entire background space
    pub fn get_tile_buffer_at(&self, pixel_x: usize, pixel_y: usize) -> [[u8; 8]; 8] {
        let tile_x = pixel_x / 8;
        let tile_y = pixel_y / 8;

        let index = (tile_y * 32) + tile_x;
        let mut result = [[0u8; 8]; 8];
        let byte = self.background_1().iter().nth(index).unwrap();
        let tile = self.tile_set[*byte as usize];
        for (row_index, row) in tile.iter().enumerate() {
            for (pixel_index, pixel) in row.iter().enumerate() {
                result[row_index][pixel_index] = self.tile_value_to_background_color(pixel) as u8;
            }
        }

        result
    }

    fn background_1(&self) -> &[u8] {
        &self.vram[0x1800..0x1C00]
    }

    fn render_scan_line(&mut self) {
        let mut scan_line: [TilePixelValue; SCREEN_WIDTH] = [Default::default(); SCREEN_WIDTH];
        if self.background_display_enabled {

            let mut tile_x_index = self.viewport_x_offset / 8;

            let tile_y_index = self.line.wrapping_add(self.viewport_y_offset);

            let tile_offset = (tile_y_index as u16 / 8) * 32u16;

            let background_tile_map = if self.background_tile_map == TileMap::X9800 {
                0x9800
            } else {
                0x9C00
            };
            let tile_map_begin = background_tile_map - 0x8000;

            let tile_map_offset = tile_map_begin + tile_offset as usize;


            let row_y_offset = tile_y_index % 8;
            let mut pixel_x_index = self.viewport_x_offset % 8;

            if self.background_and_window_data_select == BackgroundAndWindowDataSelect::X8800 {
                panic!("TODO: support 0x8800 background and window data select");
            }

            let mut canvas_buffer_offset = self.line as usize * SCREEN_WIDTH * 4;
            for line_x in 0..SCREEN_WIDTH {
                let tile_index = self.vram[tile_map_offset + tile_x_index as usize];

                let tile_value = self.tile_set[tile_index as usize][row_y_offset as usize]
                    [pixel_x_index as usize];
                let color = self.tile_value_to_background_color(&tile_value);

                self.canvas_buffer[canvas_buffer_offset] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 1] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 2] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 3] = 255;
                canvas_buffer_offset += 4;
                scan_line[line_x] = tile_value;
                pixel_x_index = (pixel_x_index + 1) % 8;

                if pixel_x_index == 0 {
                    tile_x_index = tile_x_index + 1;
                }
                if self.background_and_window_data_select == BackgroundAndWindowDataSelect::X8800 {
                    panic!("TODO: support 0x8800 background and window data select");
                }
            }
        }

        if self.object_display_enabled {
            let object_height = if self.object_size == ObjectSize::OS8X16 {
                16
            } else {
                8
            };
            for object in self.object_data.iter() {
                let line = self.line as i16;
                if object.y <= line && object.y + object_height > line {
                    let pixel_y_offset = line - object.y;
                    let tile_index = if object_height == 16 && (!object.yflip && pixel_y_offset > 7)
                        || (object.yflip && pixel_y_offset <= 7)
                    {
                        object.tile + 1
                    } else {
                        object.tile
                    };

                    let tile = self.tile_set[tile_index as usize];
                    let tile_row = if object.yflip {
                        tile[(7 - (pixel_y_offset % 8)) as usize]
                    } else {
                        tile[(pixel_y_offset % 8) as usize]
                    };

                    let canvas_y_offset = line as i32 * SCREEN_WIDTH as i32;
                    let mut canvas_offset = ((canvas_y_offset + object.x as i32) * 4) as usize;
                    for x in 0..8i16 {
                        let pixel_x_offset = if object.xflip { (7 - x) } else { x } as usize;
                        let x_offset = object.x + x;
                        let pixel = tile_row[pixel_x_offset];
                        if x_offset >= 0
                            && x_offset < SCREEN_WIDTH as i16
                            && pixel != TilePixelValue::Zero
                            && (object.priority
                            || scan_line[x_offset as usize] == TilePixelValue::Zero)
                        {
                            let color = self.tile_value_to_background_color(&pixel);

                            self.canvas_buffer[canvas_offset + 0] = color as u8;
                            self.canvas_buffer[canvas_offset + 1] = color as u8;
                            self.canvas_buffer[canvas_offset + 2] = color as u8;
                            self.canvas_buffer[canvas_offset + 3] = 255;
                        }
                        canvas_offset += 4;
                    }
                }
            }
        }

        if self.window_display_enabled {}
    }

    fn tile_value_to_background_color(&self, tile_value: &TilePixelValue) -> Color {
        match tile_value {
            TilePixelValue::Zero => self.background_colors.0,
            TilePixelValue::One => self.background_colors.1,
            TilePixelValue::Two => self.background_colors.2,
            TilePixelValue::Three => self.background_colors.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_set_buffer() {
        let gpu = GPU::new();
        gpu.tile_set_as_buffer(false);
    }
}