"use client"

import { useState } from "react";
import { startAuthentication } from '@simplewebauthn/browser';
import { redirect } from "next/navigation";
import useUserStore from "../../stores/useUserStore";

export default function Register() {
    const [name, setName] = useState('');
    const [successMessage, setSuccessMessage] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
   const userStore = useUserStore();

  const handleAuthentication = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSuccessMessage('');
    setErrorMessage('');

    try {
      // Step 1: Fetch authentication options from the server
      const response = await fetch('http://localhost:8080/api/auth/start_auth/'+name, {method: 'POST'});
      const optioonsJSON = await response.json();

        const optionsJSON = optioonsJSON.publicKey; 
        console.log("OptionsJSON\n");
        console.log(optionsJSON)
      // Step 2: Start authentication with the options from the server
      const authResponse = await startAuthentication({ optionsJSON });
      console.log(authResponse)
      console.log("auth Response\n");
      console.log(authResponse)      // Step 3: Send the authenticator response to the server for verification
      const verificationResponse = await fetch('http://localhost:8080/api/auth/finish_auth', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(authResponse),
      });

      const verificationResult = await verificationResponse.json();
      console.log("final op")
      console.log(verificationResult)
      if (verificationResult) {
        setSuccessMessage("Authentication successful!");
        userStore.setUser(name);
      } else {
        setErrorMessage(`Authentication failed. Details: ${JSON.stringify(verificationResult)}`);
      }
    } catch (error) {
        if (error instanceof Error) {
            setErrorMessage(`Error:` + error.message);}
      
    }
  };
    
    return (
        <div className="flex h-[100vh] items-center  justify-center bg-black">
        <div className="w-full max-w-md p-8 bg-white shadow-lg rounded-lg">
            <h1 className="text-2xl font-semibold text-gray-800 text-center mb-6">
                LOGIN
            </h1>
            <form onSubmit={handleAuthentication} className="space-y-6">
                <div>
                    <label htmlFor="name" className="block text-sm font-medium text-gray-700">
                        User Name
                    </label>
                    <input
                        id="name"
                        name="name"
                        type="text"
                        required
                        className="text-black mt-2 block w-full rounded-md  shadow-sm focus:ring-indigo-500 focus:border-indigo-500"
                        onChange={(e) => setName(e.target.value)}
                    />
                </div>
                <button
                    type="submit"
                    className="w-full bg-yellow-600 text-white py-2 rounded-md font-medium shadow-md hover:bg-indigo-500 focus:ring-2 focus:ring-indigo-400"
                >
                    Login
                </button>
                {successMessage && <p className="mt-4 text-sm text-green-600">{successMessage && redirect("/")}</p>}
                {errorMessage && <p className="mt-4 text-sm text-red-600">{errorMessage}</p>}
            </form>
        </div>
    </div>
    )
}