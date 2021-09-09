varying vec2 v_vt;

#ifdef VERTEX_SHADER
uniform ivec2 u_framebuffer_size;
uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;

attribute vec2 a_vt;
attribute vec2 a_pos;
void main() {
    v_vt = a_vt;
    gl_Position = u_projection_matrix * u_view_matrix * vec4(a_pos, 0.0, 1.0);
    // gl_Position.y = -gl_Position.y;
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform sampler2D u_cache_texture;
void main() {
    gl_FragColor = u_color;
    gl_FragColor.w *= texture2D(u_cache_texture, v_vt).w;
}
#endif