use colorgrad::{Color, Gradient};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use minifb_fonts::{font6x8, FbFontRenderer};
use minifb_geometry::GeometryDrawer;

use crate::core::World;



/// Struct to represent the canvas for drawing the simulation
pub struct Canvas{
    width: usize,
    height: usize,

    buffer: Vec<u32>,
    geometry_set: GeometryDrawer,
    window: Window,
    font_renderer: FbFontRenderer,
    cmap: Vec<Color>
}

impl Canvas{
    // *********** PUBLIC ***********

    /// Create a new canvas with a given height/width in pixels
    pub fn new(width:usize, height: usize) -> Canvas {

        let mut window= Window::new(
            "LifeFromScratch",
            width,
            height,
            WindowOptions {
                resize: true,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            }
        )
        .expect("Unable to create the window");

        window.set_target_fps(0);

        Canvas{ width: width, 
            height: height, 
            buffer: vec![0u32; width*height], 
            geometry_set: GeometryDrawer::new(width),
            window: window,
            font_renderer: font6x8::new_renderer(width, height, 0xffffff),
            cmap: colorgrad::preset::rainbow().colors(256)
        }
    }

    /// Draw the world at a given time to the canvas
    pub fn draw(&mut self, world: &World, time: &u64){
        if self.window.is_open() && !self.window.is_key_down(Key::Escape){
            
            // Clear the buffer
            self.buffer = vec![0u32; self.width*self.height];
            
            // Write time to buffer
            self.font_renderer.draw_text(&mut self.buffer, 10, 10, format!("Time: {}", time).as_str());

            // Draw a box
            // let _ = self.geometry_set.draw_box(&mut self.buffer, 120, 130, 220, 230, 0xffff00);

            self.world_to_buffer(world);
            
            // Write canvas to the buffer
            self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        }

    }

    // *********** PRIVATE ***********
    /// Update buffer with particles in the world
    fn world_to_buffer(&mut self, world: &World ){
        //let _= self.geometry_set.draw_box(&mut self.buffer, 120, 130, 220, 230, 0xffff00);

        for atom in world.atom_iter(){
            let col = self.get_col_int(atom.species());
            let _ = self.geometry_set.draw_circle(&mut self.buffer, 
                ((atom.x()/world.size_x()) * self.width as f32) as usize, 
                ((atom.y()/world.size_y()) * self.height as f32) as usize,
                ((atom.r()/world.size_x()) * self.width as f32) as usize,
                col);
        }
    }

    /// Get the color based on an atom species as an integer
    fn get_col_int(&self, spec: u8) -> usize {
        let rgb = self.cmap[spec as usize].to_rgba8();
        let colint: usize = rgb[0] as usize * 256 * 256 + rgb[1] as usize * 256 + rgb[2] as usize;
        return colint as usize
    }

    

}