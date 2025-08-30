use crate::gl;
use crate::gl::types::*;

use crate::renderer::rects::Vertex;

use std::{mem, ptr};

#[derive(Debug)]
pub struct FramebufferRenderer {
    fb:  GLuint,
    tex: GLuint,
    vao: GLuint,
    vbo: GLuint,

    vertices: Vec<Vertex>,
}

impl FramebufferRenderer {
    pub fn new() -> Self {
        let mut fb:  GLuint = 0;
        let mut tex: GLuint = 0;
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        let size: f32 = 1.0;
        let vertices = vec![
            Vertex{ x:-size, y: size, r: 0, g: 0, b: 0, a: 0 },
            Vertex{ x:-size, y:-size, r: 0, g: 0, b: 0, a: 0 },
            Vertex{ x: size, y: size, r: 0, g: 0, b: 0, a: 0 },
            Vertex{ x: size, y: size, r: 0, g: 0, b: 0, a: 0 },
            Vertex{ x: size, y:-size, r: 0, g: 0, b: 0, a: 0 },
            Vertex{ x:-size, y:-size, r: 0, g: 0, b: 0, a: 0 },
        ];

        unsafe {
            gl::GenFramebuffers(1, &mut fb);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb);

            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                128,
                128,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex, 0
            );

            assert!(
                gl::CheckFramebufferStatus(gl::FRAMEBUFFER)
                == gl::FRAMEBUFFER_COMPLETE
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let mut attribute_offset = 0;

            // Position.
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                attribute_offset as *const _,
            );
            gl::EnableVertexAttribArray(0);
            attribute_offset += mem::size_of::<f32>() * 2;

            // Color.
            gl::VertexAttribPointer(
                1,
                4,
                gl::UNSIGNED_BYTE,
                gl::TRUE,
                mem::size_of::<Vertex>() as i32,
                attribute_offset as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Self{ fb, tex, vao, vbo, vertices }
    } // <-- FramebufferRenderer::new()

    pub fn resize(&mut self, width: i32, height: i32) {
        let mut cur_tex: GLint = 0;
        unsafe {
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut cur_tex);

            gl::DeleteTextures(1, &mut self.tex);
            gl::GenTextures(1, &mut self.tex);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb);

            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.tex, 0
            );

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::BindTexture(gl::TEXTURE_2D, cur_tex as GLuint);
        }
    } // <-- FramebufferRenderer::resize(self, width, height)

    /// Draw the stored texture
    pub fn draw(&self, prog: GLuint) {
        let mut cur_tex: GLint = 0;
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut cur_tex);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::UseProgram(prog);

            gl::BindTexture(gl::TEXTURE_2D, self.tex);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::STREAM_DRAW,
            );

            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);

            gl::BindTexture(gl::TEXTURE_2D, cur_tex as GLuint);

            gl::UseProgram(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    } // <-- FramebufferRenderer::draw(self)

    pub fn enable(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fb);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    } // <-- FramebufferRenderer::enable(self)

    pub fn disable(&self) {
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0); }
    } // <-- FramebufferRenderer::disable(self)

    pub fn get_tex(&self) -> GLuint { self.tex }
}

impl Drop for FramebufferRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.tex);
            gl::DeleteFramebuffers(1, &mut self.fb);
            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    } // <-- Drop for FramebufferRenderer::drop(self)
}
