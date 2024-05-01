// Project imports.
import { ITreeInfo } from "@/types";

// Local imports.
import { useExternalTreeLinks } from "./hooks";
import "./styles.scss";

interface IProps {
  tree: ITreeInfo;
}

export const ExternalTreeLinks = (props: IProps) => {
  const { links } = useExternalTreeLinks(props.tree);

  return (
    <ul className="ExternalTreeLinks">
      {links.map((link, index) => (
        <li key={index}>
          <a href={link.href} title={link.tooltip} target="_blank">{link.text}</a>
        </li>
      ))}
    </ul>
  );
};
