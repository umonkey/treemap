/**
 * A hook to upload files and attach to a tree.
 */

import { useState } from "react";
import { treeMapService } from "@/services/api";
import { formatErrorMessage } from "@/utils";

export const useFileUploader = () => {
  const [uploading, setUploading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [uploadFinished, setUploadFinished] = useState<boolean>(false);

  const uploadFiles = async (tree_id: string, files: FileList) => {
    console.debug(`Uploading ${files.length} files for tree ${tree_id}...`);

    setError(null);
    setUploading(true);
    setUploadFinished(false);

    try {
      for (let n = 0; n < files.length; n++) {
        try {
          await treeMapService.uploadImage(tree_id, files[n]);
        } catch (e) {
          console.error("Failed to upload a file.", e);
          setError("Failed to upload a file. " + formatErrorMessage(e));
          return;
        }
      }

      setUploadFinished(true);

      console.debug("Upload complete.");
    } finally {
      setUploading(false);
    }
  };

  return {
    uploadFiles,
    error,
    uploading,
    uploadFinished,
  }
};
