// Global imports.
import { useRef } from "react";

// Project imports.
import { useFileUploader } from "@/hooks";

// Local imports.
import { IProps } from "./types";

export const usePhotoToolbarButton = (props: IProps) => {
  const ref = useRef<HTMLInputElement>(null);
  const { uploadFiles } = useFileUploader();

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      uploadFiles(props.id, event.target.files);
    }
  };

  const handleClick = () => {
    ref.current && ref.current.click();
  };

  return {
    handleClick,
    handleFileChange,
    ref,
  };
};
