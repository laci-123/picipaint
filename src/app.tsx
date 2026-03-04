import { MyButton } from "./button.tsx";
import { useState } from "react";


export function App() {
  const [count, setCount] = useState(0);

  function handleClick() {
    setCount(count + 1);
  }
  
  return <div>
           <MyButton value={count} onClick={handleClick} />
           <MyButton value={count} onClick={handleClick} />
         </div>;
}
