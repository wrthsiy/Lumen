import type { AnimModeFn } from "../types";

const gridZoom: AnimModeFn = (i, sel, count, _t, s) => {
  const cols = Math.ceil(Math.sqrt(count));
  const row = Math.floor(i / cols);
  const col = i % cols;
  const crow = Math.floor(sel / cols);
  const ccol = sel % cols;
  const dx = col - ccol;
  const dy = row - crow;
  const gap = 160 * s.intensity + 40;
  let x = (col - (cols - 1) / 2) * gap;
  let y = (row - Math.floor((count - 1) / cols) / 2) * gap;
  const dist = Math.hypot(dx, dy);
  const isSelected = i === sel;
  const scale = isSelected ? 1.25 : Math.max(0.85, 1 - dist * 0.04);
  const opacity = isSelected ? 1 : Math.max(0.4, 1 - dist * 0.12);
  if (!isSelected && dist <= 1) {
    x += dx * 12 * s.intensity;
    y += dy * 12 * s.intensity;
  }
  const blur = (isSelected ? 0 : dist * 0.8) * (s.performanceMode ? 0 : 1);
  return { x, y, z: isSelected ? 20 : 0, scale, rotateX: 0, rotateY: 0, rotateZ: 0, opacity, blur, zIndex: isSelected ? 100 : 10 };
};
export default gridZoom;