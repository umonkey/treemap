// Project imports.
import { AddCommentForm, LoginWithGoogleButton } from "@/components";
import { IComment } from "@/types";
import { formatDate } from "@/utils";
import { useUserInfo } from "@/hooks";

// Local imports.
import { useComments } from "./hooks";
import "./styles.scss";

interface IProps {
  id: string;
}

export const TreeComments = (props: IProps) => {
  const { comments, loading, error, reloadComments, handleLoginFailed, handleLoginSuccess } = useComments(props.id);

  const { userInfo } = useUserInfo();

  const handleCommentSent = () => {
    (async () => {
      console.debug("Comment sent, reloading the list.");
      reloadComments(props.id);
    })();
  };

  return (
    <div className="TreeComments">
      {loading && (
        <div className="message">Loading comments...</div>
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
            <p>No comments for this tree yet, be the first.</p>
          )}
        </>
      )}

      {!userInfo && (
        <>
          <p>You need to log in to add a comment.</p>
          <LoginWithGoogleButton onError={handleLoginFailed} onSuccess={handleLoginSuccess} />
        </>
      )}

      {userInfo && (
        <AddCommentForm id={props.id} onSuccess={handleCommentSent} />
      )}
    </div>
  );
};
