import { useState } from "react";
import { LoginWithGoogleButton } from "@/components";
import { useLoginInfo } from "@/hooks";
import "./style.css";

interface IProps {
  children: React.ReactNode | React.ReactNode[];
}

export const WithAuth = (props: IProps) => {
  const [success, setSuccess] = useState(false);
  const [error, setError] = useState<boolean>(false);
  const { loginInfo } = useLoginInfo();

  const handleSuccess = () => {
    setSuccess(true);
  };

  const handleError = () => {
    setError(true);
  };

  if (loginInfo || success) {
    return props.children;
  }

  return (
    <div className="LoginPage">
      <div>
        <p>You need to log in to continue.</p>

        <LoginWithGoogleButton
          onSuccess={handleSuccess}
          onError={handleError}
        />

        {error && <p className="error">Login failed, please try again.</p>}
      </div>
    </div>
  );
};
