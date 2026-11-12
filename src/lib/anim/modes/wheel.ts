import type { AnimModeFn } from "../types";

const wheel: AnimModeFn = (i, sel, count, _t, s) => {
  const rel = i - sel;
  const step = 0.35 * s.intensity + 0.15;
  const angle = rel * step;
  const radius = 420 * s.intensity + 200;
  const x = Math.sin(angle) * radius;
  const y = (1 - Math.cos(angle)) * (radius * 0.5);
  const scale = Math.max(0.6, 1 - Math.abs(rel) * 0.08);
  const opacity = Math.abs(rel) > 5 ? 0 : Math.max(0.3, 1 - Math.abs(rel) * 0.12);
  const rotateZ = angle * 20 * s.intensity;
  const blur = Math.abs(rel) * (s.performanceMode ? 0 : 1.5);
  const zIndex = 100 - Math.abs(rel);
  return { x, y: y - radius * 0.2, z: -Math.abs(rel) * 30, scale, rotateX: 0, rotateY: 0, rotateZ, opacity, blur, zIndex };
};
export default wheel;