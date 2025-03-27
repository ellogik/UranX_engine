#version 330 core
layout (location = 0) in vec3 aPos;       // Позиция вершины
layout (location = 1) in vec3 aNormal;    // Нормаль
layout (location = 2) in vec2 aTexCoords; // UV-координаты

out vec2 TexCoords;  // Передаём UV в фрагментный шейдер
out vec3 FragPos;    // Позиция для освещения
out vec3 Normal;     // Нормаль для освещения

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    FragPos = vec3(model * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(model))) * aNormal;
    TexCoords = aTexCoords;

    gl_Position = projection * view * vec4(FragPos, 1.0);
}
