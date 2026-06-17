import { useCallback, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { homeDir, join } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { isPlayableGbaRom } from "@/lib/emulation-canvas";
import { LIBRETRO_STATUS_EVENT } from "@/lib/emulation-events";
import { getGames, launchGame, scanGbaRoms, stopGame } from "@/lib/tauri";
import type { EmulationStatusPayload } from "@/types/emulation";
import type { Game } from "@/types/game";
import "./styles/app.css";

const LIBRARY_PATH_KEY = "wi-retro-library-path";

function initials(title: string): string {
  const words = title.trim().split(/\s+/).filter(Boolean);
  if (words.length === 0) return "?";
  if (words.length === 1) return words[0].slice(0, 2).toUpperCase();
  return (words[0][0] + words[1][0]).toUpperCase();
}

export default function App() {
  const [games, setGames] = useState<Game[]>([]);
  const [selected, setSelected] = useState<Game | null>(null);
  const [libraryPath, setLibraryPath] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [launching, setLaunching] = useState(false);
  const [stopping, setStopping] = useState(false);
  const [isEmulating, setIsEmulating] = useState(false);
  const [status, setStatus] = useState("Inicializando biblioteca…");
  const [isError, setIsError] = useState(false);

  const scanFolder = useCallback(async (folderPath: string) => {
    setLoading(true);
    setIsError(false);
    setStatus("Escaneando ROMs GBA…");
    try {
      const result = await scanGbaRoms(folderPath);
      setGames(result);
      setSelected((prev) => {
        if (prev) {
          return result.find((g) => g.id === prev.id) ?? result[0] ?? null;
        }
        return result[0] ?? null;
      });
      if (result.length === 0) {
        setIsError(true);
        setStatus(`0 juegos en "${folderPath}" — verifica que haya .gba o .zip`);
      } else {
        setStatus(`${result.length} juegos indexados`);
      }
    } catch (err: unknown) {
      setIsError(true);
      setStatus(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const loadLibrary = useCallback(async () => {
    setLoading(true);
    const stored = localStorage.getItem(LIBRARY_PATH_KEY);

    if (stored) {
      setLibraryPath(stored);
      await scanFolder(stored);
      return;
    }

    try {
      const cached = await getGames();
      setGames(cached);
      setSelected(cached[0] ?? null);
      setStatus(
        cached.length > 0
          ? `${cached.length} juegos en caché`
          : "Selecciona una carpeta de ROMs",
      );
    } catch (err: unknown) {
      setStatus(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [scanFolder]);

  useEffect(() => {
    loadLibrary();
  }, [loadLibrary]);

  useEffect(() => {
    const unlisten = listen<EmulationStatusPayload>(
      LIBRETRO_STATUS_EVENT,
      (event) => {
        const payload = event.payload;
        if (payload.state === "stopped") {
          setIsEmulating(false);
        }
        if (payload.state === "started") {
          setIsEmulating(true);
        }
        if (payload.message) {
          setStatus(payload.message);
          setIsError(payload.state === "error");
        }
      },
    );

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const handlePickFolder = async () => {
    let defaultPath: string | undefined;
    try {
      const home = await homeDir();
      defaultPath = await join(home, "Downloads");
    } catch {
      defaultPath = undefined;
    }

    const picked = await open({
      directory: true,
      multiple: false,
      title: "Carpeta de ROMs GBA",
      defaultPath,
    });
    if (!picked || Array.isArray(picked)) return;

    localStorage.setItem(LIBRARY_PATH_KEY, picked);
    setLibraryPath(picked);
    await scanFolder(picked);
  };

  const handleLaunch = async (game: Game) => {
    if (!isPlayableGbaRom(game.rom_path)) {
      setIsError(true);
      setStatus("Solo archivos .gba — Libretro no abre .zip directamente");
      return;
    }

    setLaunching(true);
    setIsError(false);
    setStatus(`Abriendo ventana GBA — ${game.title}…`);

    try {
      const result = await launchGame(game.rom_path);
      setIsEmulating(true);
      setStatus(result.message);
    } catch (err: unknown) {
      setIsError(true);
      setIsEmulating(false);
      setStatus(err instanceof Error ? err.message : String(err));
    } finally {
      setLaunching(false);
    }
  };

  const handleStop = async () => {
    setStopping(true);
    setIsError(false);
    setStatus("Deteniendo emulación…");
    try {
      await stopGame();
      setIsEmulating(false);
      setStatus("Emulación detenida");
    } catch (err: unknown) {
      setIsError(true);
      setStatus(err instanceof Error ? err.message : String(err));
    } finally {
      setStopping(false);
    }
  };

  const handleCardClick = (game: Game) => {
    setSelected(game);
  };

  const handleCardDoubleClick = (game: Game) => {
    setSelected(game);
    if (!isEmulating) handleLaunch(game);
  };

  const selectedPlayable =
    selected !== null && isPlayableGbaRom(selected.rom_path);

  return (
    <div className="shell">
      <aside className="platforms">
        <header className="platforms__header">
          <h1 className="platforms__logo">wi-retro</h1>
          <p className="platforms__tagline">LaunchBox · Libretro FFI</p>
        </header>

        <nav className="platforms__list">
          <button type="button" className="platform platform--active">
            <span className="platform__icon">GBA</span>
            <span className="platform__label">Game Boy Advance</span>
            <span className="platform__count">{games.length}</span>
          </button>
        </nav>

        <footer className="platforms__footer">
          <button
            type="button"
            className="platforms__action"
            onClick={handlePickFolder}
          >
            Cambiar carpeta
          </button>
        </footer>
      </aside>

      <main className="library">
        <header className="library__header">
          <div>
            <h2 className="library__title">Biblioteca</h2>
            <p className="library__subtitle">
              {libraryPath ?? "Sin carpeta configurada"}
            </p>
          </div>
          <button
            type="button"
            className="library__rescan"
            onClick={() => libraryPath && scanFolder(libraryPath)}
            disabled={!libraryPath || loading}
          >
            {loading ? "Escaneando…" : "Reindexar"}
          </button>
        </header>

        <div className="library__grid">
          {games.length === 0 && !loading && (
            <div className="library__empty">
              <p>No hay juegos indexados</p>
              <button type="button" onClick={handlePickFolder}>
                Elegir carpeta de ROMs
              </button>
            </div>
          )}

          {games.map((game) => {
            const zip = !isPlayableGbaRom(game.rom_path);
            return (
              <article
                key={game.id}
                className={`game-card${selected?.id === game.id ? " game-card--selected" : ""}${zip ? " game-card--zip" : ""}`}
                onClick={() => handleCardClick(game)}
                onDoubleClick={() => handleCardDoubleClick(game)}
              >
                <div className="game-card__cover">
                  {game.box_art ? (
                    <img src={game.box_art} alt={game.title} />
                  ) : (
                    <div className="game-card__placeholder">
                      <span>{initials(game.title)}</span>
                    </div>
                  )}
                  <div className="game-card__shine" />
                </div>
                <h3 className="game-card__title">{game.title}</h3>
              </article>
            );
          })}
        </div>

        <footer
          className={`library__status${isError ? " library__status--error" : ""}`}
        >
          {status}
        </footer>
      </main>

      <aside className="detail">
        {selected ? (
          <>
            <div className="detail__hero">
              <div className="detail__cover">
                {selected.box_art ? (
                  <img src={selected.box_art} alt={selected.title} />
                ) : (
                  <div className="detail__placeholder">
                    <span>{initials(selected.title)}</span>
                  </div>
                )}
              </div>
              <h2 className="detail__title">{selected.title}</h2>
            </div>

            <dl className="detail__meta">
              <div className="detail__row">
                <dt>Desarrollador</dt>
                <dd>{selected.developer || "Desconocido"}</dd>
              </div>
              <div className="detail__row">
                <dt>Año</dt>
                <dd>{selected.year || "—"}</dd>
              </div>
              <div className="detail__row detail__row--path">
                <dt>Ruta ROM</dt>
                <dd>{selected.rom_path}</dd>
              </div>
              <div className="detail__row">
                <dt>Motor</dt>
                <dd>Libretro mGBA · ventana GBA</dd>
              </div>
              {isEmulating && (
                <div className="detail__row">
                  <dt>Salida</dt>
                  <dd>Ventana nativa «GBA» (720×480)</dd>
                </div>
              )}
            </dl>

            {isEmulating ? (
              <button
                type="button"
                className="detail__play detail__play--active"
                disabled={stopping}
                onClick={handleStop}
              >
                {stopping ? "Deteniendo…" : "■ DETENER"}
              </button>
            ) : (
              <button
                type="button"
                className="detail__play"
                disabled={launching || !selectedPlayable}
                onClick={() => handleLaunch(selected)}
                title={
                  selectedPlayable
                    ? "Inicializar core Libretro"
                    : "Solo archivos .gba"
                }
              >
                {launching ? "Abriendo ventana GBA…" : "🎮 JUGAR"}
              </button>
            )}
          </>
        ) : (
          <div className="detail__empty">
            <p>Selecciona un juego de la rejilla</p>
          </div>
        )}
      </aside>
    </div>
  );
}
