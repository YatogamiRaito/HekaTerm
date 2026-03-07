// Clippy hates the implement_vertex macro and won't let me scope
// this warning to its use
#![allow(clippy::unneeded_field_pattern)]

use crate::renderstate::BorrowedLayers;
use ::window::bitmaps::TextureRect;
use ::window::color::LinearRgba;
use config::HsbTransform;

/// Each cell is composed of two triangles built from 4 vertices.
/// The buffer is organized row by row.
pub const VERTICES_PER_CELL: usize = 4;
pub const V_TOP_LEFT: usize = 0;
pub const V_TOP_RIGHT: usize = 1;
pub const V_BOT_LEFT: usize = 2;
pub const V_BOT_RIGHT: usize = 3;

/// a regular monochrome text glyph
const IS_GLYPH: f32 = 0.0;
/// a color emoji glyph
const IS_COLOR_EMOJI: f32 = 1.0;
/// a full color texture attached as the
/// background image of the window
const IS_BG_IMAGE: f32 = 2.0;
/// like 2.0, except that instead of an
/// image, we use the solid bg color
const IS_SOLID_COLOR: f32 = 3.0;
/// Grayscale poly quad for non-aa text render layers
const IS_GRAY_SCALE: f32 = 4.0;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    // Physical position of the corner of the character cell
    pub position: [f32; 2],
    // glyph texture
    pub tex: [f32; 2],
    pub fg_color: [f32; 4],
    pub alt_color: [f32; 4],
    pub hsv: [f32; 3],
    pub has_color: f32,
    pub mix_value: f32,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
    0 => Float32x2,
    1 => Float32x2,
    2 => Float32x4,
    3 => Float32x4,
    4 => Float32x3,
    5 => Float32,
    6 => Float32,
    ];

    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub trait QuadTrait {
    /// Assign the texture coordinates
    fn set_texture(&mut self, coords: TextureRect) {
        let x1 = coords.min_x();
        let x2 = coords.max_x();
        let y1 = coords.min_y();
        let y2 = coords.max_y();
        self.set_texture_discrete(x1, x2, y1, y2);
    }
    fn set_texture_discrete(&mut self, x1: f32, x2: f32, y1: f32, y2: f32);
    fn set_has_color_impl(&mut self, has_color: f32);

    /// Set the color glyph "flag"
    fn set_has_color(&mut self, has_color: bool) {
        self.set_has_color_impl(if has_color { IS_COLOR_EMOJI } else { IS_GLYPH });
    }

    /// Mark as a grayscale polyquad; color and alpha will be
    /// multipled with those in the texture
    fn set_grayscale(&mut self) {
        self.set_has_color_impl(IS_GRAY_SCALE);
    }

    /// Mark this quad as a background image.
    /// Mutually exclusive with `set_has_color`.
    fn set_is_background_image(&mut self) {
        self.set_has_color_impl(IS_BG_IMAGE);
    }

    fn set_is_background(&mut self) {
        self.set_has_color_impl(IS_SOLID_COLOR);
    }

    fn set_fg_color(&mut self, color: LinearRgba);

    /// Must be called after `set_fg_color`
    fn set_alt_color_and_mix_value(&mut self, color: LinearRgba, mix_value: f32);

    fn set_hsv(&mut self, hsv: Option<HsbTransform>);
    fn set_position(&mut self, left: f32, top: f32, right: f32, bottom: f32);
}

pub enum QuadImpl<'a> {
    Vert(Quad<'a>),
}

impl QuadTrait for QuadImpl<'_> {
    fn set_texture_discrete(&mut self, x1: f32, x2: f32, y1: f32, y2: f32) {
        match self {
            Self::Vert(q) => q.set_texture_discrete(x1, x2, y1, y2),
        }
    }

    fn set_has_color_impl(&mut self, has_color: f32) {
        match self {
            Self::Vert(q) => q.set_has_color_impl(has_color),
        }
    }

    fn set_fg_color(&mut self, color: LinearRgba) {
        match self {
            Self::Vert(q) => q.set_fg_color(color),
        }
    }

    fn set_alt_color_and_mix_value(&mut self, color: LinearRgba, mix_value: f32) {
        match self {
            Self::Vert(q) => q.set_alt_color_and_mix_value(color, mix_value),
        }
    }

    fn set_hsv(&mut self, hsv: Option<HsbTransform>) {
        match self {
            Self::Vert(q) => q.set_hsv(hsv),
        }
    }

    fn set_position(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        match self {
            Self::Vert(q) => q.set_position(left, top, right, bottom),
        }
    }
}

