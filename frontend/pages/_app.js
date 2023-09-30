import '@/styles/globals.css';
import { AuthProvider, useAuth } from "../context/authContext";

export default function App({ Component, pageProps }) {
  return (
    <AuthProvider>
      <Component {...pageProps} />
    </AuthProvider>
  );
}