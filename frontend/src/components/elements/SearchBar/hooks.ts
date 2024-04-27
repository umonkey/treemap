import { useState } from "react";

interface IProps {
  searchQuery: string;
  onChange: (query: string) => void;
}

export const useSearchBar = (props: IProps) => {
  const [text, setText] = useState<string>(props.searchQuery);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setText(event.target.value);
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      props.onChange(text);
    }
  };

  return {
    text,
    handleChange,
    handleKeyDown,
  };
};
