interface Props {
  caption: string;
  children?: React.ReactNode;
}

export default function MenuItem(props: Props) {
  return <div className="menu-item">
           <div className="menu-item-head" title={props.caption}>{props.caption}</div>
           {
            props.children != undefined ?
            <div className="menu-item-body">{props.children}</div> :
            null
           }
         </div>;
}
