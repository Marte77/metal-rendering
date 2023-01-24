

//let light_count = 16;
//let twopi = 6.28318530718;
@fragment //usamos da mesma maneira que o vertex, indicar que este e o ponto de entrada dos fragment shaders
fn fs_main(in: VertexOutput) -> @location(0) //indica ao wgpu que o vec4 retornado tem de ser guardado no primeiro color target
                                    vec4<f32> {
    /*var uv = vec2<f32>(in.clip_position.x,in.clip_position.y) - vec2<f32>(800.0,600.0) / 2.0;
    var color = vec3<f32>(0.0,0.1,0.2);

    for(var i:f32 = 0.0; i < light_count; i = i + 1.0){
        var lightAngle = twopi * i / light_count;
        var x = cos(lightAngle);
        var y = sin(lightAngle);
        var lightPos = vec2<f32>(x,y) * 5;
        var hue = (i + u_time * 5.0) / u_lightCount;
        var lightColour = hsl2rgb(vec3(hue, 1.0, 0.5));
        color = color + brightness / abs(length(uv - lightPos)) * lightColour;
    }

    return vec4(color,1.0);*/
    return vec4<f32>(0.9,0.0,0.3,1.0);
}