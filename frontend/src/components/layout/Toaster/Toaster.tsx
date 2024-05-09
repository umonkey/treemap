import { Toaster as BaseToaster } from "react-hot-toast";

import "./styles.scss";

export const Toaster = () => {
  return (
    <BaseToaster
      position="bottom-right"
      toastOptions={{
        duration: 5000,
      }}
      containerClassName="toaster"
    />
  );
};
