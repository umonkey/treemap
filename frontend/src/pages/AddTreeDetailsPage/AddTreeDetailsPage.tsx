import { NarrowPage, AddTreeDialog, MapWithMarker, TreeMarkers } from "@/components";

// Local imports.
import "./styles.scss";

interface IProps {
  lat: number;
  lon: number;
}

export const AddTreeDetailsPage = (props: IProps) => {
  return (
    <NarrowPage className="AddTreeDetailsPage">
      <h1>Adding a new tree</h1>

      <MapWithMarker center={{
        lat: props.lat,
        lon: props.lon,
      }}>
        <TreeMarkers />
      </MapWithMarker>

      <AddTreeDialog center={{
        lat: props.lat,
        lon: props.lon,
      }} />
    </NarrowPage>
  );
};