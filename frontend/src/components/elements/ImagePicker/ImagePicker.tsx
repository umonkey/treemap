import { useRef } from "react";
import { Button } from "@mui/material";

import "./styles.scss";

interface IProps {
  onChange: (files: FileList) => void;
  disabled?: boolean;
  children?: React.ReactNode | React.ReactNode[];
}

export const ImagePicker = (props: IProps) => {
  const ref = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      props.onChange(event.target.files);
    }
  };

  const handleClick = () => {
    if (ref.current) {
      ref.current.click();
    }
  };

  return (
    <div className="ImagePicker">
      <Button variant="contained" disabled={!!props.disabled} onClick={handleClick}>{props.children ?? "Add photos"}</Button>
      <input ref={ref} type="file" accept="image/jpeg" multiple onChange={handleFileChange} />
    </div>
  );
};