import { useState } from "react";

import { Map } from "../../maps/Map";
import { SideBar } from "../../layout/SideBar";

import "./styles.css";

export const HomePage = () => {
  const [showAdd, setShowAdd] = useState(false);

  const handleAddTree = () => {
    setShowAdd(!showAdd);
  };

  return (
    <div className="HomePage">
      <Map
        onAddTree={handleAddTree}
      />

      {showAdd && (
        <SideBar>
          <p>Hello, world?</p>
        </SideBar>
      )}
    </div>
  );
};
