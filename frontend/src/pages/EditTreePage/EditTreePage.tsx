import { useState } from "react";

import { EditTreeDialog, MapWithMarker, NarrowPage } from "@/components";
import { ITreeDetails } from "@/types";
import { treeMapService } from "@/services/api";
import { formatErrorMessage } from "@/utils";
import { locale } from "@/locale";

import { useTreeInfo } from "./hooks";
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
      return <p>{locale.loading()}</p>;
    }

    if (error) {
      return <p>{error}</p>;
    }

    if (!tree) {
      return <p>Oops, something went wrong.</p>;
    }

    return (
      <>
        <MapWithMarker
          center={{
            lat: tree.lat,
            lon: tree.lon,
          }}
        />

        <EditTreeDialog
          tree={tree}
          busy={busy}
          error={saveError}
          onSave={handleSave}
          onCancel={props.onCancel}
        />

      </>
    );
  };

  return (
    <NarrowPage className="EditTreePage">
      <h1>{locale.updateTreeDetails()}</h1>
      {render()}
    </NarrowPage>
  );
};
