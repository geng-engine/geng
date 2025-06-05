use super::*;

mod parameters;

pub use parameters::*;

#[derive(Debug, Copy, Clone)]
pub enum DrawMode {
    Points,
    Lines { line_width: f32 },
    LineStrip { line_width: f32 },
    LineLoop { line_width: f32 },
    Triangles,
    TriangleStrip,
    TriangleFan,
}

pub fn clear(
    framebuffer: &mut Framebuffer,
    color: Option<Rgba<f32>>,
    depth: Option<f32>,
    stencil: Option<StencilValue>,
) {
    let gl = &framebuffer.fbo.ugli.inner.raw;
    framebuffer.fbo.bind();
    let mut flags = 0;
    if let Some(color) = color {
        flags |= raw::COLOR_BUFFER_BIT;
        gl.clear_color(color.r as _, color.g as _, color.b as _, color.a as _);
        gl.color_mask(raw::TRUE, raw::TRUE, raw::TRUE, raw::TRUE);
    }
    if let Some(depth) = depth {
        flags |= raw::DEPTH_BUFFER_BIT;
        gl.clear_depth(depth as _);
        gl.depth_mask(raw::TRUE);
    }
    if let Some(stencil) = stencil {
        flags |= raw::STENCIL_BUFFER_BIT;
        gl.clear_stencil(stencil as _);
    }
    gl.clear(flags);
    framebuffer.fbo.ugli.debug_check();
}

fn apply_uniforms<U: Uniforms>(uniforms: U, program: &Program) {
    puffin::profile_function!();
    use std::any::{Any, TypeId};
    use std::cell::RefCell;
    thread_local! {
        static INFOS: RefCell<HashMap<(u64, TypeId), Box<dyn Any>>> = RefCell::new(HashMap::new());
    }
    INFOS.with(|infos| {
        let mut infos = infos.borrow_mut();
        let info = infos
            .entry((program.cache_key, TypeId::of::<U::ProgramInfoCacheKey>()))
            .or_insert_with(|| Box::new(U::get_program_info(program)));
        uniforms.apply_uniforms(program, info.downcast_ref().unwrap());
    })
}

