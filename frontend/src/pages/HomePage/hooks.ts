// Project imports.
import { useMapState } from "@/hooks";

export const useHomePage = () => {
  const { mapState } = useMapState();

  return {
    mapState,
  };
};
