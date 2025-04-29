#version 460 core
in vec4 gl_FragCoord;
out vec4 final_color;

uniform vec2 iResolution;
uniform float iTime;
uniform vec4 iMouse; // mouse pixel coords. xy: current (if MLB down), zw: click
vec4 union_objects(vec4 o1, vec4 o2){
       if(o1.w > o2.w){
           return o2;
       }
       return o1;
}

//smouth, but hard colors
vec4 s_union_objects(vec4 o1, vec4 o2,float k){
       float h = max(k-abs(o1.w-o2.w), 0.0)/k;
       if(o1.w > o2.w){
           return vec4(o2.xyz,o2.w-h*h*h*k*(1.0/6.0));
       }
       return vec4(o1.xyz,o1.w-h*h*h*k*(1.0/6.0));
}

//smouth with soft colors
vec4 sc_union_objects(vec4 o1, vec4 o2,float k){
   float h = max(k-abs(o1.w-o2.w), 0.0)/k;
   //float h1 = clamp(h, 0., 1.);
   float h1 = clamp(0.5 - 0.5*(o1.w-o2.w)/k, 0., 1.);
   vec3 c = mix(o2.xyz,o1.xyz,h1);
   if(o1.w > o2.w){
       return vec4(c,o2.w-h*h*h*k*(1.0/6.0));
   }
   return vec4(c,o1.w-h*h*h*k*(1.0/6.0));

}

float smin(float a, float b, float k){
    float h = max(k-abs(a-b), 0.0)/k;
    return min(a,b)-h*h*h*k*(1.0/6.0);
}

vec3 rot3D(vec3 p, vec3 axis, float angle){
    return mix(dot(axis, p)* axis, p, cos(angle))
        + cross(axis, p) * sin(angle);
}

mat2 rot2d(float angle){
    float s = sin(angle);
    float c = cos(angle);
    return mat2(c, -s, s, c);
}

float sdSphere(vec3 p, float s){
    return length(p) - s;
}

float sdBox(vec3 p, vec3 b){
    vec3 q = abs(p) - b;
    return length(
        max(q,0.0)) + 
        min(
            max(
                q.x,
                max(q.y,q.z)
            )
            ,0.0
        );
}

vec4 map(vec3 p){
    vec3 spherePos = vec3(sin(iTime)*3.,0,0);
    float sphere = sdSphere(p-spherePos, 1.);
    float obj = max(sphere,sdBox(p-spherePos,vec3(0.7)));
    
    float boxScale = 0.2;
   
    vec3 q = p;
    q.y -= iTime;
    q = fract(q) - .5;
    q.xy*=rot2d(iTime);
    q.yz*=rot2d(iTime);
    
    
    float box = sdBox(q/boxScale, vec3(.75))*boxScale;
    box = max(-sdBox(p, vec3(3)),box);
    
    float ground = p.y + .75;
    
    float box2 = sdBox(p-vec3(0.,-0.5,-1.), vec3(.5))*boxScale;
    
    
    return union_objects(
        sc_union_objects(
            vec4(1.0,0.0,0.0,box),
            vec4(0.1,0.1,0.9,obj),
            1.
        ),
        union_objects(
            sc_union_objects(
                vec4(0.,1.,0.,ground),
                vec4(1.,1.,1.,box2),
                0.2
            ),
            vec4(0.,0.,0.,1000.) //vec4(0.1,1.,1.,box2)
     ));
        
    //return min(ground , smin(box,obj,1.)); //distance to sphere
}

void main()
{
    vec2 uv = (gl_FragCoord.xy * 2. - iResolution.xy)/ iResolution.y;
    vec2 m = (iMouse.xy * 2. - iResolution.xy)/ iResolution.y;
    //init
    vec3 ro = vec3(0,0,-3);//ray origin point distance from screen
    vec3 rd = normalize(vec3(uv,1)); //pixel ray
    vec3 col = vec3(0);
    
    float t = 0.; // total distance travled
    
    
    //Horizontal vamera rotation
    ro.xz *= rot2d(-m.x);//comment for rotation from the camera
    rd.xz *= rot2d(-m.x);
    
    //Vertical vamera rotation
    //ro.yz *= rot2d(-m.y);//comment for rotation from the camera
    //rd.yz *= rot2d(-m.y);
    
    //Raymarching
    for(int i=0;i<80;i++){
         vec3 p = ro + rd * t; //position onlong the ray
         
         vec4 d = map(p);//current distance to sceene
         col = d.xyz;
         t+=d.w; //march the ray
         if(d.w < .001 || d.w > 100.) break;
         
    }
    // Output to screen
    col = col/t*5.;
    final_color = vec4(col,1.0);
}
