import { App } from "./app.tsx"
import { createRoot } from "react-dom/client"



const root = createRoot(document.getElementById("app")!);
root.render(<App />);
