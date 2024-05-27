/**
 * A checkbox that allows one to only see their own trees.
 *
 * Only available after the user logs in.
 */

// Local imports.
import { IProps } from "./types";
import { useCheckBox } from "./hooks";
import "./styles.scss";

export const CheckBox = (props: IProps) => {
  const {
    label,
    tick,
    handleClick,
  } = useCheckBox(props);

  return (
    <div className="Control CheckBox">
      <button onClick={handleClick}>{tick}</button>
      <label onClick={handleClick}>{label}</label>
    </div>
  );
}
