/**
 * A simple text input to use for searching trees.
 */

// Global imports.
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass } from "@fortawesome/free-solid-svg-icons";

// Local imports.
import { IProps } from "./types.ts";
import { useSpeciesSearchBar } from "./hooks";
import "./styles.scss";

export const SpeciesSearchBar = (props: IProps) => {
  const {
    handleChange,
    handleInput,
    handleSearchClick,
  } = useSpeciesSearchBar(props);

  return (
    <div className="Control SpeciesSearchBar">
      <input
        type="text"
        placeholder="Search for trees..."
        onChange={handleChange}
        onKeyPress={handleInput}
      />

      <button onClick={handleSearchClick}>
        <FontAwesomeIcon icon={faMagnifyingGlass} />
      </button>
    </div>
  );
}
