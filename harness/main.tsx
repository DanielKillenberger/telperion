import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { GrowerDev } from "./GrowerDev";

/* StrictMode on purpose, and it is not decoration: it double-mounts
   every effect in development, which is exactly the condition that
   once left the stage unframed and the camera 12 cm off the ground.
   Running under it here is what keeps that fixed. */
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <GrowerDev />
  </StrictMode>,
);
