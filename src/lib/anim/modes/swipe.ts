import type { AnimModeFn, TileTransform } from "../types";

const swipe: AnimModeFn = (i, sel, count, _t, s) => {
  const rel = i - sel;
  const gap = 150 * s.intensity + 100;
  const scale = rel === 0 ? 1.15 : Math.max(0.85, 1 - Math.abs(rel) * 0.06);
  const opacity = Math.abs(rel) > 4 ? 0 : Math.max(0.3, 1 - Math.abs(rel) * 0.18);
  const blur = Math.abs(rel) * (s.performanceMode ? 0 : 1.5);
  const x = rel * gap;
  const t: TileTransform = { x, y: 0, z: rel === 0 ? 10 : 0, scale, rotateX: 0, rotateY: 0, rotateZ: 0, opacity, blur, zIndex: 100 - Math.abs(rel) };
  return t;
};
export default swipe;