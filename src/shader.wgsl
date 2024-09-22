@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),

        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, -1.0)
    );

    let pos = positions[vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct Uniforms {
    pos: vec2<f32>,
    zoom: f32,
    resolution: vec2<f32>,
    offset: vec2<f32>,
    max_iter: i32,
    exponent: f32,
    fractal_type: u32,
    shading_type: u32,
    color_scheme: ColorScheme,
    palatte_speed: f32,
}

struct ColorScheme {
    a: vec3<f32>,
    b: vec3<f32>,
    c: vec3<f32>,
    d: vec3<f32>,
}

fn pal(t: f32, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    return a + b * cos(6.28318 * (c * t + d));
}

fn mul_complex(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn square_complex(a: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x * a.x - a.y * a.y, 2.0 * a.x * a.y);
}

fn powf_complex(a: vec2<f32>, n: f32) -> vec2<f32> {
    var abs_n = abs(n);
    var z: vec2<f32>;
    if abs_n == 2.0 {
        z = square_complex(a);
    } else {
        var r = sqrt(a.x * a.x + a.y * a.y);
        var theta = atan2(a.y, a.x);
        var ntheta = abs_n * theta;
        z = pow(r, abs_n) * vec2<f32>(cos(ntheta), sin(ntheta));
    }
    if n > 0.0 {
        return z;
    }
    return conjugate_complex(z) / (z.x * z.x + z.y * z.y);
}

fn exp_complex(z: vec2<f32>) -> vec2<f32> {
    return exp(z.x) * vec2<f32>(cos(z.y), sin(z.y));
}

fn mandellike(c: vec2<f32>, fractal_type: u32) -> f32 {
    var z = vec2<f32>(0.0, 0.0);

    let escape = 4.0;
    let escape_sq = escape * escape;
    let log_escape = log(escape);
    let log_exponent = log(uniforms.exponent);

    for (var i: i32 = 0; i < uniforms.max_iter; i = i + 1) {
        var zn_sq = z.x * z.x + z.y * z.y;
        if zn_sq >= escape_sq {
            var out: f32;
            switch uniforms.shading_type {
                case u32(0) {
                    return max(f32(i) - 2.0, 0.0);
                }
                case u32(1) {
                    return max(f32(i) - 2.0 - saturate(log(log(zn_sq) / (2.0 * log_escape)) / log_exponent), 0.0);
                }
                case default {
                    return 0.0;
                }
            }
            
        }

        z = mandellike_iter(z, c, fractal_type);
    }
    return -1.0;
}

fn mandellike_iter(z: vec2<f32>, c: vec2<f32>, fractal_type: u32) -> vec2<f32> {
    switch fractal_type {
        case u32(0) {
            return mandelbrot(z, c);
        }
        case u32(1) {
            return burning_ship(z, c);
        }
        case u32(2) {
            return tricorn(z, c);
        }
        case default {
            return z;
        }
    }
}

fn conjugate_complex(a: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x, -a.y);
}

fn mandelbrot(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    return powf_complex(z, uniforms.exponent) + c;
}

fn burning_ship(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    return powf_complex(abs(z), uniforms.exponent) + c;
}

fn tricorn(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    return powf_complex(conjugate_complex(z), uniforms.exponent) + c;
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = uniforms.resolution;
    let normalized = (frag_coord.xy - resolution * 0.5 - uniforms.offset) / min(resolution.x, resolution.y) * 2.0;
    let scaled = normalized * uniforms.zoom - uniforms.pos;

    let res = mandellike(scaled, uniforms.fractal_type);

    if (res < 0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    if (res != res) {
        return vec4<f32>(1.0, 0.0, 1.0, 1.0);
    }

    let color = pal(
        res * uniforms.palatte_speed,
        uniforms.color_scheme.a,
        uniforms.color_scheme.b,
        uniforms.color_scheme.c,
        uniforms.color_scheme.d,
    );

    return vec4<f32>(color, 1.0);
}
