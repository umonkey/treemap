/**
 * A hook that retrieves information on a user account.
 *
 * The data is read from the API, later possibly from the store.
 */

import { useState, useEffect } from "react";

import { treeMapService } from "@/services/api";
import { IUserInfo } from "@/types";

export const useUserInfo = (id: string) => {
  const [user, setUser] = useState<IUserInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    setLoading(true);
    setError(null);

    (async () => {
      try {
        const res = await treeMapService.getUser(id);
        setUser(res);
        console.debug(`User ${id} read from the API.`, res);
      } catch (e) {
        console.error(`Error reading user ${id} from the API: ${e}`);
        setError("Error loading user information.");
      } finally {
        setLoading(false);
      }
    })();
  }, [id]);

  return {
    user,
    error,
    loading,
  };
};
