import { Map } from "../../maps/Map";
import { SideBar } from "../../layout/SideBar";

import "./styles.css";

export const HomePage = () => {
  return (
    <div className="HomePage">
      <Map />

      <SideBar>
        <p>Hello, world?</p>
      </SideBar>
    </div>
  );
};
