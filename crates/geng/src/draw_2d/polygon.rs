use super::*;

pub struct Polygon {
    transform: Mat3<f32>,
    vertices: Vec<ColoredVertex>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vec2<f32>>, color: Color<f32>) -> Self {
        Self::new_gradient(
            vertices
                .into_iter()
                .map(|vertex| ColoredVertex {
                    a_pos: vertex,
                    a_color: color,
                })
                .collect(),
        )
    }
    pub fn new_gradient(vertices: Vec<ColoredVertex>) -> Self {
        Self {
            transform: Mat3::identity(),
            vertices,
        }
    }
}

impl Draw2d for Polygon {
    fn draw_2d(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &geng.inner.draw_2d.color_program,
            ugli::DrawMode::TriangleFan,
            &ugli::VertexBuffer::new_dynamic(geng.ugli(), self.vertices.clone()),
            (
                ugli::uniforms! {
                    u_color: Color::WHITE,
                    u_framebuffer_size: framebuffer_size,
                    u_model_matrix: self.transform,
                },
                camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
    }
}

impl Transform2d for Polygon {
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}
