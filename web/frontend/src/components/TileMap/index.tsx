import { useEffect, useRef } from "react";
import { SpriteMap } from "../../lib/spriteMap";

type Props = {
  className?: string;
  width: number;
  height: number;
  tiles: number[];
  rotates: number[];
};
export const TileMap: React.FC<Props> = ({
  className,
  width,
  height,
  tiles,
  rotates,
}) => {
  const spriteMap = useRef<SpriteMap>(null);

  useEffect(() => {
    (async () => {
      if (!spriteMap.current) {
        return;
      }

      for (let i = 0; i < tiles.length; i++) {
        const tile = tiles[i];
        const rotate = rotates[i];
        const x = i % width;
        const y = Math.floor(i / height);

        if (tile === -1) {
          continue;
        }

        await spriteMap.current.draw(tile, x, y, rotate !== -1 ? rotate : 0);
      }
    })();

    return () => {
      spriteMap.current?.clear();
    };
  }, [height, rotates, spriteMap, tiles, width]);

  return (
    <canvas
      className={className}
      width={16 * width}
      height={16 * height}
      ref={(e) => {
        if (!e) {
          return;
        }

        spriteMap.current = new SpriteMap(e, "./room.png");
      }}
    />
  );
};
