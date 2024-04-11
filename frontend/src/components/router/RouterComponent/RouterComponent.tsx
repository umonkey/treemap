import { Route, Routes } from "react-router-dom";
import { routes } from "@/utils/routes";

import { AddTreePageWrapper, DetailsPageWrapper, HomePage } from "@/components";

export const RouterComponent = () => {
  return (
    <Routes>
      <Route path={routes.home()} element={<HomePage />} />
      <Route path="/add" element={<AddTreePageWrapper />} />
      <Route path={routes.treeDetails(":id")} element={<DetailsPageWrapper />} />
    </Routes>
  );
};
