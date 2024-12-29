"use client";

import usePollStore from "@/stores/usePollStore";
import useUserStore from "@/stores/useUserStore";
import { useState, useRef } from "react";
import { Poll } from "../[pollId]/page";

export interface PollOption {
  option_id: number;
  text: string;
  votes: number;
}
export default function New() {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || '';
  const [inputs, setInputs] = useState<string[]>(["", ""]); // List of input values
  const formRef = useRef<HTMLFormElement>(null);
  const { name, setOwnedPolls } = useUserStore();
  const { polls, addPoll } = usePollStore();

  // Function to add a new input
  const handleAddInput = () => {
    setInputs([...inputs, ""]); // Add an empty string to the inputs array
  };

  // Function to handle input value changes
  const handleInputChange = (index: number, value: string) => {
    const updatedInputs = [...inputs];
    updatedInputs[index] = value; // Update the value at the specified index
    setInputs(updatedInputs);
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (!formRef.current) return;

    // Create a FormData object from the form
    const formData = new FormData(formRef.current);


    const formDataObject: Poll = {
      poll_id: 0, // Default value
      title: "",
      description: "",
      expiration_date: "",
      created_at: "",
      creator: "",
      options: [],
      users_voted: [],
      status: "active", // Default value, can be changed based on the select
    };
    formData.forEach((value, key) => {
      if (formDataObject.hasOwnProperty(key)) {
        formDataObject[key as keyof Poll] = value.toString(); // Ensure correct type
      }
    });

    const now = new Date();
    const formattedtime = now.toISOString().slice(0, 16);

    formDataObject["options"] = [];
    formDataObject["users_voted"] = [];
    formDataObject["poll_id"] = Number(formDataObject["poll_id"]);
    formDataObject["expiration_date"] = formDataObject["expiration_date"] + ":00Z";
    formDataObject["created_at"] = formattedtime + ":00Z";
    formDataObject["creator"] = name;

    inputs.forEach((value, key) => {
      const data = {
        option_id: key,
        text: value,
        votes: 0,
      };
      formDataObject["options"].push(data);
    });

    const data = JSON.stringify(formDataObject);

    try {
      const response = await fetch(`${apiUrl}/api/polls`, {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: data,
      });

      const result = await response.json();
      if (result.poll_id) {
        alert(result.poll_id + " added successfully");
        addPoll(result);
        setOwnedPolls(polls);
      } else {
        alert("Error in Adding Poll");
      }
    } catch (error) {
      console.error("Error submitting form:", error);
    }
  };

  return (
    <div className="h-full p-20">
      <h1 className="flex-row text-center text-2xl font-bold mb-4">NEW POLL TO BE CREATED</h1>
      <form className="text-xl flex-row max-w-sm mx-auto border rounded-xl  p-2" onSubmit={handleSubmit} ref={formRef}>
        <div className="mb-5">
          <label className="block mb-2  font-medium text-gray-900 dark:text-white">
            POLL ID
          </label>
          <input
            type="number"
            id="poll_id"
            name="poll_id"
            className="bg-gray-50 border border-gray-300 text-gray-900  rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 "
            placeholder="SAMPLE POLL"
            required
          />
        </div>
        <div className="mb-5">
          <label className="block mb-2  font-medium text-gray-900">
            TITLE
          </label>
          <input
            type="text"
            id="title"
            name="title"
            className="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 "
            placeholder="SAMPLE POLL"
            required
          />
        </div>
        <div className="mb-5">
          <label className="block mb-2  font-medium text-gray-900 ">
            DESCRIPTION
          </label>
          <input
            type="text"
            id="description"
            name="description"
            className="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 "
            required
          />
        </div>
        <div className="mb-5">
          <label className="block mb-2 font-medium text-gray-900 ">
            EXPIRY:
          </label>
          <input
            type="datetime-local"
            id="expiration_date"
            name="expiration_date"
            className="bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 "
            required
          />
        </div>
        <div className="mb-5">
          {inputs.map((value, index) => (
            <div key={index} className="mb-2">
              <input
                type="text"
                value={value}
                onChange={(e) => handleInputChange(index, e.target.value)}
                placeholder={`Option ${index + 1}`}
                className="bg-gray-50 border border-gray-300 text-gray-900  rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 "
                required
              />
            </div>
          ))}
          <button
            type="button"
            onClick={handleAddInput}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            +
          </button>
        </div>
        <div className="mb-5">
          <label className="block mb-2 font-medium text-gray-900 ">
            STATUS:
          </label>
          <select name="status">
            <option value="active">ACTIVE</option>
            <option value="closed">CLOSED</option>
          </select>
        </div>
        <button
          type="submit"
          className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center "
        >
          CREATE
        </button>
      </form>
    </div>
  );
}
