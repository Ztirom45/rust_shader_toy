# rust_shader_toy
a tool to execute glsl localy fragment shader like shader toy

## intention
- this tool was written to run this [raymarching shader](https://www.shadertoy.com/view/tXSGzW) localy

## stack
- this tool is based on berrilium (sdl bindings for rust) and ogl33
- make sure to install sdl and ogl on your system

## shader inputs
```
in fragCoord //pixel position

uniform vec2 iResolution; // resolution of the viewport
uniform float iTime; //time since the program has started
uniform vec4 iMouse; //[0:2] mouse pixel coords, TODO: [2:4] left & right click 

out vec4 final_color //pixel output color
```
