# rust_shader_toy
a tool to execute glsl fragment shader like shader toy

## intention
- this tool was written to run this [raymarching shader](https://www.shadertoy.com/view/tXSGzW) localy

## shader inputs
```
in fragCoord

uniform vec2 iResolution; // resolution of the viewport
uniform float iTime; //time since the program has started
uniform vec4 iMouse; //[0:2] mouse pixel coords, TODO: [2:4] left & right click 

out vec4 final_color
```
