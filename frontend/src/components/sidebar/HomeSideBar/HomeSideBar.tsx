// Project imports.
import {
  CheckBox,
  SpeciesSearchBar,
} from "@/components";

// Local imports.
import { useHomeSideBar } from "./hooks";
import "./styles.scss";

export const HomeSideBar = () => {
  const {
    handleMyChange,
    handleSearch,
    searchQuery,
  } = useHomeSideBar();

  return (
    <div className="HomeSideBar">
      <SpeciesSearchBar
        query={searchQuery}
        onSearch={handleSearch}
      />

      <CheckBox
        value={false}
        label="Only show my trees"
        onChange={handleMyChange}
      />
    </div>
  );
}
