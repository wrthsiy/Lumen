import swipe from "./modes/swipe";
import orbit from "./modes/orbit";
import coverflow from "./modes/coverflow";
import stack from "./modes/stack";
import wheel from "./modes/wheel";
import fan from "./modes/fan";
import spiral from "./modes/spiral";
import gridZoom from "./modes/grid-zoom";
import flip from "./modes/flip";
import minimalFade from "./modes/minimal-fade";
import type { AnimModeFn } from "./types";

export const modeRegistry: Record<string, AnimModeFn> = {
  swipe, orbit, coverflow, stack, wheel, fan, spiral, "grid-zoom": gridZoom, flip, "minimal-fade": minimalFade,
};

export function getMode(name: string): AnimModeFn {
  return modeRegistry[name] ?? modeRegistry.swipe;
}

export const modeNames = Object.keys(modeRegistry);