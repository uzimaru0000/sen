import { TileMap } from "../TileMap";
import styles from "./index.module.scss";
import { useDraggable } from "@dnd-kit/core";

type Props = {
  isInsert: boolean;
  rom: File;
  x: number;
  y: number;
};
export const Cartridge: React.FC<Props> = ({ isInsert, rom, x, y }) => {
  const { isDragging, attributes, listeners, setNodeRef, transform } =
    useDraggable({
      id: rom.name,
      data: { type: "rom", file: rom },
    });

  const finalX = `${x + (transform?.x ?? 0)}`;
  const finalY = `${y + (transform?.y ?? 0)}`;
  const style = {
    top: `${finalY}px`,
    left: `${finalX}px`,
    cursor: isDragging ? "grabbing" : "grab",
  };

  return (
    <div
      ref={setNodeRef}
      className={styles.wrapper}
      style={style}
      hidden={isInsert}
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
