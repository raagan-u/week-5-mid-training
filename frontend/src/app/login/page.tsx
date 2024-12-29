"use client";

import { useState } from "react";
import { startAuthentication } from '@simplewebauthn/browser';
import { redirect } from "next/navigation";
import useUserStore from "../../stores/useUserStore";
import usePollStore from "../../stores/usePollStore";

export default function Register() {
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || '';
  const [name, setName] = useState('');
  const [successMessage, setSuccessMessage] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const userStore = useUserStore();
  const { polls } = usePollStore();

  const handleAuthentication = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSuccessMessage('');
    setErrorMessage('');

    try {
      // Step 1: Fetch authentication options from the server
      const response = await fetch(`${apiUrl}/api/auth/start_auth/` + name, { method: 'POST' });
      const respJSON = await response.json();
      const optionsJSON = respJSON.publicKey;
      console.log("OptionsJSON\n", optionsJSON);

      // Step 2: Start authentication with the options from the server
      const authResponse = await startAuthentication({ optionsJSON });
      console.log("auth Response\n", authResponse);

      // Step 3: Send the authenticator response to the server for verification
      const verificationResponse = await fetch(`${apiUrl}/api/auth/finish_auth/`+name, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(authResponse),
      });

      
      const verificationResult = await verificationResponse.json();

      if (verificationResult.token) {
        setSuccessMessage("Authentication successful!");
        userStore.setUser(name);
        userStore.setOwnedPolls(polls)
      } else {
        setErrorMessage(`Authentication failed. Details: ${JSON.stringify(verificationResult)}`);
      }
    } catch (error) {
      if (error instanceof Error) {
        setErrorMessage(`Error: ${error.message}`);
      }
    }
  };

  return (
    <div className="flex h-screen justify-center items-center bg-black">
      <main className="flex flex-col w-full max-w-md bg-white p-6 rounded-lg shadow-lg">
        <h1 className="text-2xl font-bold text-center text-black mb-4">LOGIN</h1>
        <form onSubmit={handleAuthentication} className="space-y-4">
          <div>
            <label htmlFor="name" className="block text-sm font-medium text-black">
              User Name
            </label>
            <input
              id="name"
              name="name"
              type="text"
              required
              className="mt-2 block w-full p-2 border rounded-md text-black focus:outline-none focus:ring-2 focus:ring-black"
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <button
            type="submit"
            className="w-full py-2 bg-black text-white rounded-md font-medium shadow hover:bg-gray-800 focus:ring-2 focus:ring-yellow-500"
          >
            Login
          </button>
          <a href="/register">New User? Register</a>
          {successMessage && (
            <p className="mt-4 text-center text-green-600">
              {successMessage && redirect("/")}
            </p>
          )}
          {errorMessage && (
            <p className="mt-4 text-center text-red-600">
              {errorMessage}
            </p>
          )}
        </form>
      </main>
    </div>
  );
}
