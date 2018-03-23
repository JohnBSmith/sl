
use std::io::prelude::*;
use std::fs::File;
mod font;

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
    pub n: usize,
    pub color: Color,
    color_index: usize
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

// const BLACK: Color = Color{value: 0x00000000};
const GRAY:  Color = Color{value: 0x00808080};
const WHITE: Color = Color{value: 0x00ffffff};
const BLUE:  Color = Color{value: 0x00000080};
const LIGHTGRAY: Color = Color{value: 0x00e4e4e0};

pub mod color{
    use Color;
    pub const BLACK:   Color = Color{value: 0x00000000};
    pub const GRAY:    Color = Color{value: 0x00808080};
    pub const WHITE:   Color = Color{value: 0x00ffffff};
    pub const BLUE:    Color = Color{value: 0x00000080};
    pub const MAGENTA: Color = Color{value: 0x00800060};
    pub const GREEN:   Color = Color{value: 0x00006000};
    pub const RED:     Color = Color{value: 0x00a00000};
    pub const BROWN:   Color = Color{value: 0x00808000};
    pub const LIGHTGRAY: Color = Color{value: 0x00e4e4e0};
}

static COLOR_TAB: [Color;4] = [
  color::BLUE,
  color::GREEN,
  color::MAGENTA,
  color::BLACK
];

impl Canvas{
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<u32> = vec![0; width*height];
        let buffer = Buffer{width,height,data: data.into_boxed_slice()};
        return Canvas{buffer, px: (width/2) as isize, py: (height/2) as isize,
            mx: 40.0, my: 40.0, wx: 10.0, wy: 10.0, x: 0.0, y: 0.0, n: 1000,
            color: BLUE, color_index: 0
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
        self.color = GRAY;
        for x in 1..10 {
          self.vline(x as f64,LIGHTGRAY);
          self.vprint(x as f64-0.1, -0.1, &format!("{}",x));
        }
        for x in -9..0 {
          self.vline(x as f64,LIGHTGRAY);
          self.vprint(x as f64-0.1, -0.1, &format!("{}",x));
        }
        for y in 1..10 {
          self.hline(y as f64,LIGHTGRAY);
          self.vprint(0.2, y as f64+0.2, &format!("{}",y));
        }
        for y in -9..0 {
          self.hline(y as f64,LIGHTGRAY);
          self.vprint(0.2, y as f64+0.2, &format!("{}",y));
        }
        self.hline(0.0,GRAY);
        self.vline(0.0,GRAY);
        self.color = BLUE;
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
    pub fn empty_box(&mut self, px: usize, py: usize, w: usize, color: Color) {
        self.fill(px,py,w,2,color);
        self.fill(px,py,2,w,color);
        self.fill(px,py.wrapping_add(w),w+2,2,color);
        self.fill(px.wrapping_add(w),py,2,w,color);
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
        let xe = self.x+self.wx;
        let d = 10.0/(self.n as f64);
        while x<xe {
            let y = f(x);
            let px = (self.px+(x*self.mx) as isize) as usize;
            let py = (self.py.wrapping_sub((y*self.mx) as isize)) as usize;
            // println!("({}|{})",px,py);
            let color = self.color;
            self.fill(px,py,2,2,color);
            x+=d;
        }
        self.color_index+=1;
        let n = COLOR_TAB.len();
        let i = self.color_index;
        self.color = COLOR_TAB[i%n];
    }
    pub fn scatter(&mut self, a: &[[f64;2]]) {
        for t in a {
            let px = (self.px+(t[0]*self.mx) as isize) as usize;
            let py = (self.py.wrapping_sub((t[1]*self.mx) as isize)) as usize;
            let color = self.color;
            self.empty_box(px.wrapping_sub(4),py.wrapping_sub(4),8,color);
        }
    }
    pub fn pixels(&mut self, px: usize, py: usize, s: &str) {
        let color = self.color;
        let mut x = px;
        let mut y = py;
        for c in s.chars() {
            if c=='x' {
                self.fill(x,y,2,2,color);
                x = x.wrapping_add(2);
            }else if c=='\n' {
                x = px;
                y = y.wrapping_add(2);
            }else{
                x = x.wrapping_add(2);
            }
        }
    }
    fn get_pxpy(&self, x: f64, y: f64) -> (usize,usize) {
        let px = (self.px.wrapping_add((x*self.mx) as isize)) as usize;
        let py = (self.py.wrapping_sub((y*self.mx) as isize)) as usize;
        return (px,py);
    }
    pub fn print(&mut self, px: usize, py: usize, s: &str) {
        let mut x = px;
        let mut y = py;
        for c in s.chars() {
            let m = ::font::pixelmap(c);
            if c=='\n' {
                y = y.wrapping_add(14);
                x=px;
            }else{
                self.pixels(x,y,m);
                x = x.wrapping_add(12);
            }
        }
    }
    pub fn vprint(&mut self, x: f64, y: f64, s: &str) {
        let (px,py) = self.get_pxpy(x,y);
        self.print(px,py,s);
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

