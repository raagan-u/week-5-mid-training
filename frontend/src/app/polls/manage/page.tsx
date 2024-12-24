"use client";
import { useState, useEffect } from "react";

export default function Manage() {
  const [fil, setFil] = useState("active"); // Filter state
  const [data, setData] = useState<any[]>([]); // Poll data state
  const [loading, setLoading] = useState(true); // Loading state
  const [error, setError] = useState<string | null>(null); // Error state

  // Fetch data on mount
  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch("http://localhost:8080/api/polls/0");
        if (!response.ok) {
          throw new Error("Failed to fetch data");
        }
        const result = await response.json();
        setData(result); // Update data state
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false); // Stop loading
      }
    };

    fetchData();
  }, []);

  // Handle loading and error states
  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div className="flex-row">
      {/* Main Content */}
      <main className="pt-16">
        {/* Filter Dropdown */}
        <select
          value={fil}
          onChange={(e) => setFil(e.target.value)}
          className="p-4 bg-gray-100 dark:bg-gray-800"
        >
          <option value="active">ACTIVE</option>
          <option value="closed">CLOSED</option>
          <option value="expired">EXPIRED</option>
        </select>

        {/* Poll List */}
        <ul className="flex flex-wrap p-16 gap-4">
          {data
            .filter((poll) => poll.status === fil)
            .map((post) => (   
              <div
                key={post.poll_id}
                className="w-full max-w-sm p-4 bg-white border border-gray-200 rounded-lg shadow sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700"
              >
                <ul>
                  <li className="text-lg font-semibold">{post.title}</li>
                  <li>{post.description}</li>
                  <li>Status: {post.status}</li>
                  <li>Created At: {post.created_at}</li>
                  <li>Expiration Date: {post.expiration_date}</li>
                </ul>

                {/* Options List */}
                <ul className="mt-4 space-y-2">
                  {post.options.map((option: any) => {
                    const totalVotes = post.options.reduce(
                      (sum: number, opt: any) => sum + opt.votes,
                      0
                    );
                    const percentage =
                      totalVotes > 0
                        ? (option.votes / totalVotes) * 100
                        : 0;

                    return (
                      <li key={option.option_id}>
                        <div className="mb-2 text-gray-900 dark:text-gray-200">
                          {option.text} - Votes: {option.votes}
                        </div>
                        <div className="relative pt-1">
                          <div className="flex items-center justify-between mb-2">
                            <span className="text-sm font-semibold">
                              {Math.round(percentage)}%
                            </span>
                          </div>
                          <div className="w-full h-2.5 bg-gray-200 rounded-full dark:bg-gray-700">
                            <div
                              className="h-full bg-green-500 rounded-full"
                              style={{ width: `${percentage}%` }}
                            ></div>
                          </div>
                        </div>
                      </li>
                    );
                  })}
                </ul>
              </div>
            ))}
        </ul>
      </main>
    </div>
  );
}
