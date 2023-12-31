use std::{collections::HashMap, fmt::Debug};

pub trait Vertexable {
    fn get_value(&self) -> char;
}

pub struct Grid {
    pub width: isize,
    pub height: isize,
    vertices: HashMap<(isize, isize), Box<dyn Vertexable>>,
}

impl Grid {
    pub fn new(width: isize, height: isize) -> Grid {
        let vertices = HashMap::new();
        Grid {
            width,
            height,
            vertices,
        }
    }

    pub fn is_inside(&self, location: &(isize, isize)) -> bool {
        location.0 < self.width && location.1 < self.height
    }

    pub fn add_vertex(&mut self, location: (isize, isize), vertex: Box<dyn Vertexable>) {
        if !self.is_inside(&location) {
            println!(
                "Warning: location ({},{}) is outside of grid with size ({},{})",
                location.0, location.1, self.width, self.height
            );
            return;
        }

        self.vertices.entry(location).or_insert(vertex);
    }

    pub fn get(&self, location: &(isize, isize)) -> Option<&Box<dyn Vertexable>> {
        if !self.is_inside(location) {
            println!(
                "Warning: location ({},{}) is outside of grid with size ({},{})",
                location.0, location.1, self.width, self.height
            );
            return None;
        }

        self.vertices.get(location)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(v) = self.vertices.get(&(x, y)) {
                    write!(f, "{}", v.get_value())?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
