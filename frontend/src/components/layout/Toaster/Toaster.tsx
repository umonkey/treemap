import { toast, Toaster as BaseToaster, ToastBar } from "react-hot-toast";

import "./styles.scss";

export const Toaster = () => {
  return (
    <BaseToaster
      position="bottom-right"
      toastOptions={{
        duration: 5000,
      }}
      containerClassName="toaster"
    >
      {(t) => (
        <ToastBar toast={t}>
        {({ icon, message }) => (
          <>
            {icon}
            <div onClick={() => { toast.dismiss(t.id) }}>{message}</div>
          </>
        )}
        </ToastBar>
      )}
    </BaseToaster>
  );
};
