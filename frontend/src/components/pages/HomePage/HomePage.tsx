import { useState } from "react";

import { Map, SideBar } from "@/components";
import { SideBarMode } from "@/types";

import "./styles.css";

export const HomePage = () => {
  const [sideBarMode, setSideBarMode] = useState<SideBarMode>(SideBarMode.DEFAULT);

  const handleAddTree = () => {
    setSideBarMode((prev) => prev === SideBarMode.ADD_TREE ? SideBarMode.DEFAULT : SideBarMode.ADD_TREE);
  };

  return (
    <div className="HomePage">
      <Map
        onAddTree={handleAddTree}
      />

      {sideBarMode === SideBarMode.ADD_TREE && (
        <SideBar>
          <p>Hello, world?</p>
        </SideBar>
      )}
    </div>
  );
};
