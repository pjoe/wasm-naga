// vertex
struct VertexOutput {
  [[builtin(position)]] pos : vec4<f32>;
};

[[stage(vertex)]]
fn main() -> VertexOutput {
  var out: VertexOutput;
  out.pos = vec4<f32>(1.0, 1.0, 1.0, 1.0);
  return out;
}