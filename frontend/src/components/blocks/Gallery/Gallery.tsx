import LightGallery from "lightgallery/react";
import lgZoom from "lightgallery/plugins/zoom";
import "lightgallery/scss/lightgallery.scss";
import "lightgallery/scss/lg-zoom.scss";

import { IGalleryImage } from "@/types";
import "./styles.scss";

interface IProps {
  images: IGalleryImage[];
}

export const Gallery = (props: IProps) => {
  return (
    <div className="Gallery">
      <LightGallery plugins={[lgZoom]}>
        {props.images.map((image, index) => (
          <a key={index} href={image.large}>
            <img src={image.small} alt="" />
          </a>
        ))}
      </LightGallery>
    </div>
  );
};
