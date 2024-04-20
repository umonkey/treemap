import { Button, ButtonGroup } from "@mui/material";
import { useNavigate, useParams } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHouse, faPencil, faUpDownLeftRight, faCamera, faShareNodes } from "@fortawesome/free-solid-svg-icons";

import { Gallery, MapWithMarker, MoveTreeButton, TreeComments, TreeDetails, TreeMarkers, ImagePicker } from "@/components";
import { routes } from "@/utils/routes";
import { useFileUploader, useDeviceType } from "@/hooks";
import { treeMapService } from "@/services/api";
import { useTreeDetails } from "./hooks";

import "./styles.scss";

interface IProps {
  id: string;
}

export const DetailsPage = (props: IProps) => {
  const navigate = useNavigate();
  const { isPhone, isDesktop } = useDeviceType();
  const { tree, loading, error, canShare, handleShare } = useTreeDetails(props.id);
  const { uploadFiles, error: uploadError, uploading, uploadFinished } = useFileUploader();

  const handleBack = () => {
    navigate(routes.home());
  };

  const handleEdit = () => {
    navigate(routes.editTree(props.id));
  };

  const handleMove = () => {
    navigate(routes.moveTree(props.id));
  };

  const handleImageUpload = (files: FileList) => {
    uploadFiles(props.id, files);
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

          {isPhone && (
            <div className="toolbar">
              <ButtonGroup variant="contained">
                <Button onClick={handleBack}>
                  <FontAwesomeIcon icon={faHouse} />
                </Button>
                <Button onClick={handleEdit}>
                  <FontAwesomeIcon icon={faPencil} />
                </Button>
                <Button onClick={handleMove}>
                  <FontAwesomeIcon icon={faUpDownLeftRight} />
                </Button>
                <ImagePicker disabled={uploading} onChange={handleImageUpload}>
                  <FontAwesomeIcon icon={faCamera} />
                </ImagePicker>
                <Button disabled={!canShare} onClick={handleShare}>
                  <FontAwesomeIcon icon={faShareNodes} />
                </Button>
              </ButtonGroup>
            </div>
          )}

          <MapWithMarker center={{
            lat: tree.lat,
            lon: tree.lon,
          }}>
            <TreeMarkers />
          </MapWithMarker>

          {tree.files && tree.files.length > 0 && (
            <Gallery images={tree.files.map(file => ({
              small: treeMapService.getFileURL(file.small_id),
              large: treeMapService.getFileURL(file.large_id),
            }))} />
          )}

          {uploadError && (
            <div className="message">{uploadError}</div>
          )}

          {uploading && (
            <div className="message">Uploading files, please wait...</div>
          )}

          {uploadFinished && (
            <div className="message">Upload OK, the photos will show up soon.</div>
          )}

          {isDesktop && (
            <ButtonGroup variant="contained">
              <Button variant="contained" color="success" onClick={handleEdit}>Edit this tree</Button>
              <MoveTreeButton id={tree.id} />
              <ImagePicker disabled={uploading} onChange={handleImageUpload}>Add photos</ImagePicker>
              <Button onClick={handleBack}>Back to map</Button>
            </ButtonGroup>
          )}

          <TreeComments id={tree.id} />
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
