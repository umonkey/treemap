import { useEffect, useState } from "react";

import { treeMapService } from "@/services/api";

interface IProps {
  tree_id: string;
  onSuccess: () => void;
}

export const useCommentForm = (props: IProps) => {
  const [text, setText] = useState("");
  const [sending, setSending] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  // Reset form on tree change.
  useEffect(() => {
    setText("");
  }, [props.tree_id]);

  const handleTextChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setText(event.target.value);
  };

  const handleSendComment = () => {
    console.debug("Going to send a comment.");

    (async () => {
      setSending(true);
      setError(null);

      try {
        await treeMapService.addComment(props.tree_id, text);
        setText("");
        props.onSuccess();
      } catch (e) {
        console.error("Error sending comment:", e);
        setError("Error sending comment, please try again later.");
      } finally {
        setSending(false);
      }
    })();
  };

  return {
    text,
    sending,
    error,
    canSend: !sending && text.length > 0,
    handleTextChange,
    handleSendComment,
  };
};
