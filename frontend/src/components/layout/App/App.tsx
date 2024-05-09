// Global imports.
import { HashRouter } from "react-router-dom";
import "@fontsource/roboto";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

// Project imports.
import { ProgressBar, RouterComponent, Toaster } from "@/components";

// Local imports.
import "./styles.scss";
import { useApp } from "./hooks";

export const App = () => {
  useApp();

  return (
    <HashRouter>
      <Toaster />
      <RouterComponent />
      <ProgressBar />
    </HashRouter>
  )
};
