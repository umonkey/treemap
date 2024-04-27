import { useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

import { EditTreeDialog, MapWithMarker, SideBar, WithAuth, WithHeader, WithSidebar } from "@/components";
import { ITreeDetails } from "@/types";
import { treeMapService } from "@/services/api";
import { useTreeInfo } from "./hooks";
import { routes } from "@/utils/routes";
import { formatErrorMessage } from "@/utils";
import "./styles.scss";

interface IProps {
  id: string;
  onSuccess: () => void;
  onCancel: () => void;
}

export const EditTreePage = (props: IProps) => {
  const [busy, setBusy] = useState<boolean>(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  const { tree, error, loading } = useTreeInfo(props.id);

  const handleSave = async (tree: ITreeDetails) => {
    try {
      setBusy(true);
      setSaveError(null);
      await treeMapService.updateTree(tree.id, tree);
      props.onSuccess();
    } catch (e) {
      setSaveError(`Error updating tree. ${formatErrorMessage(e)}`);
    } finally {
      setBusy(false);
    }
  };

  const render = () => {
    if (loading) {
      return <p>Loading...</p>;
    }

    if (error) {
      return <p>{error}</p>;
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
        />

        <SideBar>
          <EditTreeDialog
            tree={tree}
            busy={busy}
            error={saveError}
            onSave={handleSave}
            onCancel={props.onCancel}
          />
        </SideBar>
      </WithSidebar>
    );
  };

  return (
    <div className="EditTreePage Page">
      {render()}
    </div>
  );
};

export const EditTreePageWrapper = () => {
  const { id } = useParams();

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
      <WithHeader>
        <EditTreePage
          id={id}
          onSuccess={handleSuccess}
          onCancel={handleCancel}
        />
      </WithHeader>
    </WithAuth>
  );
};
