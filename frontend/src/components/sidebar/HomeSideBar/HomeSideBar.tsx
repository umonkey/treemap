// Project imports.
import {
  SpeciesSearchBar,
} from "@/components";

// Local imports.
import { useHomeSideBar } from "./hooks";

export const HomeSideBar = () => {
  const {
    handleSearch,
    searchQuery,
  } = useHomeSideBar();

  return (
    <div className="HomeSideBar">
      <SpeciesSearchBar
        query={searchQuery}
        onSearch={handleSearch}
      />
    </div>
  );
}
