import { useParams } from "react-router-dom";

import { MapWithMarker, TreeDetails, TreeMarkers } from "@/components";
import { useTreeDetails } from "./hooks";

import "./styles.scss";

interface IProps {
  id: number;
}

export const DetailsPage = (props: IProps) => {
  const { tree, loading, error } = useTreeDetails(props.id);

  return (
    <div className="DetailsPage Page">
      {loading && (
        <p>Loading...</p>
      )}

      {error && (
        <p>{error}</p>
      )}

      {tree && (
        <>
          <TreeDetails tree={tree} />

          <MapWithMarker center={{
            lat: tree.lat,
            lon: tree.lon,
          }}>
            <TreeMarkers />
          </MapWithMarker>
        </>
      )}

      {!tree && !error && (
        <p>Oops, something went wrong.</p>
      )}
    </div>
  );
};

export const DetailsPageWrapper = () => {
  const { id } = useParams();

  if (!id) {
    console.error("Missing tree id.");
    return null;
  }

  return (
    <DetailsPage
      id={parseInt(id)}
    />
  );
};
