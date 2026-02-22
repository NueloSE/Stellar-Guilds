import React from "react";
import { Post } from "@/features/social/types";

interface PostItemProps {
  post: Post;
}

export const PostItem: React.FC<PostItemProps> = ({ post }) => {
  return (
    <div className="space-y-1 p-3 border rounded-lg">
      <div className="text-sm font-medium text-gray-800">{post.author}</div>
      <div className="text-gray-700">{post.content}</div>
      <div className="text-xs text-gray-400">{post.createdAt}</div>
    </div>
  );
};