import { useState } from "react";
import { useSearchParams, useNavigate } from "react-router-dom";

import { AddTreeDialog, MapWithMarker, SideBar, WithAuth, WithSidebar } from "@/components";
import { IAddTreeRequest, ILatLng, ITreeInfo } from "@/types";
import { treeMapService } from "@/services/api";
import { useUserInfo } from "@/utils/userinfo";
import { routes } from "@/utils/routes";

import "./styles.scss";

interface IProps {
  lat: number;
  lon: number;
  token: string;
  onSuccess: (tree: ITreeInfo) => void;
  onCancel: () => void;
}

export const AddTreePage = (props: IProps) => {
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState<boolean>(false);

  const center = {
    lat: props.lat,
    lon: props.lon,
  } as ILatLng;

  const handleSave = async (tree: IAddTreeRequest) => {
    try {
      setBusy(true);
      const res = await treeMapService.addMarker(tree, props.token);
      props.onSuccess(res);
    } catch (e) {
      console.error(`Error adding tree: ${e}`);
      setError("Error adding tree. Please try again later.");
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="AddTreePage">
      <WithSidebar>
        <MapWithMarker
          center={center}
        />

        <SideBar>
          <AddTreeDialog
            center={center}
            onSave={handleSave}
            onCancel={props.onCancel}
            error={error}
            busy={busy}
          />
        </SideBar>
      </WithSidebar>
    </div>
  );
};

export const AddTreePageWrapper = () => {
  const [ params ] = useSearchParams();
  const navigate = useNavigate();

  const { userInfo } = useUserInfo();

  const lat = params.get("lat");
  const lon = params.get("lon");

  if (!lat || !lon) {
    console.error("Missing lat or lon in URL.");
    return null;
  }

  if (!userInfo?.token) {
    console.error("Missing user token.");
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
        token={userInfo.token}
        onSuccess={handleSuccess}
        onCancel={handleCancel}
      />
    </WithAuth>
  );
};
