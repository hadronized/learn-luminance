// this was the vertex shader output; it’s now our (rasterized and interpolated) input!
in vec3 v_color;

// we will output a single color
out vec3 frag_color;

void main() {
  // KISS
  frag_color = v_color;
}
