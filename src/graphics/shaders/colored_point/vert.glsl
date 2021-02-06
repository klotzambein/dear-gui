in vec2 vertex;
in vec4 color;

out vec2 v_vertex;
out vec4 v_color;

void main() {
    v_vertex = vertex;
    v_color = color;
}