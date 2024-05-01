import { Button, ButtonGroup } from "@mui/material";
import { useNavigate } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowLeft, faPencil, faUpDownLeftRight, faCamera, faShareNodes } from "@fortawesome/free-solid-svg-icons";

import { GalleryLoader, MapWithMarker, MoveTreeButton, NarrowPage, TreeComments, TreeDetails, TreeMarkers, ImagePicker } from "@/components";
import { routes } from "@/utils";
import { useFileUploader, useDeviceType } from "@/hooks";

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

          {isPhone && (
            <div className="toolbar">
              <ButtonGroup variant="contained">
                <Button onClick={handleBack}>
                  <FontAwesomeIcon icon={faArrowLeft} />
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

          <GalleryLoader id={tree.id} />

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
    </NarrowPage>
  );
};
