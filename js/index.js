const package = import("../pkg/index.js").catch(console.error);

const total_points = 200;
const times = 2;

package.then((rust) => {
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  ctx.lineWidth = 0.5;

  rust.cardioid_times_table(total_points, times, ctx, canvas.height / 2);
});
