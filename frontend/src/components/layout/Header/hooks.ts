import { useNavigate } from "react-router-dom";

import { useSearchQuery } from "@/hooks";
import { routes } from "@/utils/routes";

export const useHeader = () => {
  const { searchQuery, setSearchQuery } = useSearchQuery();

  const navigate = useNavigate();

  const handleSearch = (query: string) => {
    console.debug(`Searching for: ${query}`);
    setSearchQuery(query);
    navigate(routes.search(query));
  };

  return {
    searchQuery,
    handleSearch,
  };
};
