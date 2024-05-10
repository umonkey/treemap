import { GoogleOAuthProvider } from "@react-oauth/google";
import { getGoogleClientId } from "@/utils/env";

// Project imports.
import { useDeviceType } from "@/hooks";
import "@/index.css";

interface IProps {
  children: React.ReactNode | React.ReactNode[];
}

export const StoryWrapper = (props: IProps) => {
  const { className } = useDeviceType();

  return (
    <GoogleOAuthProvider clientId={getGoogleClientId()}>
      <div id="root" className="StoryWrapper" style={{
        height: "100%",
        width: "100%",
      }}>
        {props.children}
      </div>
    </GoogleOAuthProvider>
  );
};
