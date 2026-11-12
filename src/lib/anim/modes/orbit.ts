import type { AnimModeFn } from "../types";

const orbit: AnimModeFn = (i, sel, count, t, s) => {
  const base = (i / Math.max(1, count)) * Math.PI * 2;
  const selA = (sel / Math.max(1, count)) * Math.PI * 2;
  let angle = base - selA;
  const radiusX = 260 * s.intensity + 120;
  const radiusZ = 120 * s.intensity + 60 + Math.sin(t * 0.6) * 8 * s.intensity;
  const x = Math.sin(angle) * radiusX;
  const z = Math.cos(angle) * radiusZ - radiusZ;
  const front = (Math.cos(angle) + 1) / 2;
  const scale = 0.7 + 0.45 * front;
  const opacity = 0.35 + 0.65 * front;
  const y = Math.sin(angle * 2 + t * 0.4) * 6 * s.intensity;
  const blur = (1 - front) * (s.performanceMode ? 0 : 2);
  return { x, y, z, scale, rotateX: 0, rotateY: angle * 15 * s.intensity, rotateZ: 0, opacity, blur, zIndex: Math.round(front * 100) };
};
export default orbit;