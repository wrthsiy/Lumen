export interface Spring { value: number; velocity: number; }

export function stepSpring(s: Spring, target: number, stiffness: number, damping: number, dt: number): Spring {
  const force = (target - s.value) * stiffness;
  const damp = s.velocity * damping;
  const accel = force - damp;
  const v = s.velocity + accel * dt;
  const val = s.value + v * dt;
  return { value: val, velocity: v };
}

export function springParams(easing: string, reduced: boolean) {
  if (reduced) return { stiffness: 400, damping: 40 };
  if (easing === "linear") return { stiffness: 800, damping: 60 };
  if (easing === "ease-out") return { stiffness: 300, damping: 30 };
  return { stiffness: 180, damping: 22 }; // spring
}