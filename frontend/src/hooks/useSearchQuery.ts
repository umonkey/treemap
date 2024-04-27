import { STORAGE_SEARCH_QUERY_KEY } from "@/utils/config";

const readValue = (): string => {
  try {
    return localStorage.getItem(STORAGE_SEARCH_QUERY_KEY) || "";
  } catch (e) {
    console.error("Error reading search query from local storage.", e);
    return "";
  }
};

const writeValue = (value: string) => {
  try {
    localStorage.setItem(STORAGE_SEARCH_QUERY_KEY, value);
  } catch (e) {
    console.error("Error writing search query from local storage.", e);
  }
};

export const useSearchQuery = () => {
  const searchQuery = readValue();

  const setSearchQuery = (value: string) => {
    writeValue(value);
  };

  return {
    searchQuery,
    setSearchQuery,
  };
};
