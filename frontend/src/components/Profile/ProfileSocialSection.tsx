import React from "react";
import { Feed } from "@/features/social/components/Feed";
import { FeedItem } from "@/features/social/types";

interface ProfileSocialSectionProps {
  feedItems: FeedItem[];
  // other props like achievements, follow status etc.
}

export const ProfileSocialSection: React.FC<ProfileSocialSectionProps> = ({
  feedItems,
}) => {
  return (
    <section className="space-y-6">
      {/* this section could combine achievements, activity feed, and follow button */}
      <Feed items={feedItems} />
    </section>
  );
};