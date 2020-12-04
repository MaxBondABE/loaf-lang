use image::{Rgb, RgbImage};
use std::{collections::HashMap, fs::create_dir, path::Path};
use crate::lang::runtime::naive::{Coordinate, DimensionBounds};
use crate::lang::runtime::naive::StateId;
use crate::render::Output;

use super::Delta;

const FRAMES_DIRECTORY:&str = "loaf-frames";

pub struct Render2D {
    colors_map: HashMap<StateId, image::Rgb<u8>>,
    buffer: image::RgbImage,
    cell_width: u32,
    x_shift: isize,
    y_shift: isize,
    name: String,
    frame_number: usize
}

impl Render2D {
    pub fn new(colors_map: HashMap<StateId, image::Rgb<u8>>, bounds: DimensionBounds, default_color: Rgb<u8>, name: String,
               cell_width: u32) -> Self {
        let width = cell_width * bounds.x_breadth() as u32;
        let height = cell_width * bounds.y_breadth() as u32;
        let mut buffer = RgbImage::from_pixel(width, height, default_color);
        let (x_shift, y_shift) = match bounds {
            DimensionBounds::DimensionBounds2D { x: (x_low, _), y: (y_low, _) } => {
                (-x_low, -y_low)
            },
            _ => panic!("Wrong dimensions")
        };
        let frames_dir = Path::new(FRAMES_DIRECTORY);
        if !frames_dir.exists() {
            create_dir(frames_dir).expect("Failed to create frames directory");
        } else if !frames_dir.is_dir() {
            panic!("Could not create frames directory: {} exists & is not a directory.", FRAMES_DIRECTORY);
        }
        Self {
            colors_map,
            buffer,
            cell_width,
            x_shift,
            y_shift,
            name,
            frame_number: 0
        }
    }

    pub fn draw_frame(&mut self, delta: Delta) {
        for (coord, state_id) in delta {
            self.draw_cell(coord, *self.colors_map.get(&state_id).expect("Colors map should be complete"));
        }
        self.save();
        self.frame_number += 1;
    }

    fn draw_cell(&mut self, coord: Coordinate, color: image::Rgb<u8>) {
        let (x, y) = match coord {
            Coordinate::Coordinate2D { x, y } => (x, y),
            _ => panic!("Wrong dimensions")
        };
        let x = self.cell_width * (x + self.x_shift) as u32;
        let y = self.cell_width * (y + self.y_shift) as u32;
        for px_x in (x..(x+self.cell_width)) {
            for px_y in (y..(y+self.cell_width)) {
                let px = self.buffer.get_pixel_mut(px_x, px_y);
                *px = color;
            }
        }
    }

    fn save(&self) {
        self.buffer.save(format!("{}/{}_frame_{}.jpg", FRAMES_DIRECTORY, self.name, self.frame_number)).unwrap();
    }
}
impl Output for Render2D {
    fn output_tick(&mut self, delta: Delta) {
        self.draw_frame(delta);
    }
}

#[cfg(test)]
mod test {
    use super::*;

//     #[test]
//     fn draw_boundaries() {
//         let mut colors_map = HashMap::new();
//         colors_map.insert(0, image::Rgb([0, 0, 0]));
//         let dims = DimensionBounds::DimensionBounds2D { x: (-5, 5), y: (-5, 5) };
//         let mut out = Output::new(
//             colors_map,
//             dims,
//             Rgb([0xff, 0xff, 0xff]),
//             3
//         );
//         for coord in dims {
//             if !dims.boundary(coord) {
//                 continue;
//             }
//             out.draw_cell(coord, image::Rgb([0, 0, 0]));
//         }
//         out.save();
//     }


    #[test]
    fn draw_frames() {
        let mut colors_map = HashMap::new();
        colors_map.insert(0, image::Rgb([0, 0, 0]));
        colors_map.insert(1, image::Rgb([0xff, 0xff, 0xff]));
        let dims = DimensionBounds::DimensionBounds2D { x: (-5, 5), y: (-5, 5) };
        let mut out = Render2D::new(
            colors_map,
            dims,
            Rgb([0xff, 0xff, 0xff]),
            "MyAnimation".into(),
            50
        );
        out.draw_frame(vec!((Coordinate::Coordinate2D {x: 0, y: 0}, 0)));
        out.draw_frame(vec!((Coordinate::Coordinate2D {x: 0, y: 0}, 1)));
        out.draw_frame(vec!((Coordinate::Coordinate2D {x: 0, y: 0}, 0)));
        out.draw_frame(vec!((Coordinate::Coordinate2D {x: 0, y: 0}, 1)));
    }

}
