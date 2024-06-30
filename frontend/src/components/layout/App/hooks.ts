// Global imports.
import { useEffect } from "react";

// Project imports.
import { treeMapService, fileUploader } from "@/services";
import { mainBus } from "@/bus";
import { useStore } from "@/store";
import { useDeviceType } from "@/hooks";

// Initialize services.
import { statsService } from "@/services";

export const useApp = () => {
  const loginInfo = useStore((state) => state.loginInfo);
  const setLoginInfo = useStore((state) => state.setLoginInfo);
  const { className } = useDeviceType();

  // Run the background file uploader.
  useEffect(() => {
    fileUploader.run();
    statsService.start();

    mainBus.emit("initialize");

    return () => {
      fileUploader.finish();
      statsService.stop();
    };
  });

  useEffect(() => {
    if (loginInfo === null) {
      treeMapService.setToken(null);
      return;
    }

    (async () => {
      try {
        await treeMapService.getUserInfo();
        console.debug("[app] User token is OK.");
      } catch (e) {
        // @ts-expect-error TS18046
        const status = e.status ?? 500;

        if (status === 401) {
          console.warn("[app] User token expired, logging out.");
          setLoginInfo(null);
        } else {
          console.error("[app] Error checking user token.", e);
        }
      }
    })();
  });

  return {
    className,
  };
};
