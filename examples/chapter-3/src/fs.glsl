in vec3 v_normal;

out vec3 frag_color;

void main() {
  vec3 obj_color = vec3(.6, .6, .6);
  vec3 light_dir = vec3(0., -1., -.5);
  float kd = dot(v_normal, -light_dir);

  frag_color = obj_color * kd;
}
