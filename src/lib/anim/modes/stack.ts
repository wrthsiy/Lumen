import type { AnimModeFn } from "../types";

const stack: AnimModeFn = (i, sel, count, _t, s) => {
  const rel = i - sel;
  const abs = Math.abs(rel);
  if (rel === 0) {
    return { x: 0, y: 0, z: 100, scale: 1.15, rotateX: 0, rotateY: 0, rotateZ: 0, opacity: 1, blur: 0, zIndex: 100 };
  }
  const sign = rel > 0 ? 1 : -1;
  const x = sign * (12 * abs * s.intensity);
  const y = abs * 8 * s.intensity;
  const scale = Math.max(0.7, 1 - abs * 0.06);
  const rotateZ = sign * abs * 3 * s.intensity;
  const opacity = Math.max(0.3, 1 - abs * 0.12);
  const z = -abs * 20;
  const blur = abs * (s.performanceMode ? 0 : 1);
  return { x, y, z, scale, rotateX: 0, rotateY: 0, rotateZ, opacity, blur, zIndex: 100 - abs };
};
export default stack;