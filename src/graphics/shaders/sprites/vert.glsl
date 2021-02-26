in vec2 vertex;
in vec2 size;
in int texture_index;

out vec2 v_vertex;
out vec2 v_size;
out int v_texture_index;

void main() {
    v_vertex = vertex;
    v_size = size;
    v_texture_index = texture_index;
}