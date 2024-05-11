// Global imports.
import { toast } from "react-hot-toast";

// Local imports.
import { IProps } from "./types";

export const useShareToolbarButton = (props: IProps) => {
  const handleClick = async () => {
    console.debug(`Sharing tree ${props.id}`);

    if (!navigator.share) {
      toast.error("Your browser does not support sharing.");
      return;
    }

    try {
      await navigator.share({
        title: "Check out this tree on the Tree Map!",
        url: window.location.href,
      });
    } catch (e) {
      console.error("Error sharing tree", e);
      toast.error("Error sharing tree.");
    }
  };

  return {
    handleClick,
  };
};
