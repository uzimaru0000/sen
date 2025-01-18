import { ComponentPropsWithRef } from "react";
import styles from "./index.module.scss";
import { TileMap } from "../TileMap";
import tvTiles from "../../assets/tv.json";
import coverTiles from "../../assets/cover.json";

type Props = ComponentPropsWithRef<"canvas"> & {
  onZoom: () => void;
};

export const TV: React.FC<Props> = ({ ref, onZoom }) => {
  return (
    <div className={styles.case} onClick={onZoom}>
      <TileMap
        className={styles.tv}
        width={2}
        height={2}
        tiles={tvTiles.tiles}
        rotates={tvTiles.rotates}
      />
      <TileMap
        className={styles.cover}
        width={2}
        height={2}
        tiles={coverTiles.tiles}
        rotates={coverTiles.rotates}
      />
      <canvas ref={ref} className={styles.canvas} width="256" height="240" />
    </div>
  );
};
