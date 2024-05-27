/**
 * Show an image gallery for a tree.
 *
 * Reads the images from the API.
 */

// Project imports.
import { Gallery } from "@/components";
import { locale } from "@/locale";

// Local imports.
import { useGalleryLoader } from "./hooks";

interface IProps {
  id: string;
}

export const GalleryLoader = (props: IProps) => {
  const { loading, error, images } = useGalleryLoader(props.id);

  return (
    <div className="GalleryLoader">
      {error && (
        <div className="error">{error}</div>
      )}

      {images && (
        <Gallery images={images} />
      )}

      {!error && !images && loading && (
        <div className="message">{locale.loading()}</div>
      )}
    </div>
  );
};
