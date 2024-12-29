"use client";
import { PollOption } from "@/app/page";
import usePollStore from "@/stores/usePollStore";
import useUserStore from "@/stores/useUserStore";
import { useState, useEffect } from "react";

export default function Home() {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || '';
  const [fil, setFil] = useState("active"); // Filter state
  const [loading, setLoading] = useState(true); // Loading state
  const [error, setError] = useState<string | null>(null); // Error state
  const { setOwnedPolls, ownedPolls } = useUserStore();
  const { polls } = usePollStore();

  // Fetch data on mount
  const fetchData = async () => {
    try {
      setOwnedPolls(polls);
    } catch (err: unknown) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("An unexpected error occurred");
      }
    } finally {
      setLoading(false); // Stop loading
    }
  };

  useEffect(() => {
    fetchData();
  }, [polls, setOwnedPolls]);

  // API call to update poll status
  const updatePollStatus = async (pollId: number, action: "close" | "reset") => {
    try {
      const endpoint =
        action === "close"
          ? `${apiUrl}/api/polls/${pollId}/close`
          : `${apiUrl}/api/polls/${pollId}/reset`;
      const response = await fetch(endpoint, {
        method: "POST",
      });

      if (!response.ok) {
        throw new Error(`Failed to ${action} the poll.`);
      } else {
        alert("success")
        fetchData();
      }

    } catch (err: unknown) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("An unexpected error occurred");
      }
    }
  };

  // Handle loading and error states
  if (loading) return <div>Loading... wait</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div className="flex h-screen justify-center items-center">
      <div className="flex h-4/5 fixed p-4 mt-16 w-screen pt-16">
        <div className="fixed w-full">
          <select
            value={fil}
            onChange={(e) => setFil(e.target.value)}
            className="p-4 font-extrabold text-black bg-transparent"
          >
            <option value="active">ACTIVE</option>
            <option value="closed">CLOSED</option>
            <option value="expired">EXPIRED</option>
          </select>
        </div>

        {/* Poll List */}
        <ul className="flex flex-wrap justify-center items-center w-full gap-4 p-20 text-xl">
          {ownedPolls
            .filter((poll) => poll.status === fil)
            .map((post) => (
              <div
                key={post.poll_id}
                className="w-full max-w-sm p-4 text-black bg-white border border-gray-950 rounded-lg shadow sm:p-6 md:p-8"
              >
                <ul>
                  <li className="text-lg font-semibold">{post.title}</li>
                  <li>{post.description}</li>
                  <li>Status: {post.status}</li>
                  <li>Created At: {post.created_at}</li>
                  <li>Created By: {post.creator}</li>
                  <li>Expiration Date: {post.expiration_date}</li>
                </ul>

                {/* Options List */}
                <ul className="mt-4 space-y-2">
                  {post.options.map((option: PollOption) => {
                    const totalVotes = post.options.reduce(
                      (sum: number, opt: PollOption) => sum + opt.votes,
                      0
                    );
                    const percentage =
                      totalVotes > 0
                        ? (option.votes / totalVotes) * 100
                        : 0;

                    return (
                      <li key={option.option_id}>
                        <div className="mb-2">
                          {option.text} - Votes: {option.votes}
                        </div>
                        <div className="relative pt-1">
                          <div className="flex items-center justify-between mb-2">
                            <span className="text-sm font-semibold">
                              {Math.round(percentage)}%
                            </span>
                          </div>
                          <div className="w-full h-2.5 bg-gray-200 rounded-full">
                            <div
                              className="h-full bg-black rounded-full"
                              style={{ width: `${percentage}%` }}
                            ></div>
                          </div>
                        </div>
                      </li>
                    );
                  })}
                </ul>

                {/* Action Buttons */}
                <div className="flex justify-between mt-4">
                  <button
                    onClick={() => updatePollStatus(post.poll_id, "close")}
                    className="px-4 py-2 text-white bg-red-500 rounded hover:bg-red-600"
                  >
                    Close Poll
                  </button>
                  <button
                    onClick={() => updatePollStatus(post.poll_id, "reset")}
                    className="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
                  >
                    Reset Poll
                  </button>
                </div>
              </div>
            ))}
        </ul>
      </div>
    </div>
  );
}
