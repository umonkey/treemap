import React from "react";
import ReactDOM from "react-dom/client";
import { GoogleOAuthProvider } from "@react-oauth/google";

// Project imports.
import { App } from "@/components";
import { initSentry } from "@/utils";
import { getGoogleClientId } from "@/utils/env";

// Local imports
import "./index.css";

initSentry();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <GoogleOAuthProvider clientId={getGoogleClientId()}>
    <React.StrictMode>
      <App />
    </React.StrictMode>
  </GoogleOAuthProvider>,
)
