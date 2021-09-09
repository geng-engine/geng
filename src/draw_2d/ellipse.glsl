varying vec2 v_quad_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_quad_pos;
uniform vec2 u_pos;
uniform vec2 u_radius;
uniform ivec2 u_framebuffer_size;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
void main() {
    v_quad_pos = a_quad_pos;
    gl_Position = u_projection_matrix * u_view_matrix * vec4(u_pos + u_radius * a_quad_pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
void main() {
    if (length(v_quad_pos) > 1.0) {
        discard;
    }
    gl_FragColor = u_color;
}
#endif