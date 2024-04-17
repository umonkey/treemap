import { IGalleryImage } from "@/types";

import "./styles.scss";

interface IProps {
  images: IGalleryImage[];
}

export const Gallery = (props: IProps) => {
  return (
    <div className="Gallery">
      <div className="items">
        {props.images.map((image, index) => (
          <div key={index} className="image">
            <img src={image.small} alt="" />
          </div>
        ))}
      </div>
    </div>
  );
};
