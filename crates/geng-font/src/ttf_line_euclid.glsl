varying vec2 v_dist_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 a_dist_pos;
uniform ivec2 u_framebuffer_size;
void main() {
    v_dist_pos = a_dist_pos;
    gl_Position = vec4(a_pos.xy / vec2(u_framebuffer_size) * 2.0 - 1.0, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    float x = max(1.0 - length(v_dist_pos), 0.0);
    gl_FragColor = vec4(x * 0.5);
}
#endif