import "./styles.scss";

interface IProps {
  children?: React.ReactNode | React.ReactNode[];
}

export const SideBar = (props: IProps) => {
  return (
    <div className="SideBar">
      {props.children}
    </div>
  );
};
