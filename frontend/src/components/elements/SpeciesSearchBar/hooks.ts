// Global imports.
import { useEffect, useState } from "react";

// Local imports.
import { IProps } from "./types.ts";

export const useSpeciesSearchBar = (props: IProps) => {
  const [searchTerm, setSearchTerm] = useState<string>(props.query);

  useEffect(() => {
    setSearchTerm(props.query);
  }, [props.query]);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(event.target.value);
  };

  const handleInput = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      props.onSearch(searchTerm);
    }
  };

  const handleSearchClick = () => {
    props.onSearch(searchTerm);
  };

  return {
    handleChange,
    handleInput,
    handleSearchClick,
  };
}
