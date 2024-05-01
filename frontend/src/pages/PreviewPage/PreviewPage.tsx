import {
  DefaultMarker,
  MapControl,
  SideBar,
  TreeMarkers,
  TreeSidePane,
  WithHeader,
  WithSidebar,
} from "@/components";

import { usePreviewPage } from "./hooks";
import "./styles.scss";

interface IProps {
  id: string;
}

export const PreviewPage = (props: IProps) => {
  const { tree, mapState } = usePreviewPage(props.id);

  return (
    <div className="PreviewPage">
      <WithHeader>
        {tree && (
          <WithSidebar>
            <MapControl
              center={{
                lat: tree.lat,
                lon: tree.lon,
              }}
              zoom={mapState.zoom}
            >
              <TreeMarkers />
              <DefaultMarker center={{
                lat: tree.lat,
                lon: tree.lon,
              }} />
            </MapControl>

            <SideBar>
              <TreeSidePane id={tree.id} />
            </SideBar>
          </WithSidebar>
        )}
      </WithHeader>
    </div>
  );
};
