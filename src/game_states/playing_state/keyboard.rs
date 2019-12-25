use glium::Surface;

pub struct KeyboardRenderer<'a> {
  display: &'a glium::Display,
  program: glium::Program,
  vertex_buffer: glium::VertexBuffer<Vertex>,
  indices: glium::IndexBuffer<u16>,
}

#[derive(Copy, Clone)]
struct Vertex {
  pos: [f32; 2],
}
implement_vertex!(Vertex, pos);

impl<'a> KeyboardRenderer<'a> {
  pub fn new(display: &'a glium::Display) -> Self {
    let vertex1 = Vertex { pos: [-1.0, -1.0] };
    let vertex2 = Vertex { pos: [1.0, -1.0] };
    let vertex3 = Vertex { pos: [1.0, 1.0] };
    let vertex4 = Vertex { pos: [-1.0, 1.0] };

    let shape: [Vertex; 4] = [vertex1, vertex2, vertex3, vertex4];
    let indices_vec: [u16; 6] = [0, 1, 3, 3, 1, 2];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(
      display,
      glium::index::PrimitiveType::TrianglesList,
      &indices_vec,
    )
    .unwrap();

    let vertex_shader_src = include_str!("../../shaders/keyboard.vert");
    let fragment_shader_src = include_str!("../../shaders/keyboard.frag");

    let program = glium::Program::new(
      display,
      glium::program::ProgramCreationInput::SourceCode {
        vertex_shader: vertex_shader_src,
        fragment_shader: fragment_shader_src,
        geometry_shader: None,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        transform_feedback_varyings: None,
        outputs_srgb: true,
        uses_point_size: false,
      },
    )
    .unwrap();

    Self {
      display,
      program,
      vertex_buffer,
      indices,
    }
  }
  pub fn draw(&self, target: &mut glium::Frame, viewport: &glium::Rect, active_notes: [bool; 88]) {
    let notes: glium::uniforms::UniformBuffer<[u32; 128]> =
      glium::uniforms::UniformBuffer::empty_dynamic(self.display).unwrap();

    let mut notes_data = [128; 128];

    for (i, n) in active_notes.iter().enumerate() {
      if *n {
        notes_data[i] = i as u32;
      }
    }

    notes.write(&notes_data);

    target
      .draw(
        &self.vertex_buffer,
        &self.indices,
        &self.program,
        &uniform! {ActiveNotes: &notes},
        &glium::DrawParameters {
          viewport: Some(viewport.to_owned()),
          ..Default::default()
        },
      )
      .unwrap();
  }
}