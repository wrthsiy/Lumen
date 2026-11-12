import { writable } from "svelte/store";

export type Action =
  | "up" | "down" | "left" | "right"
  | "confirm" | "back" | "menu" | "search" | "play"
  | "options" | "quickmenu";

export const lastAction = writable<Action | null>(null);
export const inputDevice = writable<"keyboard" | "gamepad">("keyboard");

/** Return `true` to consume the action so later handlers do not see it. */
type Handler = (a: Action) => boolean | void | Promise<void>;
const handlers = new Set<Handler>();

export function registerActionHandler(h: Handler) {
  handlers.add(h);
  return () => { handlers.delete(h); };
}

export function dispatchAction(a: Action) {
  lastAction.set(a);
  // Handlers run in registration order; an open overlay consumes its input.
  for (const h of [...handlers]) {
    if (h(a) === true) break;
  }
}

const DEADZONE = 0.5;
const REPEAT_DELAY = 400; // ms before a held direction starts repeating
const REPEAT_RATE = 110;  // ms between repeats

function isEditable(el: EventTarget | null): boolean {
  if (!(el instanceof HTMLElement)) return false;
  if (el.isContentEditable) return true;
  if (el instanceof HTMLTextAreaElement || el instanceof HTMLSelectElement) return true;
  if (el instanceof HTMLInputElement) {
    return !["checkbox", "radio", "range", "button", "submit", "color"].includes(el.type);
  }
  return false;
}

let initialized = false;

export function initInput() {
  if (initialized) return;
  initialized = true;

  window.addEventListener("keydown", (e) => {
    inputDevice.set("keyboard");

    // While typing in a field, only Escape leaves it; every other key (letters,
    // Space, Backspace, arrows, Tab) must reach the field untouched.
    if (isEditable(e.target)) {
      if (e.key === "Escape") {
        e.preventDefault();
        (e.target as HTMLElement).blur();
        dispatchAction("back");
      }
      return;
    }

    let action: Action | null = null;
    switch (e.key) {
      case "ArrowUp": action = "up"; break;
      case "ArrowDown": action = "down"; break;
      case "ArrowLeft": action = "left"; break;
      case "ArrowRight": action = "right"; break;
      case "Enter": case " ": action = "confirm"; break;
      case "Escape": case "Backspace": action = "back"; break;
      case "Tab": action = "options"; break;
      case "o": case "O": action = "options"; break;
      case "/": action = "search"; break;
      case "m": case "M": action = "quickmenu"; break;
    }
    if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "l") action = "quickmenu";
    if (action) {
      e.preventDefault();
      dispatchAction(action);
    }
  });

  let prevButtons: boolean[] = [];
  // When each held direction fires next (performance.now() ms), or absent.
  const held: Partial<Record<Action, number>> = {};

  function handleDirection(a: Action, pressed: boolean, now: number) {
    if (!pressed) { delete held[a]; return; }
    const next = held[a];
    if (next === undefined) {
      dispatchAction(a);
      held[a] = now + REPEAT_DELAY;
    } else if (now >= next) {
      dispatchAction(a);
      held[a] = now + REPEAT_RATE;
    }
  }

  function poll(now: number) {
    const pads = navigator.getGamepads ? navigator.getGamepads() : [];
    let pad: Gamepad | null = null;
    for (const p of pads) if (p && p.connected) { pad = p; break; }
    if (pad) {
      const buttons = pad.buttons.map((b) => b.pressed);
      const btnMap: [number, Action][] = [
        [0, "confirm"], [1, "back"], [2, "search"], [3, "play"],
        [8, "quickmenu"], [9, "options"],
      ];
      let any = false;
      for (const [i, a] of btnMap) {
        if (buttons[i] && !prevButtons[i]) { dispatchAction(a); any = true; }
      }
      const x = pad.axes[0] ?? 0;
      const y = pad.axes[1] ?? 0;
      const up = !!buttons[12] || y < -DEADZONE;
      const down = !!buttons[13] || y > DEADZONE;
      const left = !!buttons[14] || x < -DEADZONE;
      const right = !!buttons[15] || x > DEADZONE;
      handleDirection("up", up, now);
      handleDirection("down", down, now);
      handleDirection("left", left, now);
      handleDirection("right", right, now);
      if (any || up || down || left || right) inputDevice.set("gamepad");
      prevButtons = buttons;
    }
    requestAnimationFrame(poll);
  }
  requestAnimationFrame(poll);
}
