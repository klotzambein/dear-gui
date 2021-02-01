in vec2 start;
in vec2 end;
in vec4 color;

out vec2 v_start;
out vec2 v_end;
out vec4 v_color;

void main() {
    v_start = start;
    v_end = end;
    v_color = color;
}