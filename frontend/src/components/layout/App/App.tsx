// Global imports.
import { Toaster } from "react-hot-toast";
import { HashRouter } from "react-router-dom";
import "@fontsource/roboto";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

// Project imports.
import { RouterComponent } from "@/components";

// Local imports.
import "./styles.scss";
import { useApp } from "./hooks";

export const App = () => {
  useApp();

  return (
    <HashRouter>
      <Toaster position="bottom-center" />
      <RouterComponent />
    </HashRouter>
  )
};
