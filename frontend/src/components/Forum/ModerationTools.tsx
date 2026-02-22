import React from "react";

interface ModerationToolsProps {
  onDeletePost?: (postId: string) => void;
  onFlagPost?: (postId: string) => void;
}

export const ModerationTools: React.FC<ModerationToolsProps> = ({
  onDeletePost,
  onFlagPost,
}) => {
  return (
    <div className="flex space-x-2">
      {onFlagPost && (
        <button
          className="text-yellow-500 text-xs"
          onClick={() => onFlagPost!("")}
        >
          Flag
        </button>
      )}
      {onDeletePost && (
        <button
          className="text-red-500 text-xs"
          onClick={() => onDeletePost!("")}
        >
          Delete
        </button>
      )}
    </div>
  );
};