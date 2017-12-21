
use std::io::prelude::*;
use std::fs::File;

struct Buffer{
    width: usize,
    height: usize,
    data: Box<[u32]>
}

pub struct Canvas{
    buffer: Buffer,
    pub px: isize,
    pub py: isize,
    pub mx: f64,
    pub my: f64,
    pub x: f64,
    pub y: f64,
    pub wx: f64,
    pub wy: f64,
    pub n: usize
}

#[derive(Clone,Copy)]
pub struct Color{
    pub value: u32
}
impl Color{
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color{value: (r as u32)<<16 | (g as u32)<<8 | (b as u32)}
    }
}

const BLACK: Color = Color{value: 0x00000000};
const GRAY:  Color = Color{value: 0x00808080};
const WHITE: Color = Color{value: 0x00ffffff};
const BLUE:  Color = Color{value: 0x00000080};
const LIGHTGRAY: Color = Color{value: 0x00e4e4e0};


impl Canvas{
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<u32> = vec![0; width*height];
        let buffer = Buffer{width,height,data: data.into_boxed_slice()};
        return Canvas{buffer, px: (width/2) as isize, py: (height/2) as isize,
            mx: 40.0, my: 40.0, wx: 10.0, wy: 10.0, x: 0.0, y: 0.0, n: 1000
        };
    }
    fn hline(&mut self, y: f64, color: Color) {
        let py = (self.py-(y*self.mx) as isize) as usize;
        let w = self.buffer.width;
        self.fill(0,py,w,2,color);
    }
    fn vline(&mut self, x: f64, color: Color) {
        let px = (self.px+(x*self.mx) as isize) as usize;
        let h = self.buffer.height;
        self.fill(px,0,2,h,color);
    }
    pub fn system(&mut self) {
        self.clear(WHITE);
        for x in 1..10 {
          self.vline(x as f64,LIGHTGRAY);
        }
        for x in -9..0 {
          self.vline(x as f64,LIGHTGRAY);         
        }
        for y in 1..10 {
          self.hline(y as f64,LIGHTGRAY);
        }
        for y in -9..0 {
          self.hline(y as f64,LIGHTGRAY);
        }
        self.hline(0.0,GRAY);
        self.vline(0.0,GRAY);
    }
    pub fn clear(&mut self, color: Color) {
        for x in self.buffer.data.iter_mut() {
            *x = color.value;
        }
    }
    pub fn fill(&mut self, px: usize, py: usize, w: usize, h: usize, color: Color) {
        let width = self.buffer.width;
        let height = self.buffer.height;
        let data = &mut self.buffer.data;
        for x in px..px.wrapping_add(w) {
            for y in py..py.wrapping_add(h) {
                if y<height && x<width {
                  data[y*width+x] = color.value;
                }
            }
        }
    }
    pub fn ppm(&self) -> Vec<u8> {
        let s = format!("P6 {} {} 255\n",self.buffer.width,self.buffer.height);
        let mut bv: Vec<u8> = s.into_bytes();
        for x in self.buffer.data.iter() {
            let r = ((*x)>>16) as u8;
            let g = ((*x)>>8) as u8;
            let b = (*x) as u8;
            bv.push(r);
            bv.push(g);
            bv.push(b);
        }
        return bv;
    }
    pub fn save(&self, id: &str) {
        let ppm = self.ppm();
        save(&ppm,id);
    }
    pub fn plot(&mut self, f: &Fn(f64)->f64) {
        let mut x = self.x-self.wx;
        let mut xe = self.x+self.wx;
        let d = 10.0/(self.n as f64);
        while x<xe {
            let y = f(x);
            let px = (self.px+(x*self.mx) as isize) as usize;
            let py = (self.py.wrapping_sub((y*self.mx) as isize)) as usize;
            // println!("({}|{})",px,py);
            self.fill(px,py,2,2,BLUE);
            x+=d;
        }
    }
}

pub fn save(bv: &Vec<u8>, id: &str) {
    let mut buffer = match File::create(id) {
        Ok(file)=>file,
        Err(_) => {
            println!("Error: file '{}' could not be opened to write.",id);
            return;
        }
    };
    match buffer.write(bv) {
        Ok(_)=>{},
        Err(_) => {
            println!("Error: could not write into file '{}'.",id);        
        }
    }
}

