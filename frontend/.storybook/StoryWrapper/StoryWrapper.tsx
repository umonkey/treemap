import { GoogleOAuthProvider } from "@react-oauth/google";
import { getGoogleClientId } from "@/utils/env";

interface IProps {
  children: React.ReactNode | React.ReactNode[];
}

export const StoryWrapper = (props: IProps) => {
  return (
    <GoogleOAuthProvider clientId={getGoogleClientId()}>
      <div className="StoryWrapper">
        {props.children}
      </div>
    </GoogleOAuthProvider>
  );
};
