import { useSearchParams, useNavigate } from "react-router-dom";

import { AddTreeDialog, MapWithMarker, SideBar, WithAuth, WithSidebar } from "@/components";
import { ILatLng, ITreeInfo } from "@/types";
import { routes } from "@/utils/routes";

import "./styles.scss";

interface IProps {
  lat: number;
  lon: number;
  onSuccess: (tree: ITreeInfo) => void;
  onCancel: () => void;
}

export const AddTreePage = (props: IProps) => {
  const center = {
    lat: props.lat,
    lon: props.lon,
  } as ILatLng;

  return (
    <div className="AddTreePage">
      <WithSidebar>
        <MapWithMarker
          center={center}
        />

        <SideBar>
          <AddTreeDialog
            center={center}
            onSuccess={props.onSuccess}
            onCancel={props.onCancel}
          />
        </SideBar>
      </WithSidebar>
    </div>
  );
};

export const AddTreePageWrapper = () => {
  const [ params ] = useSearchParams();
  const navigate = useNavigate();

  const lat = params.get("lat");
  const lon = params.get("lon");

  if (!lat || !lon) {
    console.error("Missing lat or lon in URL.");
    return null;
  }

  const handleSuccess = (tree: ITreeInfo) => {
    console.debug("Tree added successfully.", tree);
    navigate(routes.home());
  };

  const handleCancel = () => {
    navigate(routes.home());
  };

  return (
    <WithAuth>
      <AddTreePage
        lat={parseFloat(lat)}
        lon={parseFloat(lon)}
        onSuccess={handleSuccess}
        onCancel={handleCancel}
      />
    </WithAuth>
  );
};
