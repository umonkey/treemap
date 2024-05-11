import {
  DetailsToolbar,
  GalleryLoader,
  MapWithMarker,
  NarrowPage,
  TreeComments,
  TreeDetails,
  TreeMarkers,
} from "@/components";

import { useDetailsPage } from "./hooks";
import "./styles.scss";

interface IProps {
  id: string;
}

export const DetailsPage = (props: IProps) => {
  const {
    error,
    loading,
    tree,
    uploadError,
    uploadFinished,
    uploading,
  } = useDetailsPage(props.id);

  return (
    <NarrowPage className="DetailsPage">
      {loading && (
        <p>Loading...</p>
      )}

      {error && (
        <p>{error}</p>
      )}

      {tree && (
        <>
          <TreeDetails tree={tree} />

          <DetailsToolbar id={tree.id} />

          <MapWithMarker center={{
            lat: tree.lat,
            lon: tree.lon,
          }}>
            <TreeMarkers />
          </MapWithMarker>

          <GalleryLoader id={tree.id} />

          {uploadError && (
            <div className="message">{uploadError}</div>
          )}

          {uploading && (
            <div className="message">Uploading files, please wait...</div>
          )}

          {uploadFinished && (
            <div className="message">Files accepted, you can continue your work while they are being uploaded.</div>
          )}

          <TreeComments id={tree.id} />
        </>
      )}

      {!tree && !error && !loading && (
        <p>Oops, something went wrong.</p>
      )}
    </NarrowPage>
  );
};
