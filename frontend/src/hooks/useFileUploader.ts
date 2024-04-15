/**
 * A hook to upload files and attach to a tree.
 */

import { useState } from "react";
import { treeMapService } from "@/services/api";

export const useFileUploader = () => {
  const [error, setError] = useState<string | null>(null);

  const uploadFiles = async (tree_id: string, files: FileList) => {
    console.debug(`Uploading ${files.length} files for tree ${tree_id}...`);

    setError(null);

    for (let n = 0; n < files.length; n++) {
      try {
        await treeMapService.uploadImage(tree_id, files[n]);
      } catch (e) {
        console.error("Failed to upload a file.", e);
        setError("Failed to upload a file.");
        return;
      }
    }

    console.debug("Upload complete.");
  };

  return {
    uploadFiles,
    error,
  }
};
