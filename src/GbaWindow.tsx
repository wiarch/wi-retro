import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { blitRgba8888 } from "@/lib/emulation-canvas";
import { LIBRETRO_FRAME_EVENT } from "@/lib/emulation-events";
import type { VideoFramePayload } from "@/types/emulation";
import "./GbaWindow.css";

/** Índices Libretro RETRO_DEVICE_ID_JOYPAD_* */
const KEY_TO_JOYPAD: Record<string, number> = {
  ArrowUp: 4,
  ArrowDown: 5,
  ArrowLeft: 6,
  ArrowRight: 7,
  KeyZ: 8,
  KeyX: 0,
  KeyA: 8,
  KeyS: 0,
  Enter: 3,
  ShiftRight: 2,
  ShiftLeft: 2,
};

export default function GbaWindow() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const heldKeys = useRef(new Set<string>());

  useEffect(() => {
    const win = getCurrentWebviewWindow();
    win.setFocus().catch(() => {});

    const setButton = (button: number, pressed: boolean) => {
      invoke("set_joypad_button", { button, pressed }).catch(() => {});
    };

    const onKeyDown = (e: KeyboardEvent) => {
      if (e.repeat || heldKeys.current.has(e.code)) return;
      const button = KEY_TO_JOYPAD[e.code];
      if (button === undefined) return;
      e.preventDefault();
      heldKeys.current.add(e.code);
      setButton(button, true);
    };

    const onKeyUp = (e: KeyboardEvent) => {
      const button = KEY_TO_JOYPAD[e.code];
      if (button === undefined) return;
      e.preventDefault();
      heldKeys.current.delete(e.code);
      setButton(button, false);
    };

    const onBlur = () => {
      heldKeys.current.forEach((code) => {
        const button = KEY_TO_JOYPAD[code];
        if (button !== undefined) setButton(button, false);
      });
      heldKeys.current.clear();
      invoke("clear_joypad").catch(() => {});
    };

    window.addEventListener("keydown", onKeyDown);
    window.addEventListener("keyup", onKeyUp);
    window.addEventListener("blur", onBlur);

    const unlistenFrame = listen<VideoFramePayload>(
      LIBRETRO_FRAME_EVENT,
      (event) => {
        const frame = event.payload;
        const canvas = canvasRef.current;
        if (!canvas || !frame?.dataB64) return;

        if (canvas.width !== frame.width || canvas.height !== frame.height) {
          canvas.width = frame.width;
          canvas.height = frame.height;
        }

        const ctx = canvas.getContext("2d");
        if (!ctx) return;
        blitRgba8888(ctx, frame);
      },
    );

    return () => {
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("keyup", onKeyUp);
      window.removeEventListener("blur", onBlur);
      invoke("clear_joypad").catch(() => {});
      unlistenFrame.then((fn) => fn());
    };
  }, []);

  return (
    <div className="gba-screen">
      <canvas ref={canvasRef} className="gba-screen__canvas" width={240} height={160} />
    </div>
  );
}
