in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 projection;
uniform mat4 view;
uniform float aspect_ratio;

void main() {
  v_normal = normal;
  gl_Position = projection * view * vec4(position, 1.);
  gl_Position.y *= aspect_ratio;
}
