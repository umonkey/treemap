// Project imports.
import { AddCommentForm, LoginWithGoogleButton } from "@/components";
import { IComment } from "@/types";
import { formatDate } from "@/utils";
import { useLoginInfo } from "@/hooks";
import { locale } from "@/locale";

// Local imports.
import { useComments } from "./hooks";
import "./styles.scss";

interface IProps {
  id: string;
}

export const TreeComments = (props: IProps) => {
  const { comments, loading, error, reloadComments, handleLoginFailed, handleLoginSuccess } = useComments(props.id);

  const { loginInfo } = useLoginInfo();

  const handleCommentSent = () => {
    (async () => {
      reloadComments(props.id);
    })();
  };

  return (
    <div className="TreeComments">
      {loading && (
        <div className="message">{locale.loadingComments()}</div>
      )}

      {error && (
        <div className="message">{error}</div>
      )}

      {!loading && !error && (
        <>
          {comments.map((comment: IComment) => (
            <div key={comment.id} className="comment">
              <div className="date">{formatDate(comment.added_at)}</div>
              <div className="text">{comment.message}</div>
            </div>
          ))}

          {comments.length === 0 && (
            <p>{locale.noComments()}</p>
          )}
        </>
      )}

      {!loginInfo && (
        <>
          <p>{locale.logInToComment()}</p>
          <LoginWithGoogleButton onError={handleLoginFailed} onSuccess={handleLoginSuccess} />
        </>
      )}

      {loginInfo && (
        <AddCommentForm id={props.id} onSuccess={handleCommentSent} />
      )}
    </div>
  );
};
