import { HashRouter } from "react-router-dom";
import { RouterComponent } from "@/components";

import "./App.scss";
import "@fontsource/roboto";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

function App() {
  return (
    <HashRouter>
      <RouterComponent />
    </HashRouter>
  )
}

export default App
