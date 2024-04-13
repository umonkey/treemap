import { useNavigate, useParams } from "react-router-dom";

import { LocationPicker, MoveTreeDialog, MapWithMarker, SideBar, WithAuth, WithSidebar } from "@/components";
import { useUserInfo } from "@/utils/userinfo";
import { routes } from "@/utils/routes";
import { useMoveTree } from "./hooks";
import "./styles.scss";

interface IProps {
  id: string;
  token: string;
  onSuccess: () => void;
  onCancel: () => void;
}

export const MoveTreePage = (props: IProps) => {
  const { tree, loading, saving, loadingError, savingError, handlePositionChange, handleSave } = useMoveTree({
    id: props.id,
    token: props.token,
    onSuccess: props.onSuccess,
  });

  const render = () => {
    if (loading) {
      return <p>Loading...</p>;
    }

    if (loadingError) {
      return <p>{loadingError}</p>;
    }

    if (!tree) {
      return <p>Oops, something went wrong.</p>;
    }

    return (
      <WithSidebar>
        <MapWithMarker
          center={{
            lat: tree.lat,
            lon: tree.lon,
          }}
        >
          <LocationPicker onChange={handlePositionChange} />
        </MapWithMarker>

        <SideBar>
          <MoveTreeDialog
            position={{
              lat: tree.lat,
              lon: tree.lon,
            }}
            busy={saving}
            error={savingError}
            onContinue={handleSave}
            onCancel={props.onCancel}
          />
        </SideBar>
      </WithSidebar>
    );
  };

  return (
    <div className="MoveTreePage Page">
      {render()}
    </div>
  );
};

export const MoveTreePageWrapper = () => {
  const { id } = useParams();
  const { userInfo } = useUserInfo();

  const navigate = useNavigate();

  if (!id) {
    console.error("Tree id not set.");
    return null;
  }

  const handleSuccess = () => {
    navigate(routes.treeDetails(id));
  };

  const handleCancel = () => {
    navigate(routes.treeDetails(id));
  };

  return (
    <WithAuth>
      <MoveTreePage
        id={id}
        token={userInfo?.token ?? ""}
        onSuccess={handleSuccess}
        onCancel={handleCancel}
      />
    </WithAuth>
  );
};
