import { Header } from "@/components";

import "./styles.scss";

interface IProps {
  children: React.ReactNode;
}

export const WithHeader = (props: IProps) => {
  return (
    <div className="WithHeader">
      <Header />
      {props.children}
    </div>
  );
};
