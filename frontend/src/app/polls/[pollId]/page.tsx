"use client";
import useUserStore from "@/stores/useUserStore";
import { useEffect, useState } from "react";

export interface PollOption {
  option_id: number;
  text: string;
  votes: number;
}

export interface Poll {
  poll_id: number;
  title: string;
  creator: string;
  description: string;
  created_at: string;
  expiration_date?: string | null;
  status: "active" | "expired" | "closed";
  options: PollOption[];
  users_voted: string[];

  [key: string]: string|number|null|string[]| PollOption[]| undefined;
}

export default function PollDetails({ params }: { params: Promise<{ pollId: string }> }) {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || '';
  const [pollId, setPollId] = useState<string | null>(null);
  const [post, setPost] = useState<Poll | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [vote, selectVote] = useState<string | null>(null);
  const [isLive, setIsLive] = useState(false);
  const { name } = useUserStore();

  const handleRadioChange = (value: string) => {
    selectVote(value);
  };

  const fetchData = async () => {
    try {
      const resolvedParams = await params;
      setPollId(resolvedParams.pollId);
      const response = await fetch(`${apiUrl}/api/polls/${resolvedParams.pollId}`);
      if (!response.ok) {
        throw new Error("Failed to fetch poll data");
      }
      const data = await response.json();

      setPost(data);
    } catch (err: unknown) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("An unexpected error occurred");
      }
    } finally {
      setLoading(false);
    }
  };

  async function onSubmit(event: React.MouseEvent<HTMLButtonElement>) {
    event.preventDefault();
    if (vote === null) {
      alert("Please select an option before voting.");
      return;
    }
    try {
      const response = await fetch(
        `${apiUrl}/api/polls/${pollId}/vote?option_id=${vote}&username=${name}`,
        {
          method: "POST",
          body: JSON.stringify({ user: name }),
        }
      );

      if (response.status !== 200) {
        throw new Error("Failed to submit vote");
      }
      alert("Vote Casted Successfully");
      fetchData();
    } catch (err: unknown) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("An unexpected error occurred");
      }
    }
  }

  useEffect(() => {
    

    fetchData();
  }, [params]);

  useEffect(() => {
    if (isLive && pollId) {
      const eventSource = new EventSource(`${apiUrl}/api/polls/${pollId}/results?live=true`);

      eventSource.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            setPost(data)
        } catch (error) {
            console.error("Error parsing data:", error);
        }
    };
    

      eventSource.onerror = () => {
        eventSource.close();
        setIsLive(false);
        alert("Live updates stopped due to an error.");
      };

      return () => {
        eventSource.close();
      };
    }
  }, [isLive, pollId]);

  if (loading) return <div className="text-center">Loading...</div>;
  if (error) return <div className="text-center text-red-500">Error: {error}</div>;

  if (!post) return <div className="text-center">No poll data available</div>;
  if (post.status === "closed") return <div className="text-center">Poll Closed</div>;
  if (post.status === "expired") return <div className="text-center">Poll Expired</div>;
  if (!post.options) return <div className="text-center">Nil</div>;

  const totalVotes = post.options.reduce((sum, option) => sum + option.votes, 0);

  return (
    <div className="w-full flex flex-col items-center mt-16 px-4 sm:px-6 lg:px-8 pt-20">
      <div className="w-full max-w-lg p-6 bg-white border border-gray-200 rounded-lg shadow-md ">
        <ul className="space-y-4">
          <li key={post.poll_id}>
            <h2 className="text-2xl font-bold text-gray-900">{post.title}</h2>
          </li>
          <li className="text-gray-900">{post.description}</li>
          <li>
            Status: <span className="font-medium text-gray-900">{post.status}</span>
          </li>
          <li>Created At: {post.created_at}</li>
          <li>Expiration Date: {post.expiration_date}</li>
        </ul>

        <ul className="mt-6 space-y-4">
          {post.options.map((option) => {
            const percentage = totalVotes > 0 ? (option.votes / totalVotes) * 100 : 0;

            return (
              <li key={option.option_id} className="flex items-center space-x-4">
                <input
                  id={`option-${option.option_id}`}
                  type="radio"
                  value={option.option_id}
               //   checked={vote === option.option_id}
                  onChange={() => handleRadioChange(option.option_id.toString())}
                  className="h-5 w-5 text-blue-600 border-gray-300 focus:ring-2 focus:ring-blue-500"
                />
                <label
                  htmlFor={`option-${option.option_id}`}
                  className="text-gray-900"
                >
                  {option.text} - {option.votes} : {Math.round(percentage)}%
                </label>
              </li>
            );
          })}
        </ul>
      </div>
      <button
        className="mt-6 px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:ring-4 focus:ring-blue-300"
        onClick={onSubmit}
        disabled={post.users_voted.includes(name)}
      >
        {name === "" ? <span>LOGIN TO VOTE</span> : <span>SUBMIT VOTE</span>}
      </button>
      <button
        className="mt-6 px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 focus:ring-4 focus:ring-green-300"
        onClick={() => setIsLive(!isLive)}
      >
        {isLive ? "STOP LIVE" : "GO LIVE"}
      </button>
    </div>
  );
}
