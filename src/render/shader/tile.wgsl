// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @location(0) i: u32,
    @location(1) attributes: vec4<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

struct InstanceInput {
    @location(1) attributes: vec4<f32>,
};

struct CameraUniform {
    pos: vec2<f32>,
    proj: vec2<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TileViewUniform {
    pos: vec2<f32>,
    width: u32,
};
@group(0) @binding(1)
var<uniform> tile_view: TileViewUniform;

@vertex
fn vs_main(
    @builtin(instance_index) i: u32,
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.attributes = instance.attributes;
    out.i = i;

    var pos = vertex.position;
    pos -= camera.pos;
    pos.x += f32(i % tile_view.width);
    pos.y += f32(i / tile_view.width);
    pos += tile_view.pos;
    pos *= camera.proj;
    out.clip_position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    var hsv = rgb_to_hsv(in.attributes.xyz);
    let rgb = hsv_to_rgb(hsv);
    return vec4<f32>(rgb, 1.0);
}

fn rgb_to_hsv(color: vec3<f32>) -> vec3<f32> {
    let R = color.x;
    let G = color.y;
    let B = color.z;

    let M = max(R, max(G, B));
    let m = min(R, min(G, B));
    let V = M;
    let d = M - m;
    if d == 0.0 {
        return vec3(0.0, 0.0, V);
    }
    let S = d / M;
    var H = 1.0 / 6.0;
    if M == R {
        H *= 0.0 + (G - B) / d;
        if H < 0.0 {
            H += 1.0;
        }
    } else if M == G {
        H *= 2.0 + (B - R) / d;
    } else if M == B {
        H *= 4.0 + (R - G) / d;
    }
    return vec3(H, S, V);
}

fn hsv_to_rgb(color: vec3<f32>) -> vec3<f32> {
    var H = color.x;
    let S = color.y;
    let V = color.z;

    H *= 6.0;
    let I = floor(H);
    let F = H - I;
    let M = V * (1.0 - S);
    let N = V * (1.0 - S * F);
    let K = V * (1.0 - S * (1.0 - F));

    if I == 0.0 {
        return vec3(V, K, M);
    } else if I == 1.0 {
        return vec3(N, V, M);
    } else if I == 2.0 {
        return vec3(M, V, K);
    } else if I == 3.0 {
        return vec3(M, N, V);
    } else if I == 4.0 {
        return vec3(K, M, V);
    } else if I == 5.0 {
        return vec3(V, M, N);
    }

    return vec3(0.0, 0.0, 0.0);
}