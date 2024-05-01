import { Route, Routes } from "react-router-dom";
import { routes } from "@/utils/routes";

import { AddTreePageWrapper, DetailsPageWrapper, EditTreePageWrapper, MoveTreePageWrapper, HomePage } from "@/pages";

export const RouterComponent = () => {
  return (
    <Routes>
      <Route path={routes.home()} element={<HomePage />} />
      <Route path="/add/continue" element={<AddTreePageWrapper />} />
      <Route path={routes.treeDetails(":id")} element={<DetailsPageWrapper />} />
      <Route path={routes.editTree(":id")} element={<EditTreePageWrapper />} />
      <Route path={routes.moveTree(":id")} element={<MoveTreePageWrapper />} />
    </Routes>
  );
};
