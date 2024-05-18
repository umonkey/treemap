// Global imports.
import { useEffect, useState } from "react";

// Project imports.
import { useStore } from "@/store";

// Local imports.
import { IProps } from "./types";

const formatPlaceholder = (count: number) => {
  if (count === 0) {
    return "Search trees...";
  }

  return `Search ${count} trees...`;
};

export const useSearchBar = (props: IProps) => {
  const [text, setText] = useState<string>(props.searchQuery);
  const stats = useStore((state) => state.stats);
  const [placeholder, setPlaceholder] = useState<string>(formatPlaceholder(stats.count));

  useEffect(() => {
    setPlaceholder(formatPlaceholder(stats.count));
  }, [stats.count]);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setText(event.target.value);
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      props.onChange(text);
    }
  };

  return {
    placeholder,
    text,
    handleChange,
    handleKeyDown,
  };
};