pub fn draw<V, U, DP>(
    framebuffer: &mut Framebuffer,
    program: &Program,
    mode: DrawMode,
    vertices: V,
    uniforms: U,
    draw_parameters: DP,
) where
    V: VertexDataSource,
    U: Uniforms,
    DP: std::borrow::Borrow<DrawParameters>,
{
    // puffin::profile_function!();
    // program.ugli.debug_check();
    let gl = &program.ugli.inner.raw;
    //
    // framebuffer.fbo.bind();
    // let draw_parameters: &DrawParameters = draw_parameters.borrow();
    // draw_parameters.apply(gl, framebuffer.size());
    program.bind();
    // unsafe {
    //     UNIFORM_TEXTURE_COUNT = 0;
    // }
    // if draw_parameters.reset_uniforms {
    //     puffin::profile_scope!("reset uniforms");
    //     for uniform in program.uniforms.values() {
    //         if let Some(default) = &uniform.default {
    //             default.apply(program, uniform);
    //         }
    //     }
    // }
    //
    // apply_uniforms(uniforms, program);
    return;

    let mut vertex_count = None;
    let mut instance_count = None;
    let mut attribute_locations = Vec::new();
    {
        puffin::profile_scope!("walk vertex data");
        vertices.walk_data(Vdc {
            program,
            attribute_locations: &mut attribute_locations,
            vertex_count: &mut vertex_count,
            instance_count: &mut instance_count,
        });
    }
    let vertex_count = vertex_count.unwrap();
    if vertex_count == 0 {
        return;
    }
    let gl_mode = match mode {
        DrawMode::Points => raw::POINTS,
        DrawMode::Lines { line_width } => {
            gl.line_width(line_width as _);
            assert!(vertex_count % 2 == 0);
            raw::LINES
        }
        DrawMode::LineStrip { line_width } => {
            gl.line_width(line_width as _);
            assert!(vertex_count >= 2);
            raw::LINE_STRIP
        }
        DrawMode::LineLoop { line_width } => {
            gl.line_width(line_width as _);
            assert!(vertex_count >= 3);
            raw::LINE_LOOP
        }
        DrawMode::Triangles => {
            assert!(vertex_count % 3 == 0);
            raw::TRIANGLES
        }
        DrawMode::TriangleStrip => {
            assert!(vertex_count >= 3);
            raw::TRIANGLE_STRIP
        }
        DrawMode::TriangleFan => {
            assert!(vertex_count >= 3);
            raw::TRIANGLE_FAN
        }
    };

    if vertex_count != 0 {
        puffin::profile_scope!("draw call");
        if let Some(instance_count) = instance_count {
            if instance_count != 0 {
                gl.draw_arrays_instanced(gl_mode, 0, vertex_count as _, instance_count as _);
            }
        } else {
            gl.draw_arrays(gl_mode, 0, vertex_count as _);
        }
    }

    {
        puffin::profile_scope!("disable");
        for location in attribute_locations {
            gl.disable_vertex_attrib_array(location);
        }
    }

    program.ugli.debug_check();

    struct Vdc<'a> {
        program: &'a Program,
        attribute_locations: &'a mut Vec<raw::UInt>,
        vertex_count: &'a mut Option<usize>,
        instance_count: &'a mut Option<usize>,
    }
    impl<'a> VertexDataVisitor for Vdc<'a> {
        fn visit<'b, D: Vertex + 'b, T: IntoVertexBufferSlice<'b, D>>(
            &mut self,
            data: T,
            divisor: Option<usize>,
        ) {
            let data = data.into_slice();
            if let Some(divisor) = divisor {
                let instance_count = data.len() * divisor;
                if let Some(current_instance_count) = *self.instance_count {
                    assert_eq!(current_instance_count, instance_count);
                } else {
                    *self.instance_count = Some(instance_count);
                }
            } else if let Some(current_vertex_count) = *self.vertex_count {
                assert_eq!(current_vertex_count, data.len());
            } else {
                *self.vertex_count = Some(data.len());
            }
            data.buffer.bind();
            D::walk_attributes(Vac::<D> {
                attribute_locations: self.attribute_locations,
                divisor,
                program: self.program,
                offset: data.range.start * mem::size_of::<D>(),
                phantom_data: PhantomData,
            });
            struct Vac<'a, D: Vertex + 'a> {
                attribute_locations: &'a mut Vec<raw::UInt>,
                offset: usize,
                divisor: Option<usize>,
                program: &'a Program,
                phantom_data: PhantomData<D>,
            }
            impl<'a, D: Vertex> VertexAttributeVisitor for Vac<'a, D> {
                fn visit<A: VertexAttribute>(&mut self, name: &str, offset: usize) {
                    let gl = &self.program.ugli.inner.raw;
                    if let Some(attribute_info) = self.program.attributes.get(name) {
                        let offset = self.offset + offset + A::primitive_offset();
                        for row in 0..A::Primitive::ROWS {
                            let offset = offset + mem::size_of::<A>() * row / A::Primitive::ROWS;
                            let location = attribute_info.location + row as raw::UInt;
                            self.attribute_locations.push(location);
                            gl.enable_vertex_attrib_array(location);
                            gl.vertex_attrib_pointer(
                                location,
                                A::Primitive::SIZE as raw::Int,
                                A::Primitive::TYPE as raw::Enum,
                                raw::FALSE,
                                mem::size_of::<D>() as raw::SizeI,
                                offset as raw::IntPtr,
                            );
                            if let Some(divisor) = self.divisor {
                                gl.vertex_attrib_divisor(location, divisor as raw::UInt);
                            } else {
                                gl.vertex_attrib_divisor(location, 0);
                            }
                        }
                    }
                }
            }
        }
    }
}
