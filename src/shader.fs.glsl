#version 460 core
in vec4 gl_FragCoord;
out vec4 final_color;

uniform vec2 iResolution;
uniform float iTime;
uniform vec4 iMouse; // mouse pixel coords. xy: current (if MLB down), zw: click

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

float map(vec3 p){
	vec3 spherePos = vec3(sin(iTime)*3.,0,0);
	float sphere = sdSphere(p-spherePos, 1.);


	float boxScale = 0.2;

	vec3 q = p;
	q.y -= iTime;
	q = fract(q) - .5;
	q.xy*=rot2d(iTime);
	q.yz*=rot2d(iTime);


	float box = sdBox(q/boxScale, vec3(.75))*boxScale;
	box = max(-sdBox(p, vec3(3)),box);

	float ground = p.y + .75;

	return min(ground , smin(box,sphere,1.)); //distance to sphere
	return length(p)-1;
	//return sqrt(p.x*p.x+p.y*p.y+p.z*p.z)-1;
}

void main() {
		
	vec2 uv = (gl_FragCoord.xy * 2. - iResolution.xy)/ iResolution.y;
	vec2 m = (iMouse.xy * 2. - iResolution.xy)/ iResolution.y;

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
		float d = map(p);//current distance to sceene
		//col = vec3(i)/80.;
		t+=d; //march the ray
		if(d < .001 || d > 100.) break;
		 
	    }
	// Output to screen
	col = vec3(t* .05);	
	final_color = vec4(col,1.0);
}
