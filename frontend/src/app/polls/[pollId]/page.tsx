"use client";
import useUserStore from "@/stores/useUserStore";
import { useEffect, useState } from "react";

export default function PollDetails({ params }: { params: Promise<{ pollId: string }> }) {
  const [pollId, setPollId] = useState<string| null>(null);
  const [post, setPost] = useState<any>(null); // State to hold fetched poll data
  const [loading, setLoading] = useState(true); // Loading state
  const [error, setError] = useState<string | null>(null); // Error state
  const [vote, selectVote] = useState<string | null>(null); // Selected vote option
  const {name} = useUserStore();

  const handleRadioChange = (
    value: string
    ) => {
    selectVote(value);
};


  async function onSubmit(event: React.MouseEvent<HTMLButtonElement>) {
    event.preventDefault();
    if (vote === null) {
      alert("Please select an option before voting.");
      return;
    }
    try {   
      const response = await fetch(
        `http://localhost:8080/api/polls/${pollId}/vote?option_id=${vote}`,
        {
          method: "POST"
        }
      );
      
      if (response.status !== 200) {
        throw new Error("Failed to submit vote");
      }
      alert("Vote Casted Successfullly")
    } catch (err: any) {
      alert(`Error: ${err.message}`);
    }
  }

  useEffect(() => {
    const fetchData = async () => {
      try {
        const resolvedParams = await params;
        setPollId(resolvedParams.pollId);
        const response = await fetch(`http://localhost:8080/api/polls/${resolvedParams.pollId}`);
        if (!response.ok) {
          throw new Error("Failed to fetch poll data");
        }
        const data = await response.json();
        
        
        setPost(data); // Set the fetched data to state
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [params]);

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  if (!post) return <div>No poll data available</div>;
  if (post.status === "closed") return <div>Poll Closed</div>
  if (post.status === "expired ") return <div>Poll Expired</div>
  if (!post.options) return <div>Nil</div>
  const totalVotes = post.options.reduce((sum: number, option: any) => sum + option.votes, 0); // Calculate total votes

  return (
    <div className="w-full h-screen flex flex-col items-center">
      <h1>{name}</h1>
  <div className="w-full max-w-sm p-4 bg-white border border-gray-200 rounded-lg shadow sm:p-6 md:p-8 dark:bg-gray-800 dark:border-gray-700">
    <ul className="space-y-2">
      <li key={post.poll_id}>
        <h2 className="text-xl font-bold">{post.title}</h2>
      </li>
      <li className="text-gray-600">{post.description}</li>
      <li>Status: <span className="font-medium">{post.status}</span></li>
      <li>Created At: {post.created_at}</li>
      <li>Expiration Date: {post.expiration_date}</li>
    </ul>

    <ul className="mt-4 space-y-3">
      {post.options.map((option: any) => {
        const percentage = totalVotes > 0 ? (option.votes / totalVotes) * 100 : 0;

        return (
          <li key={option.option_id} className="flex items-center space-x-3">
            <input
              id={`option-${option.option_id}`}
              type="radio"
              value={option.option_id}
              checked={vote === option.option_id}
              onChange={() => handleRadioChange(option.option_id)}
              className="h-4 w-4 text-blue-600 border-gray-300 focus:ring-blue-500"
            />
            <label
              htmlFor={`option-${option.option_id}`}
              className="text-gray-900 dark:text-gray-200"
            >
              {option.text} - {option.votes} : {Math.round(percentage)}%
            </label>
          </li>
        );
      })}
    </ul>
  </div>

{  name && <button
    className="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
    onClick={onSubmit}

  >
    Submit Vote
  </button>}
</div>

  );
}
