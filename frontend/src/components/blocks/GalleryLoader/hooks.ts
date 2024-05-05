// Global imports.
import { useState, useEffect } from "react";

// Project imports.
import { IGalleryImage, IFileReadyEvent } from "@/types";
import { treeMapService } from "@/services/api";
import { getFileURL } from "@/utils";
import { mainBus } from "@/bus";

export const useGalleryLoader = (id: string) => {
  const [images, setImages] = useState<IGalleryImage[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const reloadGallery = async (id: string) => {
    setError(null);
    setLoading(true);

    try {
      const res = await treeMapService.getTreeDetails(id);

      const files = res.files || [];

      setImages(files.map(file => ({
        small: getFileURL(file.small_id),
        large: getFileURL(file.large_id),
      })));

    } catch (e) {
      console.error("Error loading tree info.", e);
      setError("Error loading images, please try again later.");
    } finally {
      setLoading(false);
    }
  };

  // Update on showing different tree.
  useEffect(() => {
    (async () => {
      await reloadGallery(id);
    })();
  }, [id]);

  // Update on file becomes ready.
  useEffect(() => {
    const handler = (event: IFileReadyEvent) => {
      if (event.tree_id === id) {
        console.debug("Reloading image gallery because a new file is ready.");
        reloadGallery(id);
      }
    };

    mainBus.on("upload_ready", handler);
    return () => mainBus.off("upload_ready", handler);
  }, [id]);

  return {
    loading,
    error,
    images,
  };
};
