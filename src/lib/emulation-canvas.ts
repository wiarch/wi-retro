import type { VideoFramePayload } from "@/types/emulation";

export function frameToBytes(frame: VideoFramePayload): Uint8Array {
  if (!frame.dataB64) return new Uint8Array(0);
  const bin = atob(frame.dataB64);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

/** Blitea búfer RGBA8888 ya normalizado por el backend Rust. */
export function blitRgba8888(
  ctx: CanvasRenderingContext2D,
  frame: VideoFramePayload,
): void {
  const { width, height } = frame;
  const raw = frameToBytes(frame);
  const expected = width * height * 4;
  if (width <= 0 || height <= 0 || raw.length < expected) return;

  const imageData = ctx.createImageData(width, height);
  imageData.data.set(raw.subarray(0, expected));
  ctx.putImageData(imageData, 0, 0);
}

/** @deprecated Usar blitRgba8888 — el backend ya normaliza el formato. */
export const blitXrgb8888 = blitRgba8888;

export function isPlayableGbaRom(romPath: string): boolean {
  return romPath.toLowerCase().endsWith(".gba");
}
