import { MapControl } from "@/components";
import { DEFAULT_MAP_CENTER } from "@/utils/config";

import "./styles.css";

interface IProps {
  onAddTree: () => void;
}

export const Map = (props: IProps) => {
  const handleAddTree = () => {
    console.debug("Showing add tree form...");
    props.onAddTree();
  };

  return (
    <MapControl
      center={DEFAULT_MAP_CENTER}
      onAddTree={handleAddTree}
      picker={false}
    />
  );
};
