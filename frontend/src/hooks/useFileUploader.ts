/**
 * A hook to upload files and attach to a tree.
 *
 * Uses the background upload queue.
 */

// Global imports.
import { useEffect } from "react";
import { toast } from "react-hot-toast";

// Project imports.
import { mainBus } from "@/bus";

export const useFileUploader = () => {
  // Deliver upload finished notifications.
  useEffect(() => {
    console.debug("Bus set up.");

    const handler = () => {
      toast("Files uploaded successfully.", {
        icon: '📷',
      });
    };

    mainBus.on("upload_finished", handler);
    return () => mainBus.off("upload_finished", handler);
  });

  const uploadFiles = async (tree_id: string, files: FileList) => {
    for (let n = 0; n < files.length; n++) {
      mainBus.emit("upload_image", {
        tree: tree_id,
        file: files[n],
      });
     }
  };

  return {
    uploadFiles,
  }
};
