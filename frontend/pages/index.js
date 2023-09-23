import Head from 'next/head'
import styles from '@/styles/Home.module.css'
import { Inter } from 'next/font/google'
import { useEffect } from 'react';
import { useRouter } from 'next/router';
import { useAuth } from "../context/authContext";

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  const router = useRouter();
  const { user, loading } = useAuth();

  useEffect(() => {
    console.log("User state:", user);
    if (!loading && !user) {
      router.push('/login');
    }
  }, [user, loading, router]);

  return (
    <>
      <Head>
        <title>Restaurant App</title>
        <meta name="description" content="Restaurant App" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className={`${styles.main} ${inter.className}`}>
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
        ssss
      </main>
    </>
  )
}