/// A helper for updating the 4 vertices that compose a glyph cell
pub struct Quad<'a> {
    pub(crate) vert: &'a mut [Vertex],
}

impl QuadTrait for Quad<'_> {
    fn set_texture_discrete(&mut self, x1: f32, x2: f32, y1: f32, y2: f32) {
        self.vert[V_TOP_LEFT].tex = [x1, y1];
        self.vert[V_TOP_RIGHT].tex = [x2, y1];
        self.vert[V_BOT_LEFT].tex = [x1, y2];
        self.vert[V_BOT_RIGHT].tex = [x2, y2];
    }

    fn set_has_color_impl(&mut self, has_color: f32) {
        for v in self.vert.iter_mut() {
            v.has_color = has_color;
        }
    }

    fn set_fg_color(&mut self, color: LinearRgba) {
        for v in self.vert.iter_mut() {
            v.fg_color = color.into();
        }
        self.set_alt_color_and_mix_value(color, 0.);
    }

    /// Must be called after `set_fg_color`
    fn set_alt_color_and_mix_value(&mut self, color: LinearRgba, mix_value: f32) {
        for v in self.vert.iter_mut() {
            v.alt_color = color.into();
            v.mix_value = mix_value;
        }
    }

    fn set_hsv(&mut self, hsv: Option<HsbTransform>) {
        let (h, s, v) = hsv.map_or((1., 1., 1.), |t| (t.hue, t.saturation, t.brightness));
        for vert in self.vert.iter_mut() {
            vert.hsv = [h, s, v];
        }
    }

    fn set_position(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.vert[V_TOP_LEFT].position = [left, top];
        self.vert[V_TOP_RIGHT].position = [right, top];
        self.vert[V_BOT_LEFT].position = [left, bottom];
        self.vert[V_BOT_RIGHT].position = [right, bottom];
    }
}

pub trait QuadAllocator {
    fn allocate(&mut self) -> anyhow::Result<QuadImpl<'_>>;
    fn extend_with(&mut self, vertices: &[Vertex]);
}

pub trait TripleLayerQuadAllocatorTrait {
    fn allocate(&mut self, layer_num: usize) -> anyhow::Result<QuadImpl<'_>>;
    fn extend_with(&mut self, layer_num: usize, vertices: &[Vertex]);
}

pub struct QuadBuffer {
    pub vertices: Vec<Vertex>,
    pub num_quads: usize,
    capacity: usize,
}

impl Default for QuadBuffer {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl QuadBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            vertices: vec![Vertex::default(); capacity * VERTICES_PER_CELL],
            num_quads: 0,
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clear(&mut self) {
        self.num_quads = 0;
    }

    pub fn grow(&mut self) {
        self.capacity = self.capacity.max(1) * 2;
        self.vertices
            .resize(self.capacity * VERTICES_PER_CELL, Vertex::default());
    }

    pub fn allocate(&mut self) -> anyhow::Result<QuadImpl<'_>> {
        if self.num_quads >= self.capacity {
            self.grow();
        }
        let start = self.num_quads * VERTICES_PER_CELL;
        self.num_quads += 1;

        let mut quad = Quad {
            vert: &mut self.vertices[start..start + VERTICES_PER_CELL],
        };
        quad.set_has_color(false);

        // Transmute lifetime for QuadImpl since it will be used ephemerally
        // within the RenderState's QuadAllocator
        let vert: &mut [Vertex] =
            unsafe { std::mem::transmute(&mut self.vertices[start..start + VERTICES_PER_CELL]) };
        let mut quad = Quad { vert };
        quad.set_has_color(false);

