/**
 * A hook to upload files and attach to a tree.
 *
 * Uses the background upload queue.
 */

// Global imports.
import { useState, useEffect } from "react";
import { toast } from "react-hot-toast";

// Project imports.
import { mainBus } from "@/bus";

export const useFileUploader = () => {
  const [uploading, setUploading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [uploadFinished, setUploadFinished] = useState<boolean>(false);

  // Deliver upload finished notifications.
  useEffect(() => {
    const handler = () => {
      toast.success("Files uploaded successfully.", {
        duration: 5000,
      });
    };

    mainBus.on("upload_finished", handler);
    return () => mainBus.off("upload_finished", handler);
  });

  const uploadFiles = async (tree_id: string, files: FileList) => {
    setError(null);
    setUploading(true);
    setUploadFinished(false);

    try {
      for (let n = 0; n < files.length; n++) {
        mainBus.emit("upload_image", {
          tree: tree_id,
          file: files[n],
        });
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
