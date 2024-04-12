import { Button } from "@mui/material";
import { useNavigate, useParams } from "react-router-dom";

import { MapWithMarker, TreeDetails, TreeMarkers, ImagePicker } from "@/components";
import { routes } from "@/utils/routes";
import { useTreeDetails } from "./hooks";

import "./styles.scss";

interface IProps {
  id: string;
}

export const DetailsPage = (props: IProps) => {
  const navigate = useNavigate();
  const { tree, loading, error } = useTreeDetails(props.id);

  const handleEdit = () => {
    navigate(routes.editTree(props.id));
  };

  const handleBack = () => {
    navigate(routes.home());
  };

  const handleImageUpload = (files: FileList) => {
    console.debug(`Uploading ${files.length} images...`);
  };

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

          <div className="buttons">
            <ImagePicker onChange={handleImageUpload} />
          </div>

          <MapWithMarker center={{
            lat: tree.lat,
            lon: tree.lon,
          }}>
            <TreeMarkers />
          </MapWithMarker>

          <div className="buttons">
            <Button variant="contained" color="success" onClick={handleEdit}>Edit this tree</Button>
            <Button color="secondary" onClick={handleBack}>Back to map</Button>
          </div>
        </>
      )}

      {!tree && !error && !loading && (
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
      id={id}
    />
  );
};
