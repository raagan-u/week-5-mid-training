"use client"


import localFont from "next/font/local";
import "./globals.css";
import useUserStore from "@/stores/useUserStore";
import Link from "next/link"; // Import Link from next/link

const geistSans = localFont({
  src: "./fonts/GeistVF.woff",
  variable: "--font-geist-sans",
  weight: "100 900",
});
const geistMono = localFont({
  src: "./fonts/GeistMonoVF.woff",
  variable: "--font-geist-mono",
  weight: "100 900",
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const { name, ClearUser, clearOwnedPolls } = useUserStore();

  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        <header className="flex top-0 fixed w-full h-16 items-center justify-center">
          <div><span className="relative font-bold text-2xl">POLL</span></div>
          <nav className="flex-col text-xl w-1/3 justify-items-end top-0 h-16 text-black">
            <ul className="flex h-10 font-bold p-5 rounded-xl">
              <li className="px-5">
                <Link href="/">Home</Link> {/* Use Link component here */}
              </li>
              {name && (
                <>
                  <li className="px-5">
                    <Link href="/polls/new">Create</Link> {/* Use Link component here */}
                  </li>
                  <li className="px-5">
                    <Link href="/polls/manage">Manage</Link> {/* Use Link component here */}
                  </li>
                  <li className="px-5">
                    <span>({name})</span>
                  </li>
                  <li>
                    <button
                      onClick={() => {
                        clearOwnedPolls(name);
                        ClearUser(name);
                      }}
                      className="w-32 bg-black rounded-xl text-white"
                    >
                      LOGOUT
                    </button>
                  </li>
                </>
              )}
              {!name && (
                <li className="px-5">
                  <button className="w-32 bg-black rounded-xl text-white">
                    <Link href="/login">Login</Link> {/* Use Link component here */}
                  </button>
                </li>
              )}
            </ul>
          </nav>
        </header>
        {children}
      </body>
    </html>
  );
}
