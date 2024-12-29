"use client";
import usePollStore from "@/stores/usePollStore";
import { redirect } from "next/navigation";
import { useState, useEffect } from "react";

export interface PollOption {
  option_id: number;
  text: string;
  votes: number;
}


export default function Home() {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || '';
  const [fil, setFil] = useState("active"); // Filter state
  const [loading, setLoading] = useState(true); // Loading state
  const [error, setError] = useState<string | null>(null); // Error state
  const {polls, initPolls} = usePollStore();
  // Fetch data on mount
  useEffect(() => {
    const fetchData = async () => {
      try {

        const data = await fetch(`${apiUrl}/api/polls/0`);
        const resp = await data.json();
        console.log(resp);
        initPolls(resp);
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

    fetchData();
  }, []);


  // Handle loading and error states
  if (loading) return <div>Loading... wait</div>;
  if (error) return <div>Error: {error}</div>;
  

  return (
    <div className="flex h-screen justify-center items-center">
      <main className="flex h-4/5 fixed p-4 mt-16  w-screen pt-16">
        {/* Filter Dropdown */}
        <div className="fixed w-full">
        <select
          value={fil}
          onChange={(e) => setFil(e.target.value)}
          className="p-4 ml-10 text-2xl font-extrabold text-black bg-transparent"
        >
          <option value="active">ACTIVE</option>
          <option value="closed">CLOSED</option>
          <option value="expired">EXPIRED</option>
        </select>
        </div>

        {/* Poll List */}
        <ul className="flex flex-wrap justify-center items-center w-full gap-4 p-20 text-xl">
          {polls
            .filter((poll) => poll.status === fil)
            .map((post) => (
              <div
                key={post.poll_id}
                className="w-full max-w-sm p-4  text-black bg-white border border-gray-950 rounded-lg shadow sm:p-6 md:p-8 transition duration-300 hover:bg-gray-300"
                onClick={() => {
                  redirect("/polls/"+post.poll_id)
                }}  
              >
                <ul>
                  <li className="text-lg font-semibold">{post.poll_id}.<span className="text-3xl">{post.title}</span></li>
                  <li>{post.description}</li>
                  <li>Status: {post.status}</li>
                  <li>Created At: {post.created_at}</li>
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
                  <li>Created By: <span className="font-semibold">{post.creator}</span></li>
                </ul>
              </div>
            ))}
        </ul>
      </main>
    </div>
  );
}
