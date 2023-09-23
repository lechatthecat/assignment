import '@/styles/globals.css'
import { AuthProvider } from "../context/authContext";

export default function App({ Component, pageProps }) {
  <AuthProvider>
    <Component {...pageProps} />
  </AuthProvider>
}
