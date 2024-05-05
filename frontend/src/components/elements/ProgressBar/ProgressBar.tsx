// Local imports.
import { useProgressBar } from "./hooks";
import "./styles.scss";

export const ProgressBar = () => {
  const { progress } = useProgressBar();

  if (progress === 0) {
    return null;
  }

  return (
    <div className="ProgressBar">
      <div className="indicator" style={{
        width: `${progress}%`
      }}></div>
    </div>
  );
};
