import Menubar from "./components/menubar.tsx"; 


export default function App() {
  
  return <div id="app">
          <Menubar/>
          <canvas className="main-canvas"></canvas>
         </div>;
}
