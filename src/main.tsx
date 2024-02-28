import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import LayoutNotifications from "./layout/Notifications";

ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
).render(
  <React.StrictMode>
    <LayoutNotifications>
      <App />
    </LayoutNotifications>
  </React.StrictMode>
);
