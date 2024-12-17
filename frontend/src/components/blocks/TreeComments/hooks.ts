import { useState, useEffect } from "react";
import { IComment } from "@/types";
import { treeMapService } from "@/services/api";

export const useComments = (tree_id: string) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [comments, setComments] = useState<IComment[]>([]);

  const reloadComments = async (tree_id: string) => {
    console.debug(`Fetching comments for tree ${tree_id} ...`);

    setLoading(true);
    setError(null);

    try {
      const comments = await treeMapService.getComments(tree_id);
      console.debug(`Fetched ${comments.comments.length} comments for tree ${tree_id}.`);

      setComments(comments.comments);
    } catch (e) {
      console.error("Error fetching comments:", e);
      setError("Error loading comments, please try again later.");
    } finally {
      setLoading(false);
    }
  };

  const handleLoginFailed = () => {
    setError("Login failed, please try again.");
  };

  const handleLoginSuccess = () => {
    window.location.reload();
  };

  useEffect(() => {
    (async () => {
      reloadComments(tree_id);
    })();
  }, [tree_id]);

  return {
    comments,
    loading,
    error,
    reloadComments,
    handleLoginFailed,
    handleLoginSuccess,
  };
};
