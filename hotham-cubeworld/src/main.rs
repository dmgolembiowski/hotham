use std::path::Path;

use cgmath::vec3;
use hotham::{App, HothamResult as Result, Program, ProgramInitialization, Vertex};

#[derive(Debug, Clone)]
struct Cubeworld {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Cubeworld {
    fn new() -> Self {
        let v1 = Vertex::new(vec3(-1.0, -1.0, 1.0), vec3(1.0, 0.0, 0.0));
        let v2 = Vertex::new(vec3(1.0, 1.0, 1.0), vec3(0.0, 0.0, 1.0));
        let v3 = Vertex::new(vec3(-1.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0));
        let vertices = vec![v1, v2, v3];
        let indices = vec![0, 1, 2];
        Self { vertices, indices }
    }
}

impl Program for Cubeworld {
    fn init(&self) -> ProgramInitialization {
        let vertex_shader = Path::new("src/shaders/cube.vert.spv");
        let fragment_shader = Path::new("src/shaders/cube.frag.spv");

        ProgramInitialization {
            vertices: &self.vertices,
            vertex_shader,
            indices: &self.indices,
            fragment_shader,
        }
    }

    fn update(&self) -> (&Vec<Vertex>, &Vec<u32>) {
        (&self.vertices, &self.indices)
    }
}

fn main() -> Result<()> {
    let program = Cubeworld::new();
    let mut app = App::new(program)?;
    app.run()
}