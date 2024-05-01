import { WithHeader } from "@/components";

import "./styles.scss";

interface IProps {
  className: string;
  children: React.ReactNode | React.ReactNode[];
}

export const NarrowPage = (props: IProps) => {
  return (
    <WithHeader>
      <div className={`NarrowPage ${props.className}`}>
        {props.children}
      </div>
    </WithHeader>
  );
};
