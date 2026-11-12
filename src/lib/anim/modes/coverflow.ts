import type { AnimModeFn } from "../types";

const coverflow: AnimModeFn = (i, sel, _count, _t, s) => {
  const rel = i - sel;
  const gap = 180 * s.intensity + 80;
  const x = rel * gap;
  const rotateY = Math.max(-60, Math.min(60, rel * -45 * s.intensity));
  const scale = rel === 0 ? 1.15 : Math.max(0.8, 1 - Math.abs(rel) * 0.08);
  const opacity = Math.abs(rel) > 4 ? 0 : Math.max(0.4, 1 - Math.abs(rel) * 0.15);
  const z = -Math.abs(rel) * 40;
  const blur = Math.abs(rel) * (s.performanceMode ? 0 : 1);
  return { x, y: 0, z, scale, rotateX: 0, rotateY, rotateZ: 0, opacity, blur, zIndex: 100 - Math.abs(rel) };
};
export default coverflow;