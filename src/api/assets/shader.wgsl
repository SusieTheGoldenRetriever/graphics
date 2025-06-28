@group(0) @binding(0)
var<uniform> shape_color: vec4f;

@vertex
fn vs_main(@location(0) position: vec2f) -> @builtin(position) vec4f {
  return vec4f(position, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
  return shape_color;
}
