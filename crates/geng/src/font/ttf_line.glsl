varying float v_depth;

#ifdef VERTEX_SHADER
attribute vec3 a_pos;
uniform ivec2 u_framebuffer_size;
void main() {
    v_depth = a_pos.z;
    gl_Position = vec4(a_pos.xy / vec2(u_framebuffer_size) * 2.0 - 1.0, a_pos.z, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    float x = 1.0 - v_depth;
    gl_FragColor = vec4(x * 0.5);
}
#endif