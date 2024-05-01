// Global imports.
import { useState, useEffect } from "react";

// Project imports.
import { IGalleryImage } from "@/types";
import { treeMapService } from "@/services/api";
import { getFileURL } from "@/utils";

export const useGalleryLoader = (id: string) => {
  const [images, setImages] = useState<IGalleryImage[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
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
    })();
  }, [id]);

  return {
    loading,
    error,
    images,
  };
};
