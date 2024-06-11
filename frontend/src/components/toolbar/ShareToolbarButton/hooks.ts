// Global imports.
import { toast } from "react-hot-toast";

// Project imports.
import { locale } from "@/locale";

// Local imports.
import { IProps } from "./types";

export const useShareToolbarButton = (props: IProps) => {
  const handleClick = async () => {
    console.debug(`Sharing tree ${props.id}`);

    if (!navigator.share) {
      toast.error(locale.sharingNotSupported());
      return;
    }

    try {
      await navigator.share({
        title: locale.shareTitle(),
        url: window.location.href,
      });
    } catch (e) {
      console.error("Error sharing tree", e);
      toast.error(locale.sharingError());
    }
  };

  return {
    handleClick,
  };
};
