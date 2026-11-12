import type { AnimModeFn } from "../types";

const flip: AnimModeFn = (i, sel, _count, _t, s) => {
  const rel = i - sel;
  const gap = 160 * s.intensity + 80;
  const x = rel * gap;
  const isSelected = i === sel;
  const rotateY = isSelected ? 0 : 180;
  const scale = isSelected ? 1.15 : 0.9;
  const opacity = Math.abs(rel) > 3 ? 0 : Math.max(0.3, 1 - Math.abs(rel) * 0.15);
  const blur = Math.abs(rel) * (s.performanceMode ? 0 : 1);
  return { x, y: 0, z: isSelected ? 10 : 0, scale, rotateX: 0, rotateY, rotateZ: 0, opacity, blur, zIndex: isSelected ? 100 : 10 };
};
export default flip;