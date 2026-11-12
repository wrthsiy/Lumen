import type { AnimModeFn } from "../types";

const fan: AnimModeFn = (i, sel, count, _t, s) => {
  const center = (count - 1) / 2;
  const rel = i - sel;
  const angle = rel * (0.18 * s.intensity + 0.08);
  const radius = 300 * s.intensity + 150;
  const x = Math.sin(angle) * radius;
  const y = (1 - Math.cos(angle)) * radius * 0.6;
  const scale = rel === 0 ? 1.15 : Math.max(0.75, 1 - Math.abs(rel) * 0.05);
  const opacity = Math.abs(rel) > 6 ? 0 : Math.max(0.4, 1 - Math.abs(rel) * 0.1);
  const rotateZ = angle * 30 * s.intensity;
  const blur = Math.abs(rel) * (s.performanceMode ? 0 : 1);
  return { x, y, z: -Math.abs(rel) * 15, scale, rotateX: 0, rotateY: 0, rotateZ, opacity, blur, zIndex: 100 - Math.abs(rel) };
};
export default fan;