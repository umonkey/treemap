// Global imports.
import { useEffect, useState } from "react";

// Project imports.
import { useStore } from "@/store";
import { locale } from "@/locale";

// Local imports.
import { IProps } from "./types";

export const useSearchBar = (props: IProps) => {
  const [text, setText] = useState<string>(props.searchQuery);
  const stats = useStore((state) => state.stats);
  const [placeholder, setPlaceholder] = useState<string>(locale.searchTreesPlaceholder(stats.count));

  useEffect(() => {
    setPlaceholder(locale.searchTreesPlaceholder(stats.count));
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
