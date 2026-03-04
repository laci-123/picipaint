import MenuItem from "./menuitem.tsx";

export default function Menubar() {
  return <div className="menubar">
          <MenuItem caption="egy"/>
          <MenuItem caption="kettő">csipkebokor vessző</MenuItem>
          <MenuItem caption="három"/>
          <MenuItem caption="négy"/>
          <MenuItem caption="öt">
            <div>huszonöt</div>
            <div>százhuszonöt</div>
          </MenuItem>
         </div>;
}
