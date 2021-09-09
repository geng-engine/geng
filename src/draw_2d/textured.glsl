varying vec4 v_color;
varying vec2 v_vt;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 a_vt;
attribute vec4 a_color;
uniform ivec2 u_framebuffer_size;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
void main() {
    v_color = a_color;
    v_vt = a_vt;
    gl_Position = u_projection_matrix * u_view_matrix * vec4(a_pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform sampler2D u_texture;
void main() {
    gl_FragColor = v_color * u_color * texture2D(u_texture, vec2(v_vt.x, 1.0 - v_vt.y));
}
#endif