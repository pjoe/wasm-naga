#version 450 core

void main() {
    gl_Position = vec4(1);
    // missing return will currently error in spv-out
    // return;
}