        Ok(QuadImpl::Vert(quad))
    }

    pub fn extend_with(&mut self, vertices: &[Vertex]) {
        let num_new_quads = vertices.len() / VERTICES_PER_CELL;
        while self.num_quads + num_new_quads > self.capacity {
            self.grow();
        }
        let start = self.num_quads * VERTICES_PER_CELL;
        self.vertices[start..start + vertices.len()].copy_from_slice(vertices);
        self.num_quads += num_new_quads;
    }
}


#[derive(Default)]
pub struct HeapQuadAllocator {
    layer0: Vec<Vertex>,
    layer1: Vec<Vertex>,
    layer2: Vec<Vertex>,
}

impl std::fmt::Debug for HeapQuadAllocator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("HeapQuadAllocator").finish()
    }
}

impl HeapQuadAllocator {
    pub fn apply_to(&self, other: &mut TripleLayerQuadAllocator) -> anyhow::Result<()> {
        let start = std::time::Instant::now();
        for (layer_num, vertices) in [(0, &self.layer0), (1, &self.layer1), (2, &self.layer2)] {
            other.extend_with(layer_num, vertices);
        }
        metrics::histogram!("quad_buffer_apply").record(start.elapsed());
        Ok(())
    }
}

impl TripleLayerQuadAllocatorTrait for HeapQuadAllocator {
    fn allocate(&mut self, layer_num: usize) -> anyhow::Result<QuadImpl<'_>> {
        let vertices = match layer_num {
            0 => &mut self.layer0,
            1 => &mut self.layer1,
            2 => &mut self.layer2,
            _ => unreachable!(),
        };

        let start = vertices.len();
        vertices.resize(start + VERTICES_PER_CELL, Vertex::default());

        let vert: &mut [Vertex] =
            unsafe { std::mem::transmute(&mut vertices[start..start + VERTICES_PER_CELL]) };
        let mut quad = Quad { vert };
        quad.set_has_color(false);

        Ok(QuadImpl::Vert(quad))
    }

    fn extend_with(&mut self, layer_num: usize, vertices: &[Vertex]) {
        if vertices.is_empty() {
            return;
        }

        let dest_vertices = match layer_num {
            0 => &mut self.layer0,
            1 => &mut self.layer1,
            2 => &mut self.layer2,
            _ => unreachable!(),
        };

        dest_vertices.extend_from_slice(vertices);
    }
}

pub enum TripleLayerQuadAllocator<'a> {
    Gpu(Box<BorrowedLayers>),
    Heap(&'a mut HeapQuadAllocator),
}

impl TripleLayerQuadAllocatorTrait for TripleLayerQuadAllocator<'_> {
    fn allocate(&mut self, layer_num: usize) -> anyhow::Result<QuadImpl<'_>> {
        match self {
            Self::Gpu(b) => b.allocate(layer_num),
            Self::Heap(h) => h.allocate(layer_num),
        }
    }

    fn extend_with(&mut self, layer_num: usize, vertices: &[Vertex]) {
        match self {
            Self::Gpu(b) => b.extend_with(layer_num, vertices),
            Self::Heap(h) => h.extend_with(layer_num, vertices),
        }
    }
}

#[cfg(test)]
#[test]
fn size() {
    assert_eq!(std::mem::size_of::<Vertex>() * VERTICES_PER_CELL, 272);
}
#[cfg(test)]
#[test]
fn test_quad_buffer_allocation() {
    let mut buf = QuadBuffer::new(2);
    assert_eq!(buf.capacity(), 2);

    let _ = buf.allocate().unwrap();
    assert_eq!(buf.num_quads, 1);

    buf.extend_with(&[Vertex::default(); VERTICES_PER_CELL * 2]);
    assert_eq!(buf.num_quads, 3);
    assert!(buf.capacity() >= 4);

    buf.clear();
    assert_eq!(buf.num_quads, 0);
    assert!(buf.capacity() >= 4);

    // allocate past capacity
    for _ in 0..10 {
        let _ = buf.allocate().unwrap();
    }
    assert_eq!(buf.num_quads, 10);
    assert!(buf.capacity() >= 10);
}
