in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 projection;
uniform mat4 view;

void main() {
  v_normal = normal;
  gl_Position = projection * view * vec4(position, 1.);
}
