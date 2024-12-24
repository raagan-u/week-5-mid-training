"use client"
import useUserStore from "@/stores/useUserStore";
import { useState, useRef } from "react";

export default function New() {
  const [inputs, setInputs] = useState<string[]>(["", ""]); // List of input values
  const formRef = useRef<HTMLFormElement>(null);
  const {name} = useUserStore();

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

    // Convert FormData to an object for easier manipulation or debugging
    const formDataObject: Record<string, any> = {};
    
    formData.forEach((value, key) => {
      if (formDataObject[key]) {
        // Handle multiple values (e.g., array inputs like checkboxes)
        formDataObject[key] = [].concat(formDataObject[key], value);
      } else {
        formDataObject[key] = value;
      }
    });
    const now = new Date();
    const formattedtime = now.toISOString().slice(0,16);
    
    formDataObject["options"] = [];
    formDataObject["poll_id"] = Number(formDataObject["poll_id"]);
    formDataObject["expiration_date"] = formDataObject["expiration_date"]+":00Z";
    formDataObject["created_at"] = formattedtime + ":00Z";
    formDataObject["creator"] = name;

    inputs.forEach((value, key) => {
      const data = {
        "option_id": key,
        "text": value,
        "votes": 0
      }
      formDataObject["options"].push(data);
    });

    console.log(formDataObject)
    const data = JSON.stringify(formDataObject);

    
    try {
      console.log(data)
      const response = await fetch("http://localhost:8080/api/polls", {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: data, 
      });

     const result = await response.json();
        if (result.poll_id) {
          alert(result.poll_id + " added successfully")
        } else {
          alert("Error in Adding Poll")
        }
    } catch (error) {
      console.error("Error submitting form:", error);
    }
  }

  return (
    <div className="flex-row ">
      <h1 className="flex-row text-center ">NEW POLL TO BE CREATED</h1>
      <form className=" flex-row max-w-sm mx-auto border- p-2" onSubmit={handleSubmit} ref={formRef}>
      <div className="mb-5">
          <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            POLL ID
          </label>
          <input
            type="number"
            id="title"
            name="poll_id"
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            placeholder="SAMPLE POLL"
            required
          />
        </div>
        <div className="mb-5">
          <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            TITLE
          </label>
          <input
            type="text"
            id="title"
            name="title"
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            placeholder="SAMPLE POLL"
            required
          />
        </div>
        <div className="mb-5">
          <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            DESCRIPTION
          </label>
          <input
            type="text"
            id="desc"
            name="description"
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            required
          />
        </div>
        <div className="mb-5">
          <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            EXPIRY:
          </label>
          <input
            type="datetime-local"
            id="date"
            name="expiration_date"
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            required
          />
        </div>
        <div className="mb-5">
          {/* Render the inputs */}
          {inputs.map((value, index) => (
            <div key={index} className="mb-2">
              <input
                type="text"
                id="desc"
                value={value}
                onChange={(e) => handleInputChange(index, e.target.value)}
                placeholder={`Option ${index + 1}`}
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                required
              />
            </div>
          ))}

          {/* Button to add new input */}
          <button
            type="button"
            onClick={handleAddInput}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            +
          </button>
        </div>
        <div className="mb-5">
          <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
            STATUS:
          </label>
          <select name="status">
            <option value="active"> ACTIVE </option>
            <option value="closed"> CLOSED </option>
          </select>
        </div>
        <button
          type="submit"
          className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
        >
          CREATE
        </button>
      </form>
    </div>
  );
}
