# vertex
[[location(0)]] var<out> o_pos : vec4<f32>;

[[stage(vertex)]]
fn main() -> void {
  o_pos = vec4<f32>(1);
  return;
}