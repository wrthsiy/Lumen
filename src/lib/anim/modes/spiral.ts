import type { AnimModeFn } from "../types";

const spiral: AnimModeFn = (i, sel, count, _t, s) => {
  const rel = i - sel;
  const abs = Math.abs(rel);
  const angle = rel * (0.6 * s.intensity + 0.2);
  const radius = 80 + abs * 40 * s.intensity;
  const x = Math.sin(angle) * radius;
  const y = rel * 12;
  const z = -abs * 60 * s.intensity;
  const scale = Math.max(0.5, 1 - abs * 0.08);
  const opacity = Math.abs(rel) > 6 ? 0 : Math.max(0.25, 1 - abs * 0.12);
  const rotateZ = angle * 40 * s.intensity;
  const blur = abs * (s.performanceMode ? 0 : 2);
  return { x, y, z, scale, rotateX: 0, rotateY: 0, rotateZ, opacity, blur, zIndex: 100 - abs };
};
export default spiral;