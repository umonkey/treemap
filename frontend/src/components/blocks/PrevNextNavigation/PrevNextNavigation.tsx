import { Link } from "react-router-dom";

import { IProps } from "./types";

export const PrevNextNavigation = (props: IProps) => {
  return (
    <div className="PrevNextNavigation">
      {props.prev && (
        <Link to={props.prev}>Previous</Link>
      )}

      {props.prev && props.next && (
        <span className="sep"> | </span>
      )}

      {props.next && (
        <Link to={props.next}>Next</Link>
      )}
    </div>
  );
};
