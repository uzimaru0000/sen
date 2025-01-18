import { TileMap } from "../TileMap";
import styles from "./index.module.scss";
import { useDroppable } from "@dnd-kit/core";

type Props = {
  inCartridge: boolean;
  onRelease: () => void;
};
export const NES: React.FC<Props> = ({ inCartridge, onRelease }) => {
  const { isOver, setNodeRef } = useDroppable({ id: "nes" });

  return (
    <div
      ref={setNodeRef}
      className={styles.wrapper}
      style={{ opacity: isOver ? 0.5 : 1 }}
      onClick={onRelease}
    >
      <TileMap
        className={styles.nes}
        width={2}
        height={2}
        tiles={[24, 25, 32, 33]}
        rotates={[-1, -1, -1, -1]}
      />
      {inCartridge && (
        <TileMap
          className={styles.cartridge}
          width={1}
          height={1}
          tiles={[37]}
          rotates={[-1]}
        />
      )}
    </div>
  );
};
