// Global imports.
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCamera } from "@fortawesome/free-solid-svg-icons";

import { locale } from "@/locale";
import { usePhotoToolbarButton } from "./hooks";
import { IProps } from "./types";
import "./styles.scss";

export const PhotoToolbarButton = (props: IProps) => {
  const {
    handleClick,
    handleFileChange,
    ref,
  } = usePhotoToolbarButton(props);

  return (
    <button onClick={handleClick} className="PhotoToolbarButton">
      <div className="icon">
        <FontAwesomeIcon icon={faCamera} />
      </div>
      <div className="label">{locale.photoButton()}</div>
      <input ref={ref} type="file" accept="image/jpeg,text/plain" multiple onChange={handleFileChange} />
    </button>
  );
};
