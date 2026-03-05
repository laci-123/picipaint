import { useEffect, useRef, useState, type RefObject } from "react";

interface Props {
  caption: string;
  children?: React.ReactNode;
}

export default function MenuItem(props: Props) {
  const [bodyVisible, setBodyVisible] = useState(false);
  const ref: RefObject<HTMLDivElement | null> = useRef(null);

  function toggleVisibility() {
    setBodyVisible(!bodyVisible);
  }

  const handleClickOutside = (event: PointerEvent) => {
      if (ref.current && !(event.target instanceof Node && ref.current.contains(event.target))) {
          setBodyVisible(false);
      }
  };

  useEffect(() => {
    document.addEventListener("click", handleClickOutside, true);
    return () => {
      document.removeEventListener("click", handleClickOutside, true);
    };
  }, []);

  return <div className="menu-item" ref={ref}>
           <div className="menu-item-head" onClick={toggleVisibility} title={props.caption}>
             {props.caption}
           </div>
           {
            props.children != undefined && bodyVisible ?
            <div className="menu-item-body">{props.children}</div> :
            null
           }
         </div>;
}
