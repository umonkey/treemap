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
  } = useDetailsPage(props.id);

  return (
    <NarrowPage className="DetailsPage">
      {loading && !tree && (
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

          <TreeComments id={tree.id} />
        </>
      )}

      {!tree && !error && !loading && (
        <p>Oops, something went wrong.</p>
      )}
    </NarrowPage>
  );
};
