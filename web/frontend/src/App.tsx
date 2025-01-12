import styles from "./App.module.scss";
import { useRef, useState } from "react";
import { DndContext, DragEndEvent } from "@dnd-kit/core";
import { TileMap } from "./components/TileMap";
import roomTiles from "./assets/room.json";
import objTiles from "./assets/obj.json";
import { TV } from "./components/tv";
import { NES } from "./components/nes";
import { DropArea } from "./components/dropArea";
import { Cartridge } from "./components/cartridge";
import { readFile } from "./lib/file";
import { Emulator } from "./lib/emulator";

type Rom = {
  isInsert: boolean;
  file: File;
  x: number;
  y: number;
};

export const App = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const [emulator, setEmulator] = useState<Emulator>();
  const [roms, setRoms] = useState<Rom[]>([]);

  const freeRoms = roms.filter((rom) => !rom.isInsert);

  const handleAddRom = (file: File) => {
    const pos = {
      x: Math.random() * 100,
      y: Math.random() * 100,
    };

    setRoms([
      ...roms,
      {
        isInsert: false,
        file,
        ...pos,
      },
    ]);
  };
  const handleDragEnd = (event: DragEndEvent) => {
    const { active, delta, over } = event;
    const { id } = active;

    // 移動先の座標を確定させる
    setRoms((prevItems) =>
      prevItems.map((item) => {
        if (item.file.name === id) {
          return {
            ...item,
            x: item.x + delta.x,
            y: item.y + delta.y,
          };
        }
        return item;
      })
    );

    if (!over) {
      return;
    }

    if (over.id === "nes") {
      setRoms((prevItems) =>
        prevItems.map((item) => {
          if (item.file.name === id) {
            return {
              ...item,
              isInsert: true,
              x: 0,
              y: 0,
            };
          }
          return item;
        })
      );

      if (active.data.current) {
        handleActiveEmulator(active.data.current.file);
      }
    }
  };
  const handleActiveEmulator = async (file: File) => {
    if (!canvasRef.current) {
      return;
    }

    const raw = await readFile(file);
    const context = new AudioContext();
    const emulator = new Emulator(raw, context, canvasRef.current);
    emulator.start();

    setEmulator(emulator);
  };

  return (
    <DndContext onDragEnd={handleDragEnd}>
      <div className={styles.app}>
        <div className={styles.tiles}>
          <TileMap
            width={8}
            height={8}
            tiles={roomTiles.tiles}
            rotates={roomTiles.rotates}
          />
          <TileMap
            width={8}
            height={8}
            tiles={objTiles.tiles}
            rotates={objTiles.rotates}
          />
        </div>
        <div className={styles.tv}>
          <TV ref={canvasRef} />
        </div>
        <div className={styles.nes}>
          <NES inCartridge={!!emulator} />
        </div>
        <div className={styles.roms}>
          {freeRoms.map((rom) => (
            <Cartridge key={rom.file.name} rom={rom.file} x={rom.x} y={rom.y} />
          ))}
        </div>
      </div>

      <DropArea onAddRom={handleAddRom} />
    </DndContext>
  );
};
