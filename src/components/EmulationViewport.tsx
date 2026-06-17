import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { blitRgba8888 } from "@/lib/emulation-canvas";
import { LIBRETRO_FRAME_EVENT } from "@/lib/emulation-events";
import type { VideoFramePayload } from "@/types/emulation";
import "./EmulationViewport.css";

interface EmulationViewportProps {
  active: boolean;
  title: string;
  avInfo?: string;
  onStop: () => void;
  stopping?: boolean;
}

export default function EmulationViewport({
  active,
  title,
  avInfo,
  onStop,
  stopping = false,
}: EmulationViewportProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [frameCount, setFrameCount] = useState(0);
  const [canvasSize, setCanvasSize] = useState({ w: 240, h: 160 });

  useEffect(() => {
    if (!active) {
      setFrameCount(0);
      return;
    }

    let unlisten: UnlistenFn | undefined;

    listen<VideoFramePayload>(LIBRETRO_FRAME_EVENT, (event) => {
      const frame = event.payload;
      const canvas = canvasRef.current;
      if (!canvas || !frame?.dataB64) return;

      if (canvas.width !== frame.width || canvas.height !== frame.height) {
        canvas.width = frame.width;
        canvas.height = frame.height;
        setCanvasSize({ w: frame.width, h: frame.height });
      }

      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      blitRgba8888(ctx, frame);
      setFrameCount((n) => n + 1);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, [active]);

  if (!active) return null;

  return (
    <div className="emu-overlay" role="dialog" aria-label={`Emulando ${title}`}>
      <div className="emu-viewport">
        <header className="emu-viewport__header">
          <div>
            <h2 className="emu-viewport__title">{title}</h2>
            <p className="emu-viewport__meta">
              {avInfo ? `${avInfo} · ` : ""}
              Libretro en memoria
              {frameCount > 0
                ? ` · ${frameCount} frames`
                : " · esperando video…"}
            </p>
          </div>
          <button
            type="button"
            className="emu-viewport__stop"
            onClick={onStop}
            disabled={stopping}
          >
            {stopping ? "Deteniendo…" : "■ DETENER"}
          </button>
        </header>

        <div className="emu-viewport__screen">
          <canvas
            ref={canvasRef}
            className="emu-viewport__canvas"
            width={canvasSize.w}
            height={canvasSize.h}
          />
          {frameCount === 0 && (
            <div className="emu-viewport__waiting">
              <span className="emu-viewport__pulse" />
              <p>Core activo — el video llegará vía {LIBRETRO_FRAME_EVENT}</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
