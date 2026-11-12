import type { AnimModeFn } from "../types";

const minimalFade: AnimModeFn = (i, sel, _count, _t, _s) => {
  const isSelected = i === sel;
  return {
    x: 0, y: 0, z: 0,
    scale: isSelected ? 1.05 : 1,
    rotateX: 0, rotateY: 0, rotateZ: 0,
    opacity: isSelected ? 1 : 0.45,
    blur: 0,
    zIndex: isSelected ? 10 : 1,
  };
};
export default minimalFade;