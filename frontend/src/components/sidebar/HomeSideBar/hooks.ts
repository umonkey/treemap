// Global imports.
import { useNavigate } from "react-router-dom";

// Project imports.
import { useSearchQuery } from "@/hooks";
import { routes } from "@/utils/routes";
import { mainBus } from "@/bus";

export const useHomeSideBar = () => {
  // Remember search query across sessions.
  const { searchQuery, setSearchQuery } = useSearchQuery();

  const navigate = useNavigate();

  const handleSearch = (query: string) => {
    console.debug(`Searching for: ${query}`);
    mainBus.emit("before_search");
    mainBus.emit("search", query);

    setSearchQuery(query);
    navigate(routes.search(query));
  };

  return {
    handleSearch,
    searchQuery,
  };
}
