// Shared types for the social feature

export interface FeedItem {
  id: string;
  userAddress: string;
  content: string;
  timestamp: string;
  type: "achievement" | "activity" | "status" | "announcement";
}

export interface Notification {
  id: string;
  userAddress: string;
  message: string;
  timestamp: string;
  read: boolean;
}

export interface Message {
  id: string;
  from: string;
  to: string;
  body: string;
  timestamp: string;
  encrypted?: boolean; // indicates if the message has been encrypted/decrypted
}

export interface Thread {
  id: string;
  title: string;
  creator: string;
  posts: Post[];
  createdAt: string;
}

export interface Post {
  id: string;
  author: string;
  content: string;
  createdAt: string;
  editedAt?: string;
}

export interface FollowRelationship {
  follower: string;
  following: string;
  since: string;
}

export interface UserPrivacySettings {
  showActivityFeed: boolean;
  allowMessagesFromNonFollowers: boolean;
  showAchievements: boolean;
}
