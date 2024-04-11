import "./styles.scss";

interface IProps {
  children: React.ReactNode | React.ReactNode[];
}

export const WithSidebar = (props: IProps) => {
  return (
    <div className="WithSidebar">
      {props.children}
    </div>
  );
};
