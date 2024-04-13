import { useState } from "react";

import { ILatLng } from "@/types";
import { useTreeDetails } from "@/hooks";
import { formatErrorMessage } from "@/utils";
import { treeMapService } from "@/services/api";

interface IProps {
  id: string;
  token: string;
  onSuccess: () => void;
}

export const useMoveTree = (props: IProps) => {
  const { tree, error: treeLoadingError, loading } = useTreeDetails(props.id);

  const [saving, setSaving] = useState<boolean>(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [position, setPosition] = useState<ILatLng | null>(null);

  const handlePositionChange = (position: ILatLng) => {
    setPosition(position);
  };

  const handleSave = async () => {
    if (!tree) {
      setSaveError("Tree not loaded.");
      return;
    }

    if (!position) {
      setSaveError("Position not set.");
      return;
    }

    try {
      setSaving(true);
      setSaveError(null);
      await treeMapService.updateTreePosition(tree.id, position, props.token);
      props.onSuccess();
    } catch (e) {
      console.log(`Error updating tree: ${e}`);
      setSaveError(`Error updating tree. ${formatErrorMessage(e)}`);
    } finally {
      setSaving(false);
    }
  };

  return {
    tree,
    loading,
    saving,
    loadingError: treeLoadingError,
    savingError: saveError,
    handlePositionChange,
    handleSave,
  };
};
