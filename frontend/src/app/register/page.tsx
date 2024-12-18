"use client"

import { useState } from "react";
import { startRegistration } from '@simplewebauthn/browser';

export default function Register() {
    const [name, setName] = useState('');
    const [successMessage, setSuccessMessage] = useState('');
    const [errorMessage, setErrorMessage] = useState('');

    const handleRegister = async (event: React.FormEvent<HTMLFormElement>) => {
        // Reset messages
        event.preventDefault();
        setSuccessMessage('');
        setErrorMessage('');

        try {
            // Step 1: Get registration options from the server
            const response = await fetch('http://localhost:8080/auth/start_reg/'+name, {
                method: "POST"
            });
            const jsonresp = await response.json();
            const options = jsonresp.publicKey
            
            // Step 2: Call startRegistration with the options received
            const attResp = await startRegistration({optionsJSON: options});

            // Step 3: POST the response to the server for verification
            
            const verificationResponse = await fetch('http://localhost:8080/auth/finish_reg', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(attResp),
            });

            // Wait for the results of verification
            const verificationJSON = await verificationResponse.text();

            // Show UI based on the verification status
            if (verificationJSON) {
                setSuccessMessage('Registration successful!');
            } else {
                setErrorMessage(`Registration failed! Response: ${JSON.stringify(verificationJSON)}`);
            }
        } catch (error) {
            // Handle any errors that occur during the registration process
            if (error instanceof Error) {
            setErrorMessage(`Error:` + error.message);}
        }
    };
    return (
        <div className="flex h-[100vh] items-center  justify-center bg-black">
        <div className="w-full max-w-md p-8 bg-white shadow-lg rounded-lg">
            <h1 className="text-2xl font-semibold text-gray-800 text-center mb-6">
                Login
            </h1>
            <form onSubmit={handleRegister} className="space-y-6">
                <div>
                    <label htmlFor="email" className="block text-sm font-medium text-gray-700">
                        Email address
                    </label>
                    <input
                        id="email"
                        name="email"
                        type="email"
                        required
                        className="mt-2 block w-full rounded-md border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500"
                        onChange={(e) => setName(e.target.value)}
                    />
                </div>
                <div>
                    <label htmlFor="name" className="block text-sm font-medium text-gray-700">
                        User Name
                    </label>
                    <input
                        id="name"
                        name="name"
                        type="text"
                        required
                        className="mt-2 block w-full rounded-md border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500"
                        onChange={(e) => setName(e.target.value)}
                    />
                </div>
                <button
                    type="submit"
                    className="w-full bg-yellow-600 text-white py-2 rounded-md font-medium shadow-md hover:bg-indigo-500 focus:ring-2 focus:ring-indigo-400"
                >
                    Login
                </button>
                {successMessage && <p className="mt-4 text-sm text-green-600">{successMessage}</p>}
                {errorMessage && <p className="mt-4 text-sm text-red-600">{errorMessage}</p>}
            </form>
            <h1>TETS</h1>
        </div>
    </div>
    )
}