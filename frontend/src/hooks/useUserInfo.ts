/**
 * A hook that retrieves information on a user account.
 *
 * The data is read from the API, later possibly from the store.
 */

import { useState, useEffect } from "react";

import { treeMapService } from "@/services/api";
import { IUserInfo } from "@/types";
import { useStore } from "@/store";

export const useUserInfo = (id: string) => {
  const users = useStore((state) => state.users);
  const addUsers = useStore((state) => state.addUsers);

  const [user, setUser] = useState<IUserInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    const cached = users.get(id);

    if (cached) {
      setUser(cached);
      return;
    }

    (async () => {
      try {
        setLoading(true);
        setError(null);

        const res = await treeMapService.getUser(id);

        setUser(res);
        addUsers([res]);

        console.debug(`User ${id} read from the API.`, res);
      } catch (e) {
        console.error(`Error reading user ${id} from the API: ${e}`);
        setError("Error loading user information.");
      } finally {
        setLoading(false);
      }
    })();
  }, [id, users, addUsers]);

  return {
    user,
    error,
    loading,
  };
};
