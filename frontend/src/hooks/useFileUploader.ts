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
        const ticket = await treeMapService.createUploadTicket();
        console.debug(`Upload ticket ${ticket.id} received, url=${ticket.upload_url}.`);
      } catch (e) {
        console.error("Failed to get upload ticket.", e);
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
