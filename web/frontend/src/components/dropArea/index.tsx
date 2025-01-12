import { PropsWithChildren, useEffect, useRef, useState } from "react";
import styles from "./index.module.scss";

type Props = {
  onAddRom: (file: File) => void;
};
export const DropArea: React.FC<PropsWithChildren<Props>> = ({
  children,
  onAddRom,
}) => {
  const dragCounter = useRef(0);
  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    const handleDrop = (event: DragEvent) => {
      if (!isFileDrag(event)) {
        return;
      }

      event.preventDefault();
      dragCounter.current = 0;
      setIsDragging(false);

      if (event.dataTransfer?.files) {
        const files = Array.from(event.dataTransfer.files);
        files.forEach(onAddRom);
      }
    };

    const handleDragEnter = (event: DragEvent) => {
      if (!isFileDrag(event)) {
        return;
      }

      event.preventDefault();
      dragCounter.current += 1;
      setIsDragging(true);
    };

    const handleDragOver = (event: DragEvent) => {
      event.preventDefault();
    };

    const handleDragLeave = (event: DragEvent) => {
      if (!isFileDrag(event)) {
        return;
      }

      event.preventDefault();
      dragCounter.current -= 1;

      if (dragCounter.current === 0) {
        setIsDragging(false);
      }
    };

    document.body.addEventListener("dragenter", handleDragEnter);
    document.body.addEventListener("dragover", handleDragOver);
    document.body.addEventListener("dragleave", handleDragLeave);
    document.body.addEventListener("drop", handleDrop);

    return () => {
      document.body.removeEventListener("dragenter", handleDragEnter);
      document.body.removeEventListener("dragover", handleDragOver);
      document.body.removeEventListener("dragleave", handleDragLeave);
      document.body.removeEventListener("drop", handleDrop);
    };
  }, [onAddRom]);

  return (
    <div className={`${styles.dropArea} ${isDragging ? styles.over : ""}`}>
      {children}
    </div>
  );
};

const isFileDrag = (event: DragEvent) => {
  return event.dataTransfer?.types.includes("Files");
};
