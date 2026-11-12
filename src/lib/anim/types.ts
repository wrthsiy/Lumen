export interface TileTransform {
  x: number; y: number; z: number;
  scale: number; rotateX: number; rotateY: number; rotateZ: number;
  opacity: number; blur: number; zIndex: number;
}

export interface AnimSettings {
  mode: string; speed: number; intensity: number; easing: string;
  parallax: boolean; tilt: boolean; glow: boolean; reflection: boolean;
  reducedMotion: boolean; performanceMode: boolean;
}

export type AnimModeFn = (index: number, selectedIndex: number, count: number, time: number, settings: AnimSettings) => TileTransform;

export const defaultTransform = (): TileTransform => ({
  x: 0, y: 0, z: 0, scale: 1, rotateX: 0, rotateY: 0, rotateZ: 0, opacity: 1, blur: 0, zIndex: 1,
});