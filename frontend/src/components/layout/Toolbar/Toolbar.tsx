import "./styles.scss";

interface IProps {
  className: string;
  children: React.ReactNode[];
}

export const Toolbar = (props: IProps) => {
  return (
    <div className={`Toolbar ${props.className}`}>
      {props.children}
    </div>
  );
};
