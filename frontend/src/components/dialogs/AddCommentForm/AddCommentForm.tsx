import { Button, TextField } from "@mui/material";

import { useCommentForm } from "./hooks";

interface IProps {
  id: string;
  onSuccess: () => void;
}

export const AddCommentForm = (props: IProps) => {
  const { text, sending, error, canSend, handleTextChange, handleSendComment } = useCommentForm({
    tree_id: props.id,
    onSuccess: props.onSuccess,
  });

  return (
    <div className="AddCommentForm">
      {sending && (
        <div className="message">Sending comment...</div>
      )}

      {error && (
        <div className="message">{error}</div>
      )}

      <TextField multiline minRows={3} placeholder="Add your comment..." value={text} onChange={handleTextChange} />
      <Button variant="contained" disabled={!canSend} onClick={handleSendComment}>Send comment</Button>
    </div>
  );
};
