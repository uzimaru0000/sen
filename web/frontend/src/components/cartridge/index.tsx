import { TileMap } from "../TileMap";
import styles from "./index.module.scss";
import { useDraggable } from "@dnd-kit/core";

type Props = {
  rom: File;
  x: number;
  y: number;
};
export const Cartridge: React.FC<Props> = ({ rom, x, y }) => {
  const { isDragging, attributes, listeners, setNodeRef, transform } =
    useDraggable({
      id: rom.name,
      data: { type: "rom", file: rom },
    });

  const finalX = `calc(${x}% + ${transform?.x ?? 0})`;
  const finalY = `calc(${y}% + ${transform?.y ?? 0})`;
  const style = {
    top: `${finalY}`,
    left: `${finalX}`,
    cursor: isDragging ? "grabbing" : "grab",
  };

  return (
    <div
      ref={setNodeRef}
      className={styles.wrapper}
      style={style}
      {...listeners}
      {...attributes}
    >
      <TileMap
        className={styles.rom}
        width={1}
        height={1}
        tiles={[35]}
        rotates={[-1]}
      />
    </div>
  );
};
