varying vec4 v_color;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec4 a_color;
uniform ivec2 u_framebuffer_size;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
void main() {
    v_color = a_color;
    gl_Position = u_projection_matrix * u_view_matrix * vec4(a_pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
void main() {
    gl_FragColor = v_color * u_color;
}
#endif